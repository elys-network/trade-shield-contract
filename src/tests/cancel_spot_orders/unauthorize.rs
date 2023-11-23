use super::*;

#[test]
fn unauthorize() {
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with an empty list of orders.
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        spot_orders: vec![],
        margin_orders: vec![],
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

    let sender = Addr::unchecked("user");

    let err = app
        .execute_contract(
            sender.clone(),
            addr,
            &&ExecuteMsg::CancelSpotOrders {
                order_ids: None,
                owner_address: "not_user".to_string(),
                order_type: None,
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::Unauthorized { sender },
        err.downcast().unwrap()
    );
}
