use crate::{
    contract::*,
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg},
    state::Order,
};
use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor};

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
