use cosmwasm_std::{coin, Addr};
use crate::types::{order::order::Order, order_type::OrderType};

impl Order {
    pub fn new_dummy() -> Order {
        Order {
            id: 777,
            order_type: OrderType::StopLoss,
            stop_price: coin(255, "eth"),
            user_token: coin(1000, "btc"),
            user_address: Addr::unchecked("user"),
        }
    }
}