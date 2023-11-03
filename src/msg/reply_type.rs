use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdError, StdResult};

#[cw_serde]
pub enum ReplyType {
    SpotOrder = 0,
    MarginOpenPosition = 1,
    MarginClosePosition = 2,
}

impl ReplyType {
    pub fn from(value: u64) -> StdResult<Self> {
        match value {
            0 => Ok(Self::SpotOrder),
            1 => Ok(Self::MarginOpenPosition),
            2 => Ok(Self::MarginClosePosition),
            _ => Err(StdError::generic_err("reply out of range")),
        }
    }
}
