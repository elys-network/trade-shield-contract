use crate::{
    tests::{mock::multitest::ElysApp, read_processed_order_id::read_processed_order_id},
    types::SwapAmountInRoute,
};

use super::*;
use cosmwasm_std::{coins, Coin};

#[test]
fn process_spot_order_processing() {
    // Initialize wallets for "owner" and "user."
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("owner", coins(120, "usdc")), ("user", vec![])];
    let mut app = ElysApp::new_with_wallets(wallets);

    // Define ubtc and USDC prices at two different time intervals (t0 and t1).
    let prices = vec![coin(40, "ubtc"), coin(1, "usdc")];

    // Create a contract wrapper and store its code.
    let code =
        ContractWrapper::new(execute, instantiate, query).with_reply(crate::entry_point::reply);
    let code_id = app.store_code(Box::new(code));

    // Create a "limit buy" order (dummy order) with a specific rate and balance.
    let dummy_order = SpotOrder::new(
        SpotOrderType::LimitBuy,
        SpotOrderPrice {
            base_denom: "ubtc".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Decimal::from_atomics(Uint128::new(38), 0).unwrap(), // Rate at which ubtc will be bought (38 USDC per ubtc).
        },
        coin(120, "usdc"), // 120 USDC to be used for buying.
        Addr::unchecked("user"),
        "ubtc".to_string(),
        vec![SwapAmountInRoute::new(1, "ubtc")],
        &vec![],
    );

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: vec![dummy_order],
    };

    // Create an execution message to process orders.
    let execute_msg = ExecuteMsg::ProcessSpotOrders {};

    // Instantiate the contract with "owner" as the deployer and deposit 120 USDC.
    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(120, "usdc"),
            "Contract",
            None,
        )
        .unwrap();

    // Update the ubtc and USDC prices to match the order rate.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices))
        .unwrap();

    // Execute the order processing.
    let resp = app
        .execute_contract(Addr::unchecked("owner"), addr.clone(), &execute_msg, &[])
        .unwrap();

    // Verify the swap occurred.

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "ubtc")
            .unwrap()
            .amount
            .u128(),
        3
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "ubtc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    // Find the order ID in the emitted events and ensure it's not present.
    let order_ids = read_processed_order_id(resp);

    assert!(order_ids.is_empty());

    // Try to cancel an order that's processing
    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::CancelSpotOrder { order_id: 0 },
            &vec![],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::ProcessSpotOrderProcessing { order_id: 0 },
        err.downcast().unwrap()
    );
}
