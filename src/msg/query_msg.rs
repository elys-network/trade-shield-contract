use cosmwasm_schema::{cw_serde,QueryResponses};
#[allow(unused_imports)]
use super::query_resp::GetOrderResp;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetOrderResp)]
    GetOrder { order_id: u128 },
}

