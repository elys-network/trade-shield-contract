#[allow(unused_imports)]
use super::query_resp::*;
use crate::types::{MarginOrderType, SpotOrderType};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
#[allow(unused_imports)]
use elys_bindings::query_resp::*;
use elys_bindings::types::PageRequest;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetSpotOrderResp)]
    GetSpotOrder { order_id: u64 },
    #[returns(GetAllPricesResponse)]
    GetAllPrices { limit: u64 },
    #[returns(OracleAssetInfoResponse)]
    AssetInfo { denom: String },
    #[returns(GetMarginOrderResp)]
    GetMarginOrder { id: u64 },
    #[returns(GetSpotOrdersResp)]
    GetSpotOrders {
        pagination: PageRequest,
        order_owner: Option<String>,
        order_type: Option<SpotOrderType>,
    },
    #[returns(GetMarginOrdersResp)]
    GetMarginOrders {
        pagination: PageRequest,
        order_owner: Option<String>,
        order_type: Option<MarginOrderType>,
    },
    #[returns(AmmSwapEstimationByDenomResponse)]
    SwapEstimationByDenom {
        amount: Coin,
        denom_in: String,
        denom_out: String,
    },
    #[returns(MarginMtpResponse)]
    GetMarginPosition { id: u64, address: String },
    #[returns(MarginQueryPositionsResponse)]
    GetMarginPositions { pagination: PageRequest },
}
