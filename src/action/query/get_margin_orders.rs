use super::*;

pub fn get_margin_orders(
    deps: Deps<ElysQuery>,
    pagination: PageRequest,
) -> Result<PositionsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp: PositionsResponse = querier.positions(pagination)?;
    Ok(resp)
}
