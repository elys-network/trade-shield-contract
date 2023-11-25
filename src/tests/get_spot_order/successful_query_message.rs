use super::*;
use query_resp::GetSpotOrderResp;
// This test case verifies the successful query of an existing order in the contract.
#[test]
fn successful_query_message() {
    // Initialize the ElysApp instance.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with an initial dummy order.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![SpotOrder::new_dummy()],
        margin_orders: vec![],
    };

    // Extract the order ID from the dummy order.
    let id = instantiate_msg.spot_orders[0].order_id.clone().to_owned();

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
    let resp: GetSpotOrderResp = app
        .wrap()
        .query_wasm_smart(&addr, &QueryMsg::GetSpotOrder { order_id: id })
        .unwrap();

    // Verify that the response matches the expected order (the initial dummy order).
    assert_eq!(
        resp,
        GetSpotOrderResp {
            order: SpotOrder::new_dummy(),
        }
    );
}
