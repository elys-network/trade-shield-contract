use super::*;
use crate::types::EarnType;

pub fn claim_rewards_request(
    env: Env,
    info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    withdraw_type: EarnType,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::withdraw_rewards(
        env.contract.address.into_string(),
        info.sender.into_string(),
        withdraw_type,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}