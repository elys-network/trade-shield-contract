use super::*;
use cosmwasm_std::{Uint128, Coin};

pub fn join_amm_pool_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    pool_id: u64,
	max_amounts_in: Vec<Coin>,
	share_amount_out: Uint128,
    no_remaining: bool,
) -> Result<Response<ElysMsg>, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);
    let mut max_amounts_in_real_denom = vec![];
    
    // Re-initiate the coin input with the real denom
    for coin in max_amounts_in {
        let denom_entry = querier.get_asset_profile(coin.denom.clone())?;
        let real_denom = denom_entry.entry.denom;

        let real_coin = Coin {
            denom: real_denom.clone(),
            amount: coin.amount,
        };

        max_amounts_in_real_denom.push(real_coin.clone());
    }

    // Construct amm join pool message.
    let msg: ElysMsg = ElysMsg::amm_join_pool(
        info.sender.into_string(),
        pool_id,
        max_amounts_in_real_denom,
        share_amount_out,
        no_remaining,
    );

    let resp = Response::new().add_message(msg);
    Ok(resp)
}
