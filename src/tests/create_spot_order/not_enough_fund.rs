use cosmwasm_std::{OverflowError, StdError};

use super::*;

// This test case verifies that attempting to create an order with insufficient funds results in an "Overflow" error.
#[test]
fn not_enough_fund() {
    // Create a wallet for the "user" with an initial balance of 40 ETH.
    let wallets = vec![("user", coins(40, "eth"))];

    // Initialize the ElysApp instance with the specified wallets.
    let mut app = ElysApp::new_with_wallets(wallets);

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: vec![],
    };

    // Define the parameters for creating an order with insufficient funds.
    let create_order_msg = ExecuteMsg::CreateSpotOrder {
        order_type: SpotOrderType::LimitSell,
        order_price: SpotOrderPrice {
            base_denom: "btc".to_string(),
            quote_denom: "eth".to_string(),
            rate: Decimal::from_atomics(Uint128::new(19), 0).unwrap(),
        },
        order_amm_routes: vec![],
        order_source_denom: "eth".to_string(),
        order_target_denom: "btc".to_string(),
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

    // User "user" attempts to create an order with insufficient funds.
    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &create_order_msg,
            &coins(45, "eth"),
        )
        .unwrap_err();

    // Define the expected "Overflow" error message.
    let error_msg: StdError = StdError::Overflow {
        source: OverflowError::new(cosmwasm_std::OverflowOperation::Sub, 40, 45),
    };

    // Verify that the error message matches the expected error.
    assert_eq!(error_msg, err.downcast().unwrap());

    // Verify that the "user" still has a balance of 40 ETH, and the contract address has 0 ETH.
    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        40
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
