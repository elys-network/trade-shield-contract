use super::*;
use cosmwasm_std::Coin;

pub fn elys_cancel_unstake_request(
    info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    validator_address: String,
    amount: Coin,
    creation_height: i64,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::cancel_unbonding(
        info.sender.into_string(),
        validator_address,
        amount,
        creation_height,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
