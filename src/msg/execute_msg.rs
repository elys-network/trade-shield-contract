use crate::types::OrderType;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

#[cw_serde]
pub enum ExecuteMsg {
    CreateOrder {
        order_type: OrderType,
        order_price: Coin,
    },
    CancelOrder {
        order_id: u128,
    },
    ProcessOrder {},
}
