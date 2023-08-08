use super::*;

use mock::oracle::*;

// in this exemple we assume :
//  USDC have a value of 1
//  Bitcoin have a value of 20 000

#[test]
fn successful_process_stop_loss_order() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user"), vec![])
            .unwrap();

        router
            .bank
            .init_balance(storage, &Addr::unchecked("owner"), coins(2, "btc"))
            .unwrap();
    });

    let dummy_order = Order::new(
        OrderType::StopLoss,
        coin(20000, "usdc"),
        coin(2, "btc"),
        Addr::unchecked("user"),
        &vec![],
    );

    let instantiate_msg = InstantiateMsg {
        orders: vec![dummy_order],
    };

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(2, "btc"),
            "Contract",
            None,
        )
        .unwrap();

    app.execute_contract(
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
        2
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

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
        0
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        40000
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
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
