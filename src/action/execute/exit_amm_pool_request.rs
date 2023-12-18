use super::*;
use cosmwasm_std::{Uint128, Coin};

pub fn exit_amm_pool_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    pool_id: u64,
	min_amounts_out: Vec<Coin>,
	share_amount_in: Uint128,
    token_out_denom: String,
) -> Result<Response<ElysMsg>, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);
    let mut min_amounts_out_real_denom = vec![];
    
    // Re-initiate the coin input with the real denom
    for coin in min_amounts_out {
        let denom_entry = querier.get_asset_profile(coin.denom.clone())?;
        let real_denom = denom_entry.entry.denom;

        let real_coin = Coin {
            denom: real_denom.clone(),
            amount: coin.amount,
        };

        min_amounts_out_real_denom.push(real_coin.clone());
    }

    let msg: ElysMsg = ElysMsg::amm_exit_pool(
        info.sender.into_string(),
        pool_id,
        min_amounts_out_real_denom,
        share_amount_in,
        token_out_denom,
    );

    let resp = Response::new().add_message(msg);
    Ok(resp)
}
