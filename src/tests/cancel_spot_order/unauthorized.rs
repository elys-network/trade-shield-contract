use crate::tests::mock::multitest::ElysApp;

use super::*;
// This test case verifies that an unauthorized user is unable to cancel an order in the contract.
#[test]
fn unauthorized() {
    // Initialize the ElysApp instance.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with an order owned by the "user"
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: vec![SpotOrder::new_dummy()],
    };

    // Retrieve the order ID from the instantiated message for later use.
    let id = instantiate_msg.orders[0].order_id.clone().to_owned();

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

    // An unauthorized user (not_user) attempts to cancel the order.
    let err = app
        .execute_contract(
            Addr::unchecked("not_user"),
            addr,
            &ExecuteMsg::CancelSpotOrder {
                order_id: id.clone(),
            },
            &[],
        )
        .unwrap_err();

    // Verify that the user receives an error indicating unauthorized access.
    assert_eq!(
        ContractError::Unauthorized {
            sender: Addr::unchecked("not_user")
        },
        err.downcast().unwrap()
    );
}
