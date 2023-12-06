use super::*;
// delegator_address, validator_address, denom
pub fn claim_validator_commission_request(
    env: Env,
    info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    validator_address: String,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::withdraw_validator_commissions(info.sender.into_string(), validator_address);

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
