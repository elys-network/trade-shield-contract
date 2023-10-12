use crate::{tests::mock::multitest::ElysApp, types::SwapAmountInRoute};

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
    let prices_at_t0 = vec![coin(20000, "btc"), coin(1, "usdc")];
    let prices_at_t1 = vec![coin(30000, "btc"), coin(1, "usdc")];

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    // Create a "limit sell" order (dummy order) with a specific rate and balance.
    let dummy_order = Order::new(
        OrderType::LimitSell,
        OrderPrice {
            base_denom: "btc".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Uint128::new(30000), // Rate at which BTC will be sold (30,000 USDC per BTC).
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
    let execute_msg = ExecuteMsg::ProcessOrders {};

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
        .execute_contract(addr.clone(), addr.clone(), &execute_msg, &[])
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
    let order_id: Option<u128> = resp.events.iter().find_map(|e| {
        e.attributes
            .iter()
            .find(|attr| {
                attr.key == "order_id"
                    && attr.value == instantiate_msg.orders[0].order_id.to_string()
            })
            .and_then(|attr| attr.value.parse::<u128>().ok())
    });

    assert!(order_id.is_none());

    // Update the BTC and USDC prices to match the order rate.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t1))
        .unwrap();

    // Execute the order processing again.
    let resp = app
        .execute_contract(addr.clone(), addr.clone(), &execute_msg, &[])
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
    let order_id: Option<u128> = resp.events.iter().find_map(|e| {
        e.attributes
            .iter()
            .find(|attr| {
                attr.key == "order_id"
                    && attr.value == instantiate_msg.orders[0].order_id.to_string()
            })
            .and_then(|attr| attr.value.parse::<u128>().ok())
    });

    assert!(order_id.is_some());
}
