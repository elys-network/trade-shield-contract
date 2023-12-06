use super::*;
use cosmwasm_std::Int128;

pub fn eden_vest_request(
    env: Env,
    info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    amount: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg: ElysMsg = ElysMsg::eden_vesting(
        info.sender.into_string(),
        Int128::from(amount),
        "ueden".to_string(),
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
