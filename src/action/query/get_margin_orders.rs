use super::*;

pub fn get_margin_orders(
    deps: Deps<ElysQuery>,
    pagination: PageRequest,
) -> Result<MarginQueryPositionsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp: MarginQueryPositionsResponse = querier.positions(pagination)?;

    Ok(resp)
}
