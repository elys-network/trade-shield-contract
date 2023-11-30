use crate::bindings::query::ElysQuery;

use super::*;
pub fn elys_cancel_unstake_request(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    delegator_address: String,
    validator_address: String,
    amount: Coin,
    creation_height: i64,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::cancel_unbonding(
        delegator_address,
        validator_address,
        amount,
        creation_height,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}