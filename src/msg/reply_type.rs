use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ReplyType {
    SpotOrder,
    MarginOpenPosition,
    MarginClosePosition,
}
