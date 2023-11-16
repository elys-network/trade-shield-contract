use crate::types::{MarginPosition, SpotOrderPrice, SpotOrderType, SwapAmountInRoute};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

#[cw_serde]
pub enum ExecuteMsg {
    CreateSpotOrder {
        order_type: SpotOrderType,
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
        order_type: Option<SpotOrderType>,
    },

    ProcessSpotOrders {},

    CreateMarginOrder {
        position: MarginPosition,
        collateral: Coin,
        leverage: Decimal,
        borrow_asset: String,
        take_profit_price: Decimal,
    },

    CancelMarginOrder {
        order_id: u64,
    },
}
