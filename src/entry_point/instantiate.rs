use super::*;
use crate::states::*;
use msg::InstantiateMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<ElysMsg>> {
    MAX_REPLY_ID.save(deps.storage, &0)?;
    Ok(Response::new())
}
