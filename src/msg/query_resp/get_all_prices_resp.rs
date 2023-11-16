use cosmwasm_schema::cw_serde;
use elys_bindings::types::Price;

#[cw_serde]
pub struct GetAllPricesResponse {
    pub prices: Vec<Price>,
}
