use crate::types::{OrderPricePair, OrderType, SwapAmountInRoute};
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    CreateOrder {
        order_type: OrderType,
        order_source_denom: String,
        order_target_denom: String,
        order_price_pair: OrderPricePair,
        order_amm_routes: Vec<SwapAmountInRoute>,
    },
    CancelOrder {
        order_id: u128,
    },
    ProcessOrder {},
}
