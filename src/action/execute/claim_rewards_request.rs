use super::*;
use elys_bindings::types::EarnType;

pub fn claim_rewards_request(
    env: Env,
    info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    withdraw_type: EarnType,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::withdraw_rewards(info.sender.into_string(), withdraw_type);

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
