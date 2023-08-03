use super::*;

#[test]
fn not_enough_fund() {
    let list_of_user = vec![("user".to_owned(), coins(40, "eth"))];

    let mut app = new_app(&list_of_user);

    let instantiate_msg = InstantiateMsg::new(vec![]);

    let addr = new_contract_addr(&mut app, &instantiate_msg, &list_of_user);

    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateOrder {
                order_type: OrderType::TakeProfit,
                order_price: coin(255, "btc"),
            },
            &coins(45, "eth"),
        )
        .unwrap_err();
    let error_msg = "error executing WasmMsg:\nsender: user\nExecute { contract_addr: \"contract0\", msg: {\"create_order\":{\"order_type\":\"take_profit\",\"order_price\":{\"denom\":\"btc\",\"amount\":\"255\"}}}, funds: [Coin { 45 \"eth\" }] }";

    assert_eq!(error_msg.to_owned(), err.to_string());

    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        40
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
