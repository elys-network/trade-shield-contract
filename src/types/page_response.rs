use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct PageResponse {
    pub key: Option<Binary>,
    pub total: u32,
}
