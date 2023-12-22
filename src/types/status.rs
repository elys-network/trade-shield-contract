use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum Status {
    Pending,
    Executed,
    Canceled,
    Error(String),
}

impl Status {
    pub fn error(msg: impl Into<String>) -> Self {
        Self::Error(msg.into())
    }
}
