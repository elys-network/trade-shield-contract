use cosmwasm_std::Coin;
use cw_multi_test::{ContractWrapper, App};
use crate::{contract::{*, self}, msg::{InstantiateMsg, QueryMsg}, state::{Order, OrderType, ORDER}, error::ContractError};
#[test]
fn query_message() {
    let mut app = App::default();

    let instantiate_msg = InstantiateMsg{ orders : vec![Order::dummy()]};
    let id = instantiate_msg.orders[0].id.clone().to_owned();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let addr = app.instantiate_contract(code_id, Addr::unchecked("owner"),&instantiate_msg ,"Contract",None).unwrap();

    let resp = app.wrap().query_wasm_smart(&addr, QueryMsg::GetOrder { order_id: id }).unwrap();

    assert_eq!(resp, instantiate_msg.orders[0]);
}

#[test]

fn not_found() {
    let mut app = App::default();

    let instantiate_msg = InstantiateMsg{ orders : vec![]};
    let id = instantiate_msg.orders[0].id.clone().to_owned();


    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let addr = app.instantiate_contract(code_id, Addr::unchecked("owner"),&instantiate_msg ,"Contract",None).unwrap();

    let resp = app.wrap().query_wasm_smart(&addr, QueryMsg::GetOrder { order_id: id }).unwrap_err();

    assert_eq!(resp, ContractError::OrderNotFound { order_id: id });
}