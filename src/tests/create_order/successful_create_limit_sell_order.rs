use crate::tests::{get_order_id_from_events::get_order_id_from_events, mock::multitest::ElysApp};

use super::*;
// This test case verifies the successful creation of a "limit sell" order in the contract.
// In a "limit sell" order, the user specifies the desired selling price (rate) for their BTC in terms of USDC.
// If the market price of BTC reaches or exceeds the specified rate, the order is executed.
#[test]
fn successful_create_limit_sell_order() {
    // Create a wallet for the "user" with an initial balance of 2 BTC.
    let wallet = vec![("user", coins(2, "btc"))];

    // Initialize the ElysApp instance with the specified wallet.
    let mut app = ElysApp::new_with_wallets(wallet);

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        epoch_cycle_interval: 1,
        orders: vec![],
    };

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    // Instantiate the contract with "owner" as the deployer.
    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();

    // User "user" creates a "limit sell" order for BTC, specifying the selling price in USDC.
    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateOrder {
                order_type: OrderType::LimitSell,
                order_price_pair: OrderPricePair {
                    base_denom: "btc".to_string(),
                    quote_denom: "usdc".to_string(),
                    rate: Uint128::new(40000), // The desired selling price of 40000 USDC per BTC.
                },
                order_amm_routes: vec![],
                order_source_denom: "btc".to_string(),
                order_target_denom: "usdc".to_string(),
            },
            &coins(2, "btc"), // User's BTC balance.
        )
        .unwrap();

    // Verify that the "user" no longer has any BTC after creating the order.
    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    // Verify that the contract address now holds the 2 BTC from the order.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        2
    );

    // Verify that an order ID is emitted in the contract's events.
    assert!(get_order_id_from_events(&resp.events).is_some());
}