use cosmwasm_std::{coin, Coin, QuerierWrapper, QueryRequest, StdResult};

use crate::{bindings::query_resp::GetAllPricesResp, types::PageRequest};

use super::query::ElysQuery;

pub struct ElysQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ElysQuery>,
}

impl<'a> ElysQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, ElysQuery>) -> Self {
        ElysQuerier { querier }
    }
    pub fn get_all_prices(&self, page_request: &mut PageRequest) -> StdResult<Vec<Coin>> {
        let prices_query = ElysQuery::GetAllPrices {
            page_request: page_request.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = ElysQuery::into(prices_query);
        let resp: GetAllPricesResp = self.querier.query(&request)?;
        page_request.update(resp.page_response.key);
        let result: Vec<Coin> = resp
            .prices
            .iter()
            .map(|price| coin(price.price.atomics().u128(), &price.asset))
            .collect();
        Ok(result)
    }
}
