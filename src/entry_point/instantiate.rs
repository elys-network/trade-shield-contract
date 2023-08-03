use super::*;
use crate::states::*;
use msg::InstantiateMsg;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ORDER.save(deps.storage, &msg.orders)?;
    PRICES.save(deps.storage, &msg.prices)?;

    Ok(Response::new())
}
