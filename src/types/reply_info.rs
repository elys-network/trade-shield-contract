use crate::msg::ReplyType;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct ReplyInfo {
    pub id: u64,
    pub reply_type: ReplyType,
    pub data: Option<Binary>,
}
