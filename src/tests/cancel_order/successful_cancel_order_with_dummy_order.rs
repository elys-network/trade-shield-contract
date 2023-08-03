use super::*;

#[test]
fn successful_cancel_order_with_dummy_order() {
    let list_of_user: Vec<(String, Vec<Coin>)> = vec![
        ("user".to_owned(), coins(150, "eth")),
        ("owner".to_owned(), coins(1200, "btc")),
    ];

    let mut app = new_app(&list_of_user);

    let instantiate_msg = InstantiateMsg {
        orders: vec![Order::new_dummy()],
    };

    let addr = new_contract_addr(&mut app, &instantiate_msg, &list_of_user);

    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CancelOrder {
            order_id: instantiate_msg.orders[0].order_id,
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
