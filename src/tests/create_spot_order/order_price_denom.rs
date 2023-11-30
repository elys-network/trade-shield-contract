use super::*;

// Tests the behavior when creating a "limit sell" order with an invalid order price.
// - Initializes the contract with a user having 45 ETH.
// - Attempts to create an order with an invalid order price (ETH/USDC for BTC/ETH) resulting in an expected error.
// - Verifies that the user's ETH balance remains 45, and the contract's ETH balance stays at 0.
#[test]
fn order_price_denom() {
    let wallets = vec![("user", coins(45, "eth"))];

    let mut app = ElysApp::new_with_wallets(wallets);

    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        margin_orders: vec![],
    };

    let create_order_msg = ExecuteMsg::CreateSpotOrder {
        order_type: SpotOrderType::LimitSell,
        order_price: Some(OrderPrice {
            base_denom: "eth".to_string(),
            quote_denom: "usdc".to_string(), // Invalid pair.
            rate: Decimal::from_atomics(Uint128::new(1700), 0).unwrap(),
        }),

        order_source_denom: "eth".to_string(),
        order_target_denom: "btc".to_string(),
    };

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

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

    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &create_order_msg,
            &coins(45, "eth"),
        )
        .unwrap_err();

    let error_msg = ContractError::OrderPriceDenom;

    assert_eq!(error_msg, err.downcast().unwrap());

    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        45
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        0
    );
}
