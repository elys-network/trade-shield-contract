use crate::tests::mock::multitest::ElysApp;

use super::*;

#[test]
fn unauthorized() {
    let mut app = ElysApp::new();

    let instantiate_msg = InstantiateMsg {
        orders: vec![Order::new_dummy()],
    };
    let id = instantiate_msg.orders[0].order_id.clone().to_owned();

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
