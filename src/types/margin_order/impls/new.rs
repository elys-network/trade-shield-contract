use cosmwasm_std::{Coin, Decimal};

use crate::types::{MarginOrder, MarginPosition};

impl MarginOrder {
    pub fn new(
        position: MarginPosition,
        creator: String,
        collateral: Coin,
        leverage: Decimal,
        borrow_token: Coin,
        order_vec: &Vec<MarginOrder>,
    ) -> Self {
        let order_id: u64 = match order_vec.iter().max_by_key(|s| s.order_id) {
            Some(x) => x.order_id + 1,
            None => 0,
        };

        Self {
            order_id,
            position,
            collateral,
            borrow_token,
            creator,
            leverage,
        }
    }
}
