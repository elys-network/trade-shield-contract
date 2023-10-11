use crate::types::{order::order::Order, order_type::OrderType, OrderPrice};
use cosmwasm_std::{coin, Addr, Uint128};

impl Order {
    pub fn new_dummy() -> Order {
        Order {
            order_id: 777,
            order_type: OrderType::StopLoss,
            order_amount: coin(1000, "btc"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![],
            order_price: OrderPrice {
                quote_denom: "eth".to_string(),
                base_denom: "btc".to_string(),
                rate: Uint128::new(5),
            },
            order_target_denom: "eth".to_string(),
        }
    }
}
