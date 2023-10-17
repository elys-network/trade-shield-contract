use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

use crate::types::{PageResponse, Price};

#[cw_serde]
pub struct AllPriceResponse {
    pub price: Vec<Price>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct QuerySwapEstimationResponse {
    pub spot_price: Decimal,
    pub token_out: Coin,
}
