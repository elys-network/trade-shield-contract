use cosmwasm_schema::cw_serde;

use crate::types::MarginSpotOrderType;

#[cw_serde]
pub struct MarginSpotOrder {
    pub order_id: u64,
    pub order_type: MarginSpotOrderType,
}
