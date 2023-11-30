use super::*;
use cosmwasm_std::Int128;

pub fn eden_cancel_vest_request(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    creator: String,
    amount: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::eden_cancel_vesting(
        creator,
        Int128::from(amount),
        "ueden".to_string(),
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}