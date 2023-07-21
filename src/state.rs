use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

#[cw_serde]
pub enum OrderType {
    StopLoss = "stop_loss",
    TakeProfit = "take_profit",
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

    fn new_dummy() -> Order {
        Order {
            id : "id".to_owned(),
            order_type: OrderType::StopLoss,
            stop_price : 255,
            user_token : Coin {
                denom:"btc".to_owned(),
                amount:100
            },
            user : Addr::unchecked("user"),
            selling_denom: "eth".to_owned()
        }
    }
}

pub const ORDER: Item<Vec<Order>> = Item::new("order");
