use cosmwasm_std::Uint128;

use crate::{tests::mock::multitest::ElysApp, types::OrderPricePair};

use super::*;
// This test case verifies that attempting to create an order without specifying the amount results in a "CoinNumber" error.
#[test]
fn coin_number() {
    // Initialize the ElysApp instance.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        epoch_cycle_interval: 1,
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

    // User "user" attempts to create an order without specifying the amount.
    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::CreateOrder {
                order_type: OrderType::StopLoss,
                order_price_pair: OrderPricePair {
                    rate: Uint128::new(17),
                    base_denom: "btc".to_string(),
                    quote_denom: "eth".to_string(),
                },
                order_source_denom: "eth".to_owned(),
                order_target_denom: "btc".to_string(),
                order_amm_routes: vec![],
            },
            &[],
        )
        .unwrap_err();

    // Verify that the error is of type "CoinNumber."
    assert_eq!(ContractError::CoinNumber, err.downcast().unwrap());
}
