use crate::{
    contract::*,
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg},
    state::{Order, OrderType},
};
use cosmwasm_std::{coins, Addr, Event};
use cw_multi_test::{App, ContractWrapper, Executor};

fn get_attribute_value(events: &Vec<Event>, key: &str) -> Option<String> {
    for event in events {
        if let Some(attr) = event.attributes.iter().find(|attr| attr.key == key) {
            return Some(attr.value.clone());
        }
    }
    None
}

#[test]
fn not_found() {
    let mut app = App::default();

    let instantiate_msg = InstantiateMsg { orders: vec![] };
    let id = "id".to_owned();

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
            &ExecuteMsg::CancelOrder {
                order_id: id.clone(),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::OrderNotFound { order_id: id },
        err.downcast().unwrap()
    );
}

#[test]
fn unauthorized() {
    let mut app = App::default();

    let instantiate_msg = InstantiateMsg {
        orders: vec![Order::new_dummy()],
    };
    let id = instantiate_msg.orders[0].id.clone().to_owned();

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
            Addr::unchecked("not_user"),
            addr,
            &ExecuteMsg::CancelOrder {
                order_id: id.clone(),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::Unauthorized {
            sender: Addr::unchecked("not_user")
        },
        err.downcast().unwrap()
    );
}

#[test]
fn cancel_order() {
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
    let id = get_attribute_value(&resp.events, "order_id").unwrap();

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
