use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct Price {
    pub asset: String,
    pub price: Decimal,
    pub source: String,
    pub provider: String,
    pub timestamp: u64,
}
