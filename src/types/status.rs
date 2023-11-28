use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum Status {
    NotProcessed,
    Processing,
    Processed,
    Canceled,
}
