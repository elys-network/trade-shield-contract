use crate::types::*;
use cosmwasm_std::{Addr, Coin};

impl SpotOrder {
    pub fn new(
        order_type: OrderType,
        order_price: SpotOrderPrice,
        order_amount: Coin,
        owner_address: Addr,
        order_target_denom: String,
        order_amm_routes: Vec<SwapAmountInRoute>,
        order_vec: &Vec<SpotOrder>,
    ) -> SpotOrder {
        let order_id: u64 = match order_vec.iter().max_by_key(|s| s.order_id) {
            Some(x) => x.order_id + 1,
            None => 0,
        };

        SpotOrder {
            order_type,
            order_price,
            order_amount,
            owner_address,
            order_id,
            order_amm_routes,
            order_target_denom,
        }
    }
}
