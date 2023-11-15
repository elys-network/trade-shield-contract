use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use elys_bindings::types::Price;

#[cw_serde]
pub struct GetAllPricesResponse {
    pub prices: Vec<Price>,
}
