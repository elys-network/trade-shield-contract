#[allow(unused_imports)]
use super::query_resp::*;
use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused_imports)]
use elys_bindings::query_resp::*;
use elys_bindings::types::PageRequest;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetSpotOrderResp)]
    GetSpotOrder { order_id: u64 },
    #[returns(GetAllPricesResponse)]
    GetAllPrices {},
    #[returns(OracleAssetInfoResponse)]
    AssetInfo { denom: String },
    #[returns(MTPResponse)]
    GetMarginOrder { address: String, id: u64 },
    #[returns(PositionsResponse)]
    GetMarginOrders { pagination: PageRequest },
}
