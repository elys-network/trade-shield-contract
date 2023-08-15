use crate::tests::mock::multitest::ElysApp;

use super::*;

#[test]
fn not_found() {
    let mut app = ElysApp::new();

    let instantiate_msg: InstantiateMsg = InstantiateMsg { orders: vec![] };
    let id: u128 = 0;

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

    let err: Result<Binary, StdError> = app
        .wrap()
        .query_wasm_smart(&addr, &QueryMsg::GetOrder { order_id: id });
    let err = err.unwrap_err();
    let error_reference = StdError::GenericErr {
        msg: format!(
            "Querier contract error: {}",
            ContractError::OrderNotFound { order_id: id }.to_string()
        ),
    };
    assert_eq!(err, error_reference);
}
