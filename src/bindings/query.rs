use super::query_resp::GetAllPricesResp;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    #[returns(GetAllPricesResp)]
    GetAllPrices {},
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn get_all_prices() -> Self {
        ElysQuery::GetAllPrices {}
    }
}
