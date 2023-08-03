use super::*;

// in this exemple we assume :
//  Bitcoin have a value of 10
//  Ethereum have a value of 2

#[test]
fn successful_process_stop_loss_order() {
    let list_of_user: Vec<(String, Vec<Coin>)> = vec![
        ("user".to_owned(), coins(150, "eth")),
        ("owner".to_owned(), coins(1200, "btc")),
    ];

    let mut app = new_app(&list_of_user);

    let dummy_order = Order::new_dummy();

    let instantiate_msg = InstantiateMsg {
        orders: vec![dummy_order.clone()],
    };

    let addr = new_contract_addr(&mut app, &instantiate_msg, &list_of_user);

    let resp = app
        .execute_contract(
            addr.clone(),
            addr.clone(),
            &ExecuteMsg::ProcessOrder {},
            &[],
        )
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        200
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        350
    );

    let refunded_id: Option<u128> = resp.events.iter().find_map(|e| {
        e.attributes
            .iter()
            .find(|attr| {
                attr.key == "refunded_order_id" && attr.value == dummy_order.order_id.to_string()
            })
            .and_then(|attr| attr.value.parse::<u128>().ok())
    });

    assert!(refunded_id.is_some());
}
