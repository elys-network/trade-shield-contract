use super::*;
use crate::state::ORDER;
use msg::InstantiateMsg;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ORDER.save(deps.storage, &msg.orders)?;
    Ok(Response::new())
}
