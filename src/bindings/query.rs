use crate::types::{PageRequest, SwapAmountInRoute};

#[allow(unused_imports)]
use super::query_resp::*;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, CustomQuery};

#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    #[returns(AllPriceResponse)]
    PriceAll { pagination: PageRequest },
    #[returns(QuerySwapEstimationResponse)]
    QuerySwapEstimation {
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
    },
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn get_all_prices(pagination: PageRequest) -> Self {
        ElysQuery::PriceAll { pagination }
    }
    pub fn swap_estimation(routes: &Vec<SwapAmountInRoute>, token_in: &Coin) -> Self {
        Self::QuerySwapEstimation {
            routes: routes.to_owned(),
            token_in: token_in.to_owned(),
        }
    }
}
