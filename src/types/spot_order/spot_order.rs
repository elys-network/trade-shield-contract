use crate::types::{spot_order_type::SpotOrderType, OrderPrice, Status};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct SpotOrder {
    pub order_type: SpotOrderType,
    pub order_id: u64,
    pub order_price: OrderPrice,
    pub order_amount: Coin,
    pub owner_address: Addr,
    pub order_target_denom: String,
    pub status: Status,
}
