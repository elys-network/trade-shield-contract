use super::*;
use cosmwasm_std::{Coin, Decimal};

pub fn swap_estimation_by_denom(
    deps: Deps<ElysQuery>,
    amount: Coin,
    denom_in: String,
    denom_out: String,
    _user_address: String,
) -> Result<AmmSwapEstimationByDenomResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp: AmmSwapEstimationByDenomResponse =
        querier.amm_swap_estimation_by_denom(&amount, denom_in, denom_out, &Decimal::zero())?;

    Ok(resp)
}
