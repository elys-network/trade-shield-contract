use crate::types::{OrderType, SwapAmountInRoute};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

#[cw_serde]
pub enum ExecuteMsg {
    CreateOrder {
        order_type: OrderType,
        order_price: Coin,
        order_amm_routes: Vec<SwapAmountInRoute>,
    },
    CancelOrder {
        order_id: u128,
    },
    ProcessOrder {},
}
