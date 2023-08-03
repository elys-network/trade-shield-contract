use crate::tests::get_order_id_from_events::get_order_id_from_events;

use super::*;

#[test]
fn successful_create_order() {
    let list_of_user = vec![("user".to_owned(), coins(150, "eth"))];

    let mut app = new_app(&list_of_user);

    let instantiate_msg = InstantiateMsg::new(vec![]);

    let addr = new_contract_addr(&mut app, &instantiate_msg, &list_of_user);

    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateOrder {
                order_type: OrderType::TakeProfit,
                order_price: coin(255, "btc"),
            },
            &coins(45, "eth"),
        )
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        105
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        45
    );

    assert!(get_order_id_from_events(&resp.events).is_some());
}
