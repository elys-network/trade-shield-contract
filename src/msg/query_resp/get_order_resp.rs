use crate::types::Order;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetOrderResp {
    pub order: Order,
}

