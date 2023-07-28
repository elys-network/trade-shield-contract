use cosmwasm_schema::cw_serde;
use crate::types::Order;

#[cw_serde]
pub struct InstantiateMsg {
    pub orders : Vec<Order>
}
