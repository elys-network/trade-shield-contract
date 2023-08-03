use super::*;
use get_order_id_from_events::get_order_id_from_events;

#[test]
fn successful_cancel_order_with_created_order() {
    let list_of_user: Vec<(String, Vec<Coin>)> = vec![("user".to_owned(), coins(150, "eth"))];

    let mut app = new_app(&list_of_user);

    let instantiate_msg = InstantiateMsg {
        orders: vec![Order::new_dummy()],
    };

    let addr = new_contract_addr(&mut app, &instantiate_msg, &list_of_user);

    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateOrder {
                order_type: OrderType::TakeProfit,
                order_price: coin(255, "eth"),
            },
            &coins(45, "eth"),
        )
        .unwrap();
    let id = get_order_id_from_events(&resp.events).unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CancelOrder { order_id: id },
        &[],
    )
    .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        150
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
