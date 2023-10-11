use cosmwasm_schema::cw_serde;

use crate::types::{PageResponse, Price};

#[cw_serde]
pub struct AllPriceResponse {
    pub price: Vec<Price>,
    pub pagination: PageResponse,
}
