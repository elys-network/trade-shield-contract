use cosmwasm_std::Reply;

use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    _deps: DepsMut<ElysQuery>,
    _env: Env,
    _msg: Reply,
) -> Result<Response<ElysMsg>, ContractError> {
    Ok(Response::default())
}
