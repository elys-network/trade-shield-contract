use crate::bindings::query::ElysQuery;

use super::*;
pub fn eden_vest_request(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    creator: String,
    amount: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg: ElysMsg = ElysMsg::eden_vesting(
        creator,
        Int128::from(amount),
        "ueden".to_string(),
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}