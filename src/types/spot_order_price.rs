use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct OrderPrice {
    pub base_denom: String,
    pub quote_denom: String,
    pub rate: Decimal,
}
