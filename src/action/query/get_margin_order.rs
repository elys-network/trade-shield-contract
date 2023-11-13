use super::*;

pub fn get_margin_order(
    deps: Deps<ElysQuery>,
    address: String,
    id: u64,
) -> Result<MTPResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp: MTPResponse = querier.mtp(address, id)?;
    Ok(resp)
}
