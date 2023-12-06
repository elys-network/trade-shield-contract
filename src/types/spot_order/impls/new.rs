use crate::types::*;
use cosmwasm_std::{Addr, BlockInfo, Coin, Decimal};

impl SpotOrder {
    pub fn new(
        order_type: SpotOrderType,
        order_price: Option<OrderPrice>,
        order_amount: Coin,
        owner_address: Addr,
        order_target_denom: String,
        order_vec: &Vec<SpotOrder>,
        block_info: &BlockInfo,
    ) -> SpotOrder {
        let order_id: u64 = match order_vec.iter().max_by_key(|s| s.order_id) {
            Some(x) => x.order_id + 1,
            None => 0,
        };

        let order_price = match order_price {
            Some(order_price) => order_price,
            None => OrderPrice {
                base_denom: "".to_owned(),
                quote_denom: "".to_owned(),
                rate: Decimal::zero(),
            },
        };

        let status = Status::Pending;

        SpotOrder {
            order_type,
            order_price,
            order_amount,
            owner_address,
            order_id,
            order_target_denom,
            status,
            date: Date::from(block_info),
        }
    }
}
