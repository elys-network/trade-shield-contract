use crate::types::{order_type::OrderType, OrderPrice, Status};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct SpotOrder {
    pub order_type: OrderType,
    pub order_id: u64,
    pub order_price: OrderPrice,
    pub order_amount: Coin,
    pub owner_address: Addr,
    pub order_target_denom: String,
    pub status: Status,
}
