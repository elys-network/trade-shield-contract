use crate::bindings::query::ElysQuery;
use crate::types::EarnType;

use super::*;
pub fn claim_rewards_request(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    delegator_address: String,
    withdraw_type: EarnType,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::withdraw_rewards(
        delegator_address,
        withdraw_type,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}