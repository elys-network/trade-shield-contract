use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct OrderPrice {
    pub base_denom: String,
    pub quote_denom: String,
    pub rate: Uint128,
}
