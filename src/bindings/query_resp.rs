use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

use crate::types::{AssetInfo, PageResponse, Price};

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

#[cw_serde]
pub struct AssetInfoResponse {
    pub asset_info: AssetInfo,
}
