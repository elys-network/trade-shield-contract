use cosmwasm_std::{Coin, Decimal};

use crate::types::{MarginOrder, MarginPosition};

impl MarginOrder {
    pub fn new(
        position: MarginPosition,
        creator: impl Into<String>,
        collateral: Coin,
        leverage: Decimal,
        borrow_token: Coin,
        take_profit_price: Decimal,
    ) -> Self {
        let order_id: u64 = 0;

        Self {
            order_id,
            position,
            collateral,
            borrow_token,
            creator: creator.into(),
            leverage,
            take_profit_price,
        }
    }
}
