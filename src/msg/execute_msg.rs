use crate::types::{MarginOrderType, MarginPosition, OrderPrice, SpotOrderType};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Int128, Uint128};
use elys_bindings::types::EarnType;

#[cw_serde]
pub enum ExecuteMsg {
    CreateSpotOrder {
        order_type: SpotOrderType,
        order_source_denom: String,
        order_target_denom: String,
        order_price: Option<OrderPrice>,
    },
    CancelSpotOrder {
        order_id: u64,
    },

    CancelSpotOrders {
        order_ids: Option<Vec<u64>>,
        owner_address: String,
        order_type: Option<SpotOrderType>,
    },
    CreateMarginOrder {
        position: Option<MarginPosition>, // Can be null if it's not a LimitOpen or MarketOpen type
        leverage: Option<Decimal>,        // Can be null if it's not a LimitOpen or MarketOpen type
        trading_asset: Option<String>,    // Can be null if it's not a LimitOpen or MarketOpen type
        take_profit_price: Option<Decimal>, // Can be null if it's not a LimitOpen or MarketOpen type
        order_type: MarginOrderType,
        trigger_price: Option<OrderPrice>, // Can be null if it's a MarketOpen or MarketClose type
        position_id: Option<u64>, // Can be null if it's not a LimitClose, MarketClose or StopLoss type
    },
    CancelMarginOrder {
        order_id: u64,
    },

    CancelMarginOrders {
        order_ids: Option<Vec<u64>>,
        owner_address: String,
        order_type: Option<MarginOrderType>,
    },
    CloseMarginPosition {
        id: u64,
        amount: Int128,
    },
    StakeRequest {
        amount: u64,
        asset: String,
        validator_address: Option<String>,
    },
    UnstakeRequest {
        amount: u64,
        asset: String,
        validator_address: Option<String>,
    },
    ElysRedelegateRequest {
        validator_src_address: String,
        validator_dst_address: String,
        amount: Coin,
    },
    ElysCancelUnstakeRequest {
        validator_address: String,
        // amount is always less than or equal to unbonding delegation entry balance
        amount: Coin,
        // creation_height is the height which the unbonding took place.
        creation_height: i64,
    },
    EdenVestRequest {
        amount: u64,
    },
    EdenCancelVestRequest {
        amount: u64,
    },
    ClaimRewardsRequest {
        withdraw_type: EarnType,
    },
    ClaimValidatorCommissionRequest {
        validator_address: String,
    },
    AmmJoinPoolRequest {
        pool_id: u64,
        max_amounts_in: Vec<Coin>,
        share_amount_out: Uint128,
        no_remaining: bool,
    },
    AmmExitPoolRequest {
        pool_id: u64,
        min_amounts_out: Vec<Coin>,
        share_amount_in: Uint128,
        token_out_denom: String,
    }
}
