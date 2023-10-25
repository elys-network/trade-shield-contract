#[allow(unused_imports)]
use super::query_resp::*;
use crate::bindings::query_resp::AssetInfoResponse;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetOrderResp)]
    GetOrder { order_id: u64 },
    #[returns(GetAllPricesResponse)]
    GetAllPrices {},
    #[returns(AssetInfoResponse)]
    AssetInfo { denom: String },
}
