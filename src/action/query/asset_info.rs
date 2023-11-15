use super::*;

pub fn asset_info(
    deps: Deps<ElysQuery>,
    denom: String,
) -> Result<OracleAssetInfoResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    Ok(querier.asset_info(denom)?)
}
