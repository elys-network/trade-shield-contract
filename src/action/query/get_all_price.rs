use super::*;
use crate::{
    bindings::{querier::ElysQuerier, query::ElysQuery},
    msg::query_resp::GetAllPricesResponse,
};

pub fn get_all_prices(deps: Deps<ElysQuery>) -> Result<GetAllPricesResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let mut pagination = PageRequest::new(20);
    let prices = querier.get_all_prices(&mut pagination)?;
    let resp: GetAllPricesResponse = GetAllPricesResponse { prices };

    Ok(resp)
}
