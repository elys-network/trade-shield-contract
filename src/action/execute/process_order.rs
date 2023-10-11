use cosmwasm_std::SubMsg;

use crate::bindings::query::ElysQuery;

use super::*;

pub fn process_order(
    _deps: DepsMut<ElysQuery>,
    _info: MessageInfo,
    _env: Env,
) -> Result<Response<ElysMsg>, ContractError> {
    Ok(Response::default())
}
