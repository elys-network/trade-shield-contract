use crate::tests::get_order_id_from_events::get_order_id_from_events;

use super::*;

// This test case verifies the successful creation of a "market" order in the contract.
// A "market" order is designed to execute right after.
#[test]
fn successful_create_stop_loss_order() {
    // Create a wallet for the "user" with an initial balance of 2 BTC.
    let wallet = vec![("user", coins(2, "btc"))];

    let prices = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(30000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];

    // Initialize the ElysApp instance with the specified wallet.
    let mut app = ElysApp::new_with_wallets(wallet);

    // Set the BTC and USDC prices.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices))
        .unwrap();

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: vec![],
    };

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
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

    // User "user" creates a "market" order for BTC to USDC.
    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateSpotOrder {
                order_type: SpotOrderType::MarketBuy,
                // Empty order price - not utilized in market orders
                order_price: SpotOrderPrice {
                    base_denom: "".to_string(),
                    quote_denom: "".to_string(),
                    rate: Decimal::zero(),
                },
                order_amm_routes: Some(vec![SwapAmountInRoute::new(1, "usdc")]),
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

    // Verify that the contract address has swap the 2 BTC for the order (market).
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    // Verify that the contract address now holds the 60000 usdc.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        60000
    );

    // Verify that an order ID is emitted in the contract's events.
    assert!(get_order_id_from_events(&resp.events).is_some());

    app.execute_contract(
        Addr::unchecked("owner"),
        addr.clone(),
        &ExecuteMsg::ProcessSpotOrders {},
        &[],
    )
    .unwrap();

    // Verify that the user got his swaped token
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        60000
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );
}
