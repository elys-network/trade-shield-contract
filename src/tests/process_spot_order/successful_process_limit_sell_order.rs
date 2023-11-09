use crate::{tests::read_processed_order_id::read_processed_order_id, types::SwapAmountInRoute};

use super::*;
use cosmwasm_std::{coins, Coin};
// This test case verifies the successful processing of a "limit sell" order in the contract.
// The scenario involves a "limit sell" order created by a user to sell BTC at a specific price.
// - Initially, the BTC price is 20,000 USDC, and the order rate is set at 30,000 USDC per BTC.
// - The order is created with 2 BTC to be sold at the specified rate.
// - After processing the order, the BTC price increases to 30,000 USDC, matching the order rate.
// - The order is executed, and the user receives 60,000 USDC.
#[test]
fn successful_process_limit_sell_order() {
    // Initialize the ElysApp instance with wallets for "owner" and "user."
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("owner", coins(2, "btc")), ("user", vec![])];
    let mut app = ElysApp::new_with_wallets(wallets);

    // Define BTC and USDC prices at two different time intervals (t0 and t1).
    let prices_at_t0 = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(20000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];
    let prices_at_t1 = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(30000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];
    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
    let code_id = app.store_code(Box::new(code));

    // Create a "limit sell" order (dummy order) with a specific rate and balance.
    let dummy_order = SpotOrder::new(
        SpotOrderType::LimitSell,
        SpotOrderPrice {
            base_denom: "btc".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Decimal::from_atomics(Uint128::new(30000), 0).unwrap(), // Rate at which BTC will be sold (30,000 USDC per BTC).
        },
        coin(2, "btc"), // 2 BTC to be sold.
        Addr::unchecked("user"),
        "usdc".to_string(),
        vec![SwapAmountInRoute::new(1, "usdc")],
        &vec![],
    );

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: vec![dummy_order],
    };

    // Create an execution message to process orders.
    let execute_msg = ExecuteMsg::ProcessSpotOrders {};

    // Instantiate the contract with "owner" as the deployer and deposit 2 BTC.
    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(2, "btc"),
            "Contract",
            None,
        )
        .unwrap();

    // Set the initial BTC and USDC prices.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t0))
        .unwrap();

    // Execute the order processing.
    let resp = app
        .execute_contract(Addr::unchecked("owner"), addr.clone(), &execute_msg, &[])
        .unwrap();

    // Verify the resulting balances after order processing.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        2
    );
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
            .query_balance("user", "btc")
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

    // Update the BTC and USDC prices to match the order rate.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t1))
        .unwrap();

    let resp = app
        .execute_contract(Addr::unchecked("owner"), addr.clone(), &execute_msg, &[])
        .unwrap();

    // Verify the swap occurred.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        60000
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
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

    // Execute the order processing again.
    let resp = app
        .execute_contract(Addr::unchecked("owner"), addr.clone(), &execute_msg, &[])
        .unwrap();

    // Verify the resulting balances after order processing.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
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
            .query_balance("user", "btc")
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
        60000 // User receives 60,000 USDC from the executed "limit sell" order.
    );

    // Find the order ID in the emitted events and ensure it's present.
    let order_ids = read_processed_order_id(resp);

    assert_eq!(order_ids[0], 0);
}
