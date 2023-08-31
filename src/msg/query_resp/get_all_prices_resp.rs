use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

#[cw_serde]
pub struct GetAllPricesResponse {
    pub prices: Vec<Coin>,
}
