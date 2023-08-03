use super::*;
use query_resp::GetOrderResp;

#[test]
fn successful_query_message() {
    let list_of_user: Vec<(String, Vec<Coin>)> = vec![
        ("user".to_owned(), coins(150, "eth")),
        ("owner".to_owned(), coins(1200, "btc")),
    ];

    let mut app = new_app(&list_of_user);

    let instantiate_msg = InstantiateMsg {
        orders: vec![Order::new_dummy()],
    };

    let addr = new_contract_addr(&mut app, &instantiate_msg, &list_of_user);

    let id = instantiate_msg.orders[0].order_id;

    let resp: GetOrderResp = app
        .wrap()
        .query_wasm_smart(&addr, &QueryMsg::GetOrder { order_id: id })
        .unwrap();

    assert_eq!(
        resp,
        GetOrderResp {
            order: Order::new_dummy(),
        }
    );
}
