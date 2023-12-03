use super::*;
use crate::action::sudo::*;
use crate::msg::SudoMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    deps: DepsMut<ElysQuery>,
    env: Env,
    msg: SudoMsg,
) -> Result<Response<ElysMsg>, ContractError> {
    match msg {
        SudoMsg::ClockEndBlock {} => process_orders(deps, env),
    }
}
