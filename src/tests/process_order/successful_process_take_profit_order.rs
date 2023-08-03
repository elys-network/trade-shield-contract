use super::*;

// in this exemple we assume :
//  Bitcoin have a value of 15
//  Ethereum have a value of 5

#[test]
fn successful_process_stop_loss_order() {
    let list_of_user: Vec<(String, Vec<Coin>)> = vec![
        ("user".to_owned(), coins(150, "eth")),
        ("owner".to_owned(), coins(1200, "btc")),
    ];

    let mut app = new_app(&list_of_user);

    let instantiate_msg = InstantiateMsg::new(vec![(
        "user",
        coin(4, "eth"),
        coin(600, "btc"),
        OrderType::TakeProfit,
    )]);

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
        600
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
                attr.key == "refunded_order_id"
                    && attr.value == instantiate_msg.orders[0].order_id.to_string()
            })
            .and_then(|attr| attr.value.parse::<u128>().ok())
    });

    assert!(refunded_id.is_some());
}
