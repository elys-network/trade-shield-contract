use crate::types::{order::order::Order, order_type::OrderType};
use cosmwasm_std::{Addr, Coin};

impl Order {
    pub fn new(
        order_type: OrderType,
        order_price: Coin,
        order_amount: Coin,
        owner_address: Addr,
        order_vec: &Vec<Order>,
    ) -> Order {
        let order_id: u128 = match order_vec.iter().max_by_key(|s| s.order_id) {
            Some(x) => x.order_id + 1,
            None => 0,
        };

        Order {
            order_type,
            order_price,
            order_amount,
            owner_address,
            order_id,
        }
    }
}
