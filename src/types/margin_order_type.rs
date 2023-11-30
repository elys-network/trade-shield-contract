use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum MarginOrderType {
    LimitOpen,
    LimitClose,

    MarketOpen,
    MarketClose,

    StopLoss,
}
