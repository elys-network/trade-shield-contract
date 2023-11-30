use super::*;
// delegator_address, validator_address, denom
pub fn claim_validator_commission_request(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    delegator_address: String,
    validator_address: String,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::withdraw_validator_commissions(
        delegator_address,
        validator_address,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}