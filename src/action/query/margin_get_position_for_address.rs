use super::*;

pub fn margin_get_position_for_address(
    deps: Deps<ElysQuery>,
    address: String,
    pagination: PageRequest,
) -> Result<MarginGetPositionsForAddressResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp = querier.margin_get_position_for_address(address, pagination)?;

    Ok(resp)
}
