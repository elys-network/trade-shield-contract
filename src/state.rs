use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::Item;

#[cw_serde]
pub enum OrderType {
    StopLoss,
    TakeProfit,
}

#[cw_serde]
pub struct Order {
    pub order_type: OrderType,
    pub id: String,
    pub stop_price: u128,
    pub user_token: Coin,
    pub user: Addr,
    pub selling_denom: String,
}

impl Order {
    pub fn new_dummy() -> Order {
        Order {
            id: "id".to_owned(),
            order_type: OrderType::StopLoss,
            stop_price: 255,
            user_token: Coin {
                denom: "btc".to_owned(),
                amount: Uint128::new(1000),
            },
            user: Addr::unchecked("user"),
            selling_denom: "eth".to_owned(),
        }
    }

    pub fn new(
        order_type: OrderType,
        stop_price: u128,
        user_token: Coin,
        user: Addr,
        selling_denom: String,
    ) -> Order {
        use sha2::{Digest, Sha256};
        use std::time::{SystemTime, UNIX_EPOCH};

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let input = format!(
            "{}-{:?}-{}-{}-{}-{}",
            current_time.as_millis(),
            &order_type,
            &stop_price,
            &user_token,
            &user,
            &selling_denom
        );

        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();

        let id = format!("{:X}", result);

        Order {
            order_type,
            stop_price,
            user_token,
            selling_denom,
            user,
            id,
        }
    }
}

pub const ORDER: Item<Vec<Order>> = Item::new("order");
