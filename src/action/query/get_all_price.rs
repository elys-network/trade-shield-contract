use super::*;
use crate::{
    bindings::{querier::ElysQuerier, query::ElysQuery},
    msg::query_resp::GetAllPricesResp,
};

pub fn get_all_prices(deps: Deps<ElysQuery>) -> Result<GetAllPricesResp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let mut page_request = PageRequest::new(20);
    let prices = querier.get_all_prices(&mut page_request)?;
    let resp = GetAllPricesResp { prices };

    Ok(resp)
}
