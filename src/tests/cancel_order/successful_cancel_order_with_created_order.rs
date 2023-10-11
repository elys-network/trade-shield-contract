use crate::tests::mock::multitest::ElysApp;

use super::*;
use get_order_id_from_events::get_order_id_from_events;

// This test case verifies the successful cancellation of a created order in the contract.
#[test]
fn successful_cancel_order_with_created_order() {
    // Create a wallet for the "user" with an initial balance of 150 ETH.
    let wallets = vec![("user", coins(150, "eth"))];

    // Initialize the ElysApp instance with the specified wallets.
    let mut app = ElysApp::new_with_wallets(wallets);

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: vec![],
    };

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

    // User "user" creates an order in the contract.
    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateOrder {
                order_type: OrderType::StopLoss,
                order_price: OrderPrice {
                    rate: Uint128::new(18),
                    base_denom: "btc".to_string(),
                    quote_denom: "eth".to_string(),
                },
                order_source_denom: "eth".to_owned(),
                order_target_denom: "btc".to_string(),
                order_amm_routes: vec![],
            },
            &coins(45, "eth"),
        )
        .unwrap();

    // Retrieve the order ID from the events emitted during order creation.
    let id = get_order_id_from_events(&resp.events).unwrap();

    // User "user" cancels the created order.
    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CancelOrder { order_id: id },
        &[],
    )
    .unwrap();

    // Verify that the "user" now has a balance of 150 ETH, and the contract address has 0 ETH.
    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        150
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        0
    );
}
