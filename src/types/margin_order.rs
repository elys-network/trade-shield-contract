use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};
use elys_bindings::types::MarginPosition;

use super::{OrderPrice, OrderType};

#[cw_serde]
pub struct MarginOrder {
    pub order_id: u64,
    pub position: MarginPosition,
    pub collateral: Coin,
    pub borrow_asset: String,
    pub owner: String,
    pub leverage: Decimal,
    pub take_profit_price: Decimal,
    pub order_type: OrderType,
    pub trigger_price: OrderPrice,
}

impl MarginOrder {
    pub fn new(
        position: &MarginPosition,
        collateral: &Coin,
        borrow_asset: impl Into<String>,
        owner: impl Into<String>,
        leverage: &Decimal,
        take_profit_price: &Decimal,
        order_type: &OrderType,
        trigger_price: &Option<OrderPrice>,
        order_vec: &Vec<MarginOrder>,
    ) -> Self {
        let order_id: u64 = match order_vec.iter().max_by_key(|s| s.order_id) {
            Some(x) => x.order_id + 1,
            None => 0,
        };

        let trigger_price = match trigger_price {
            Some(trigger_price) => trigger_price.to_owned(),
            None => OrderPrice {
                base_denom: "".to_string(),
                quote_denom: "".to_string(),
                rate: Decimal::zero(),
            },
        };

        Self {
            order_id,
            position: position.to_owned(),
            collateral: collateral.to_owned(),
            borrow_asset: borrow_asset.into(),
            owner: owner.into(),
            leverage: leverage.to_owned(),
            take_profit_price: take_profit_price.to_owned(),
            order_type: order_type.to_owned(),
            trigger_price,
        }
    }
}
