use crate::types::{OrderPrice, OrderType, SwapAmountInRoute};
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    CreateOrder {
        order_type: OrderType,
        order_source_denom: String,
        order_target_denom: String,
        order_price: OrderPrice,
        order_amm_routes: Vec<SwapAmountInRoute>,
    },
    CancelOrder {
        order_id: u128,
    },
    ProcessOrder {},
}
