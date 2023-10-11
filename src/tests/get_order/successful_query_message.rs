use crate::tests::mock::multitest::ElysApp;

use super::*;
use query_resp::GetOrderResp;
// This test case verifies the successful query of an existing order in the contract.
#[test]
fn successful_query_message() {
    // Initialize the ElysApp instance.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with an initial dummy order.
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: vec![Order::new_dummy()],
    };

    // Extract the order ID from the dummy order.
    let id = instantiate_msg.orders[0].order_id.clone().to_owned();

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    // Instantiate the contract with "owner" as the deployer.
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

    // Query the contract for the existing order.
    let resp: GetOrderResp = app
        .wrap()
        .query_wasm_smart(&addr, &QueryMsg::GetOrder { order_id: id })
        .unwrap();

    // Verify that the response matches the expected order (the initial dummy order).
    assert_eq!(
        resp,
        GetOrderResp {
            order: Order::new_dummy(),
        }
    );
}
