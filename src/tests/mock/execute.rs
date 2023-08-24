use cosmwasm_std::{coins, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response};

use crate::{bindings::query::ElysQuery, msg::ExecuteMsg, ContractError};

pub fn execute(
    _deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        _ => pass_a_token_to_the_owner(),
    }
}

fn pass_a_token_to_the_owner() -> Result<Response, ContractError> {
    let owner_address = "owner".to_owned();
    let bank_msg = BankMsg::Send {
        to_address: owner_address,
        amount: coins(1, "usdc"),
    };

    let resp = Response::new().add_message(CosmosMsg::Bank(bank_msg));

    Ok(resp)
}
