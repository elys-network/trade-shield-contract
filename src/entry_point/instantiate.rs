use super::*;
use crate::{bindings::query::ElysQuery, states::*};
use msg::InstantiateMsg;

pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<ElysMsg>> {
    ORDER.save(deps.storage, &vec![])?;

    Ok(Response::new())
}
