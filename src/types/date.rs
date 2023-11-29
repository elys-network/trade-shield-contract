use cosmwasm_schema::cw_serde;
use cosmwasm_std::{BlockInfo, Timestamp};

#[cw_serde]
pub struct Date {
    pub height: u64,
    pub time: Timestamp,
}

impl From<&BlockInfo> for Date {
    fn from(value: &BlockInfo) -> Self {
        Self {
            height: value.height.clone(),
            time: value.time.clone(),
        }
    }
}
