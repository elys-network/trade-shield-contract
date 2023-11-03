use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdError;

#[cw_serde]
pub enum MarginPosition {
    Unspecified = 0,
    Long = 1,
    Short = 2,
}

impl MarginPosition {
    pub fn try_from_i32(value: i32) -> Result<Self, StdError> {
        match value {
            0 => Ok(Self::Unspecified),
            1 => Ok(Self::Long),
            2 => Ok(Self::Short),
            _ => Err(StdError::generic_err("MarginPosition out of range")),
        }
    }
}
