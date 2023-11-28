use crate::msg::query_resp::GetSpotOrdersResp;

use super::*;

// This test case verifies the successful creation of a "market" order in the contract.
// A "market" order is designed to execute right after.
#[test]
fn successful_create_market_buy_order() {
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
        spot_orders: vec![],
        margin_orders: vec![],
    };

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query)
        .with_reply(reply)
        .with_sudo(sudo);
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
    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CreateSpotOrder {
            order_type: OrderType::MarketBuy,
            // Empty order price - not utilized in market orders
            order_price: None,
            order_source_denom: "btc".to_string(),
            order_target_denom: "usdc".to_string(),
        },
        &coins(2, "btc"), // User's BTC balance.
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

    let res: GetSpotOrdersResp = app
        .wrap()
        .query_wasm_smart(
            addr.clone(),
            &QueryMsg::GetSpotOrders {
                pagination: PageRequest::new(5),
                order_owner: None,
                order_type: None,
            },
        )
        .unwrap();

    assert_eq!(res.orders[0].status, Status::Processed);
}
