use super::query_resp::GetAllPricesResp;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CustomQuery, PageRequest};

#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    #[returns(GetAllPricesResp)]
    GetAllPrices { page_request: PageRequest },
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn get_all_prices(page_request: PageRequest) -> Self {
        ElysQuery::GetAllPrices { page_request }
    }
}
