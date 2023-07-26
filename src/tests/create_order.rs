use crate::{
    contract::*,
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg},
    state::OrderType,
};
use cosmwasm_std::{coins, Addr, Event};
use cw_multi_test::{App, ContractWrapper, Executor};

#[test]
fn coin_number() {
    let mut app = App::default();

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
    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::CreateOrder {
                order_type: OrderType::StopLoss,
                stop_price: 100,
                selling_denom: "eth".to_owned(),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(ContractError::CoinNumber, err.downcast().unwrap());
}

#[test]
fn create_order() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user"), coins(150, "eth"))
            .unwrap()
    });

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg { orders: vec![] },
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
                stop_price: 255,
                selling_denom: "btc".to_owned(),
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

    let expected_event = Event::new("wasm").add_attribute("action", "create an order");
    assert_eq!(resp.has_event(&expected_event), true);

}
