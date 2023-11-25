use std::str::FromStr;

use super::*;

#[test]
fn successful_create_margin_market_order() {
    // Create a wallet for the "user" with an initial balance of 10 BTC.
    let wallet = vec![("user", coins(10, "btc"))];

    // Initialize the ElysApp instance with the specified wallet.
    let mut app = ElysApp::new_with_wallets(wallet);

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        margin_orders: vec![],
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

    // User "user" creates a "MakerBuy" margin order for BTC
    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CreateMarginOrder {
            position: MarginPosition::Long,
            collateral: coin(10, "btc"),
            leverage: Decimal::from_atomics(Uint128::new(215), 2).unwrap(),
            borrow_asset: "btc".to_string(),
            take_profit_price: Decimal::from_atomics(Uint128::new(200), 2).unwrap(),
            order_type: OrderType::MarketBuy,
            trigger_price: Some(OrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_str("1.7").unwrap(),
            }),
        },
        &coins(10, "btc"), // User's BTC balance.
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

    // Verify that the contract address send the BTC to the Margin Module.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    let last_module = app
        .init_modules(|router, _, store| router.custom.get_last_module(store).unwrap())
        .unwrap();

    assert_eq!(last_module, "MarginBrokerOpen");
}
