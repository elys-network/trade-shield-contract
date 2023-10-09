use super::*;

#[test]
fn order_price_pair() {
    let wallets = vec![("user", coins(45, "eth"))];

    let mut app = ElysApp::new_with_wallets(wallets);

    let instantiate_msg = InstantiateMockMsg {
        epoch_cycle_interval: 1,
        orders: vec![],
    };

    let create_order_msg = ExecuteMsg::CreateOrder {
        order_type: OrderType::LimitSell,
        order_price_pair: OrderPricePair {
            base_denom: "eth".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Uint128::new(1700),
        },
        order_amm_routes: vec![],
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

    let error_msg = ContractError::OrderPricePair;

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
