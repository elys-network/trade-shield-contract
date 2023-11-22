use crate::types::{MarginPosition, OrderType, SpotOrderPrice, SwapAmountInRoute};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

#[cw_serde]
pub enum ExecuteMsg {
    CreateSpotOrder {
        order_type: OrderType,
        order_source_denom: String,
        order_target_denom: String,
        order_price: SpotOrderPrice,
        order_amm_routes: Vec<SwapAmountInRoute>,
    },
    CancelSpotOrder {
        order_id: u64,
    },

    CancelSpotOrders {
        order_ids: Option<Vec<u64>>,
        owner_address: String,
        order_type: Option<OrderType>,
    },

    ProcessSpotOrders {},

    CreateMarginOrder {
        position: MarginPosition,
        collateral: Coin,
        leverage: Decimal,
        borrow_asset: String,
        take_profit_price: Decimal,
        order_type: OrderType,
    },

    CancelMarginOrder {
        order_id: u64,
    },
}
