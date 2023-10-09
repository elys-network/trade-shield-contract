use crate::types::{order_type::OrderType, swap_route::SwapAmountInRoute, OrderPricePair};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct Order {
    pub order_type: OrderType,
    pub order_id: u128,
    pub order_price_pair: OrderPricePair,
    pub order_amount: Coin,
    pub owner_address: Addr,
    pub order_target_denom: String,
    pub order_amm_routes: Vec<SwapAmountInRoute>,
}
