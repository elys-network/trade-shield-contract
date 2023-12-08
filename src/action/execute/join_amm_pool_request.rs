use super::*;
use cosmwasm_std::{Uint128, Coin};

pub fn join_amm_pool_request(
    info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    pool_id: u64,
	max_amounts_in: Vec<Coin>,
	share_amount_out: Uint128,
    no_remaining: bool,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg: ElysMsg = ElysMsg::amm_join_pool(
        info.sender.into_string(),
        pool_id,
        max_amounts_in,
        share_amount_out,
        no_remaining,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
