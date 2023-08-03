use crate::tests::get_order_id_from_events::get_order_id_from_events;

use super::*;

#[test]
fn successful_create_order() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user"), coins(150, "eth"))
            .unwrap()
    });

    let instantiate_msg = InstantiateMsg { orders: vec![] };

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
