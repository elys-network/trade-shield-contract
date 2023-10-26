use crate::types::MarginPosition;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

#[cw_serde]
pub struct MarginOrder {
    pub order_id: u64,
    pub position: MarginPosition,
    pub collateral: Coin,
    pub borrow_token: Coin,
    pub creator: String,
    pub leverage: Decimal,
}
