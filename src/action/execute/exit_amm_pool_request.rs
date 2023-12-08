use super::*;
use cosmwasm_std::{Uint128, Coin};

pub fn exit_amm_pool_request(
    info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    pool_id: u64,
	min_amounts_out: Vec<Coin>,
	share_amount_in: Uint128,
    token_out_denom: String,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg: ElysMsg = ElysMsg::amm_exit_pool(
        info.sender.into_string(),
        pool_id,
        min_amounts_out,
        share_amount_in,
        token_out_denom,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
