use crate::types::{order::order::Order, order_type::OrderType};
use cosmwasm_std::{Addr, Coin};

impl Order {
    pub fn new(
        order_type: OrderType,
        stop_price: Coin,
        user_token: Coin,
        user_address: Addr,
        nuber_of_order: u128,
    ) -> Order {
        let id = nuber_of_order + 1;
        Order {
            order_type,
            stop_price,
            user_token,
            user_address,
            id,
        }
    }
}
