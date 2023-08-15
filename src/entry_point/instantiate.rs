use super::*;
use crate::{bindings::query::ElysQuery, states::*};
use msg::InstantiateMsg;

pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ORDER.save(deps.storage, &msg.orders)?;

    Ok(Response::new())
}
