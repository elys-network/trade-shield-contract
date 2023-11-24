use crate::types::{MarginPosition, OrderPrice, OrderType, SwapAmountInRoute};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

#[cw_serde]
pub enum ExecuteMsg {
    CreateSpotOrder {
        order_type: OrderType,
        order_source_denom: String,
        order_target_denom: String,
        order_price: OrderPrice,
        order_amm_routes: Option<Vec<SwapAmountInRoute>>,
    },
    CancelSpotOrder {
        order_id: u64,
    },

    CancelSpotOrders {
        order_ids: Option<Vec<u64>>,
        owner_address: String,
        order_type: Option<OrderType>,
    },
    CreateMarginOrder {
        position: MarginPosition,
        collateral: Coin,
        leverage: Decimal,
        borrow_asset: String,
        take_profit_price: Decimal,
        order_type: OrderType,
        trigger_price: OrderPrice,
    },

    CancelMarginOrder {
        order_id: u64,
    },
}
