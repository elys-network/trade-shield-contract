use cosmwasm_std::Coin;

use super::*;

#[test]
fn successful_process_stop_loss_order() {
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

    let dummy_order = Order::new_dummy();
    let denom_values: Vec<Coin> = vec![coin(10, "btc"), coin(2, "eth")];

    let instantiate_msg = InstantiateMsg {
        orders: vec![dummy_order.clone()],
        prices: denom_values,
    };

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(1200, "btc"),
            "Contract",
            None,
        )
        .unwrap();

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
