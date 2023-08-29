use crate::types::{order::order::Order, order_type::OrderType};
use cosmwasm_std::{coin, Addr};

impl Order {
    pub fn new_dummy() -> Order {
        Order {
            order_id: 777,
            order_type: OrderType::StopLoss,
            order_price: coin(5, "eth"),
            order_amount: coin(1000, "btc"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![],
        }
    }
}
