use cosmwasm_std::{coin, Coin, QuerierWrapper, QueryRequest, StdResult};

use crate::{
    bindings::query_resp::AllPriceResponse,
    types::{PageRequest, SwapAmountInRoute},
};

use super::{query::ElysQuery, query_resp::QuerySwapEstimationResponse};

pub struct ElysQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ElysQuery>,
}

impl<'a> ElysQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, ElysQuery>) -> Self {
        ElysQuerier { querier }
    }
    pub fn get_all_prices(&self, pagination: &mut PageRequest) -> StdResult<Vec<Coin>> {
        let prices_query = ElysQuery::PriceAll {
            pagination: pagination.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(prices_query);
        let resp: AllPriceResponse = self.querier.query(&request)?;
        pagination.update(resp.pagination.next_key);
        let result: Vec<Coin> = resp
            .price
            .iter()
            .map(|price| coin(price.price.atomics().u128(), &price.asset))
            .collect();
        Ok(result)
    }
    pub fn swap_estimation(
        &self,
        routes: &Vec<SwapAmountInRoute>,
        token_in: &Coin,
    ) -> StdResult<QuerySwapEstimationResponse> {
        let request = QueryRequest::Custom(ElysQuery::swap_estimation(routes, token_in));
        let resp: QuerySwapEstimationResponse = self.querier.query(&request)?;
        Ok(resp)
    }
}
