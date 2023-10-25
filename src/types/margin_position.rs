use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum MarginPosition {
    Unspecified,
    Long,
    Short,
}
