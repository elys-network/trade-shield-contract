use crate::types::{PageResponse, SpotOrder};
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetSpotOrdersResp {
    pub page_response: PageResponse,
    pub orders: Vec<SpotOrder>,
}

impl GetSpotOrdersResp {
    pub fn empty(have_total: bool) -> Self {
        Self {
            page_response: PageResponse::empty(have_total),
            orders: vec![],
        }
    }
}
