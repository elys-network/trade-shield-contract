use super::*;

#[test]
fn successful_cancel_order_with_dummy_order() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user"), coins(150, "eth"))
            .unwrap();

        router
            .bank
            .init_balance(storage, &Addr::unchecked("owner"), coins(1200, "btc"))
            .unwrap();
    });

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let dummy_order = Order::new_dummy();

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                orders: vec![dummy_order.clone()],
            },
            &coins(1200, "btc"),
            "Contract",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CancelOrder {
            order_id: dummy_order.id,
        },
        &[],
    )
    .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        1000
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        200
    );
}
