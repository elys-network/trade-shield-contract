use cosmwasm_std::Int128;

use super::*;

pub fn close_margin_position(
    info: MessageInfo,

    id: u64,
    amount: Int128,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::margin_close_position(info.sender, id, amount.i128());

    let resp = Response::new().add_message(CosmosMsg::Custom(msg));

    Ok(resp)
}
