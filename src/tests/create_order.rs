use crate::{
    contract::*,
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg},
    state::OrderType,
};
use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor};


#[test] 
fn coin_number() {
    let mut app = App::default();

    let instantiate_msg = InstantiateMsg {
        orders: vec![],
    };
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
        &ExecuteMsg::CreateOrder { order_type: OrderType::StopLoss, stop_price: 100 , selling_denom: "eth".to_owned() },
        &[],
    )
    .unwrap_err();

    assert_eq!(ContractError::CoinNumber, err.downcast().unwrap());
}
