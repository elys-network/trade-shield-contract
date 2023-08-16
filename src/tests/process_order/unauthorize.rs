use super::*;
use crate::tests::mock::multitest::ElysApp;

#[test]
fn unauthorize() {
    let mut app = ElysApp::default();
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let init_msg = InstantiateMsg { orders: vec![] };
    let random_user = Addr::unchecked("random");
    let exec_msg = ExecuteMsg::ProcessOrder {};

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &init_msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let err = app
        .execute_contract(random_user.clone(), addr, &exec_msg, &[])
        .unwrap_err();

    assert_eq!(
        ContractError::Unauthorized {
            sender: random_user
        },
        err.downcast().unwrap()
    );
}
