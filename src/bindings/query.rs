use crate::types::PageRequest;

use super::query_resp::AllPriceResponse;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    #[returns(AllPriceResponse)]
    PriceAll { pagination: PageRequest },
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn get_all_prices(pagination: PageRequest) -> Self {
        ElysQuery::PriceAll { pagination }
    }
}
