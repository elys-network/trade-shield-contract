use crate::types::{PageResponse, SpotOrder};
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetSpotOrdersResp {
    pub pagination: PageResponse,
    pub orders: Vec<SpotOrder>,
}

impl GetSpotOrdersResp {
    pub fn empty(have_total: bool) -> Self {
        Self {
            pagination: PageResponse::empty(have_total),
            orders: vec![],
        }
    }
}
