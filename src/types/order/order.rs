use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Addr};
use crate::types::order_type::OrderType;


#[cw_serde]
pub struct Order {
    pub order_type: OrderType,
    pub id: u128,
    pub order_price: Coin,
    pub user_token: Coin,
    pub user_address: Addr,
}