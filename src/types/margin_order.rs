use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, StdResult};
use elys_bindings::types::MarginPosition;

use super::{MarginOrderType, OrderPrice, Status};

#[cw_serde]
pub struct MarginOrder {
    pub order_id: u64,
    pub owner: String,
    pub order_type: MarginOrderType,
    pub position: MarginPosition,
    pub trigger_price: Option<OrderPrice>,
    pub collateral: Coin,
    pub trading_asset: String,
    pub leverage: Decimal,
    pub take_profit_price: Decimal,
    pub position_id: Option<u64>,
    pub status: Status,
}

impl MarginOrder {
    pub fn new_open(
        owner: impl Into<String>,
        position: &MarginPosition,
        order_type: &MarginOrderType,
        collateral: &Coin,
        trading_asset: impl Into<String>,
        leverage: &Decimal,
        take_profit_price: &Decimal,
        trigger_price: &Option<OrderPrice>,
        order_vec: &Vec<MarginOrder>,
    ) -> Self {
        let order_id: u64 = match order_vec.iter().max_by_key(|s| s.order_id) {
            Some(x) => x.order_id + 1,
            None => 0,
        };

        let status = Status::Pending;

        Self {
            order_id,
            owner: owner.into(),
            position: position.to_owned(),
            collateral: collateral.to_owned(),
            trading_asset: trading_asset.into(),
            leverage: leverage.to_owned(),
            take_profit_price: take_profit_price.to_owned(),
            order_type: order_type.to_owned(),
            trigger_price: trigger_price.to_owned(),
            status,
            position_id: None,
        }
    }
    pub fn new_close(
        owner: impl Into<String>,
        position: i32,
        order_type: &MarginOrderType,
        collateral: &Coin,
        trading_asset: impl Into<String>,
        leverage: &Decimal,
        position_id: u64,
        trigger_price: &Option<OrderPrice>,
        take_profit_price: &Decimal,
        order_vec: &Vec<MarginOrder>,
    ) -> StdResult<Self> {
        let order_id: u64 = match order_vec.iter().max_by_key(|s| s.order_id) {
            Some(x) => x.order_id + 1,
            None => 0,
        };

        let status = Status::Pending;

        let position = MarginPosition::try_from_i32(position)?;

        Ok(Self {
            order_id,
            status,
            order_type: order_type.to_owned(),
            position,
            owner: owner.into(),
            trigger_price: trigger_price.to_owned(),
            collateral: collateral.to_owned(),
            trading_asset: trading_asset.into(),
            position_id: Some(position_id),
            leverage: leverage.to_owned(),
            take_profit_price: take_profit_price.to_owned(),
        })
    }
}
