use cosmwasm_schema::cw_serde;

use crate::types::{page_response::PageResponse, Price};

#[cw_serde]
pub struct GetAllPricesResp {
    pub prices: Vec<Price>,
    pub page_response: PageResponse,
}
