use crate::types::*;
use cosmwasm_std::{coin, Addr, Decimal, Uint128};

impl SpotOrder {
    pub fn new_dummy() -> SpotOrder {
        SpotOrder {
            order_id: 777,
            order_type: OrderType::StopLoss,
            order_amount: coin(1000, "btc"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                quote_denom: "eth".to_string(),
                base_denom: "btc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(5), 0).unwrap(),
            },
            order_target_denom: "eth".to_string(),
            status: Status::NotProcessed,
        }
    }
}
