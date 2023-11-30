use super::*;

// Tests the behavior when creating a "limit sell" order with the wrong source fund denomination.
// - Initializes the contract with a user having 45 ETH.
// - Attempts to create an order with the source denomination as "usdc", which doesn't match the user's balance, resulting in an expected error.
// - Verifies that the user's ETH balance remains 45, and the contract's ETH balance stays at 0.
#[test]
fn order_wrong_fund() {
    let wallets = vec![("user", coins(45, "eth"))];
    let mut app = ElysApp::new_with_wallets(wallets);

    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        margin_orders: vec![],
    };

    let create_order_msg = ExecuteMsg::CreateSpotOrder {
        order_type: SpotOrderType::LimitSell,
        order_price: Some(OrderPrice {
            base_denom: "btc".to_string(),
            quote_denom: "eth".to_string(),
            rate: Decimal::from_atomics(Uint128::new(19), 0).unwrap(),
        }),

        order_source_denom: "usdc".to_string(), // Incorrect source denomination.
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

    let error_msg = ContractError::SpotOrderWrongFund;

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
