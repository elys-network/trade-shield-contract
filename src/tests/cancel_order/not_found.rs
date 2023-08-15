use crate::tests::mock::multitest::ElysApp;

use super::*;

#[test]
fn not_found() {
    let mut app = ElysApp::new();

    let instantiate_msg = InstantiateMsg { orders: vec![] };
    let id: u128 = 0;

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
            &ExecuteMsg::CancelOrder { order_id: id },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::OrderNotFound { order_id: id },
        err.downcast().unwrap()
    );
}
