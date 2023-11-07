#[allow(unused_imports)]
use super::query_resp::*;
use crate::{
    bindings::query_resp::AssetInfoResponse,
    types::{PageRequest, SpotOrderType},
};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetSpotOrderResp)]
    GetSpotOrder { order_id: u64 },
    #[returns(GetAllPricesResponse)]
    GetAllPrices {},
    #[returns(AssetInfoResponse)]
    AssetInfo { denom: String },
    #[returns(GetSpotOrdersResp)]
    GetSpotOrders {
        pagination: PageRequest,
        order_owner: Option<String>,
        order_type: Option<SpotOrderType>,
    },
}
