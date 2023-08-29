use crate::{bindings::query::ElysQuery, tests::mock::multitest::ElysApp};

use super::*;
use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response};

#[test]
fn automated_order_execution_test() {
    // Define initial wallet balances: owner with 5 USDC tokens
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("owner", coins(5, "usdc"))];

    // Create an instance of the ElysApp with the provided wallets
    let mut app = ElysApp::new_with_wallets(wallets);

    // Wrap the contract code and store it, obtaining a code ID
    let code = ContractWrapper::new(mock_execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    // Instantiate message for the mock contract
    let instantiate_msg = InstantiateMockMsg {
        epoch_cycle_interval: 1,
        orders: vec![],
    };

    // Instantiate the contract and obtain its address
    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(5, "usdc"),
            "Contract",
            None,
        )
        .unwrap();

    // Assert the initial balances of the contract and owner
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        5
    );
    assert_eq!(
        app.wrap()
            .query_balance("owner", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    // Simulate the passage of time (block execution)
    app.next_block();

    // Assert the updated balances after block execution
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        4
    );
    assert_eq!(
        app.wrap()
            .query_balance("owner", "usdc")
            .unwrap()
            .amount
            .u128(),
        1
    );
}

// Mock contract execution function
pub fn mock_execute(
    _deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // For any message, send a token to the owner
        _ => send_a_token_to_the_owner(),
    }
}

// Function to send a token to the owner
fn send_a_token_to_the_owner() -> Result<Response, ContractError> {
    let owner_address = "owner".to_owned();
    let bank_msg = BankMsg::Send {
        to_address: owner_address,
        amount: coins(1, "usdc"),
    };

    let resp = Response::new().add_message(CosmosMsg::Bank(bank_msg));

    Ok(resp)
}
