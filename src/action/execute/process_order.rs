use cosmwasm_std::{coin, coins, to_binary, SubMsg, WasmMsg};

use crate::{bindings::query::ElysQuery, msg::ExecuteMsg};

use super::*;

pub fn process_order(
    _deps: DepsMut<ElysQuery>,
    _info: MessageInfo,
    _env: Env,
) -> Result<Response<ElysMsg>, ContractError> {
    Ok(Response::new())
}
