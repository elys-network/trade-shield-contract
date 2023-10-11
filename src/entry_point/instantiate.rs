use super::*;
use crate::{bindings::query::ElysQuery, states::*};
use msg::InstantiateMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    ORDER.save(deps.storage, &vec![])?;

    Ok(Response::new())
}
