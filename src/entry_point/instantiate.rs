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
    SPOT_ORDER.save(deps.storage, &vec![])?;
    MARGIN_ORDER.save(deps.storage, &vec![])?;
    REPLY_INFO.save(deps.storage, &vec![])?;

    Ok(Response::new())
}
