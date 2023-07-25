use crate::{
    contract::*,
    error::ContractError,
    msg::{GetOrderResp, InstantiateMsg, QueryMsg},
    state::Order,
};
use cosmwasm_std::{Addr, Binary, StdError};
use cw_multi_test::{App, ContractWrapper, Executor};

#[test]
fn query_message() {
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

#[test]
fn not_found() {
    let mut app: App = App::default();

    let instantiate_msg: InstantiateMsg = InstantiateMsg { orders: vec![] };
    let id: String = "id".to_owned();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id: u64 = app.store_code(Box::new(code));
    let addr: Addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let err: Result<Binary, StdError> = app.wrap().query_wasm_smart(
        &addr,
        &QueryMsg::GetOrder {
            order_id: id.clone(),
        },
    );
    let err = err.unwrap_err();
    let error_reference = StdError::GenericErr {
        msg: format!(
            "Querier contract error: {}",
            ContractError::OrderNotFound { order_id: id }.to_string()
        ),
    };
    assert_eq!(err, error_reference);
}
