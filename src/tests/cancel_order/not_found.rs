use crate::tests::mock::multitest::ElysApp;

use super::*;
// This test case simulates a scenario where a user attempts to cancel an order that does not exist.
#[test]
fn not_found() {
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with an empty list of orders.
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: vec![],
    };

    // Specify the order ID that the user wants to cancel.
    let id: u64 = 0;

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    // Instantiate the contract and obtain its address.
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

    // User attempts to cancel the order.
    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::CancelSpotOrder { order_id: id },
            &[],
        )
        .unwrap_err();

    // Verify that the user receives an error indicating that the order was not found.
    assert_eq!(
        ContractError::SpotOrderNotFound { order_id: id },
        err.downcast().unwrap()
    );
}
