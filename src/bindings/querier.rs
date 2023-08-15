use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use super::{query::ElysQuery, query_resp::GetAllPricesResp};

pub struct ElysQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ElysQuery>,
}

impl<'a> ElysQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, ElysQuery>) -> Self {
        ElysQuerier { querier }
    }
    pub fn get_all_prices(&self) -> StdResult<GetAllPricesResp> {
        let prices_query = ElysQuery::GetAllPrices {};
        let request: QueryRequest<ElysQuery> = ElysQuery::into(prices_query);
        self.querier.query(&request)
    }
}
