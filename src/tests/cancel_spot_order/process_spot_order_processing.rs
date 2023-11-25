use crate::tests::read_processed_order_id::read_processed_order_id;

use super::*;
use cosmwasm_std::{coins, Coin};

#[test]
fn process_spot_order_processing() {
    // Initialize wallets for "owner" and "user."
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("owner", coins(120, "usdc")), ("user", vec![])];
    let mut app = ElysApp::new_with_wallets(wallets);

    // Define ubtc and USDC prices at two different time intervals (t0 and t1).
    let prices: Vec<Price> = vec![
        Price::new("ubtc", Decimal::from_atomics(Uint128::new(40), 0).unwrap()),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query)
        .with_reply(reply)
        .with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    // Create a "limit buy" order (dummy order) with a specific rate and balance.
    let dummy_order = SpotOrder::new(
        OrderType::LimitBuy,
        Some(OrderPrice {
            base_denom: "ubtc".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Decimal::from_atomics(Uint128::new(38), 0).unwrap(), // Rate at which ubtc will be bought (38 USDC per ubtc).
        }),
        coin(120, "usdc"), // 120 USDC to be used for buying.
        Addr::unchecked("user"),
        "ubtc".to_string(),
        &vec![],
    );

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![dummy_order],
        margin_orders: vec![],
    };

    // Create an sudo message to process orders.
    let sudo_msg = SudoMsg::ClockEndBlock {};

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
    let resp = app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

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
