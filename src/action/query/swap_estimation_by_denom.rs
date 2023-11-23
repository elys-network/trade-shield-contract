use super::*;
use cosmwasm_std::Coin;

pub fn swap_estimation_by_denom(
    deps: Deps<ElysQuery>,
    amount: Coin,
    denom_in: String,
    denom_out: String,
) -> Result<AmmSwapEstimationByDenomResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp: AmmSwapEstimationByDenomResponse =
        querier.amm_swap_estimation_by_denom(&amount, denom_in, denom_out)?;

    Ok(resp)
}
