use cosmwasm_schema::cw_serde;

use crate::types::{page_response::PageResponse, Price};

#[cw_serde]
pub struct AllPriceResponse {
    pub price: Vec<Price>,
    pub pagination: PageResponse,
}
