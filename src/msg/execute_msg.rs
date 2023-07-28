use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use crate::types::OrderType;

#[cw_serde]
pub enum ExecuteMsg {
    CreateOrder {
        order_type: OrderType,
        stop_price: Coin,
    },
    CancelOrder {
        order_id: u128,
    },
    ExecuteOrder {},
}
