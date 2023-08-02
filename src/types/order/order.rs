use crate::types::order_type::OrderType;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct Order {
    pub order_type: OrderType,
    pub order_id: u128,
    pub order_price: Coin,
    pub order_amount: Coin,
    pub owner_address: Addr,
}
