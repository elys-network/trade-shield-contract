use crate::types::{order::order::Order, order_type::OrderType};
use cosmwasm_std::{Addr, Coin};

impl Order {
    pub fn new(
        order_type: OrderType,
        order_price: Coin,
        user_token: Coin,
        user_address: Addr,
        order_vec: &Vec<Order>,
    ) -> Order {
        let id: u128 = match order_vec.iter().max_by_key(|s| s.id) {
            Some(x) => x.id + 1,
            None => 0,
        };

        Order {
            order_type,
            order_price,
            user_token,
            user_address,
            id,
        }
    }
}
