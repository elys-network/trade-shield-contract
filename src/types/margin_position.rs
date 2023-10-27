use cosmwasm_schema::cw_serde;

#[cw_serde]
#[repr(i32)]
pub enum MarginPosition {
    Unspecified = 0,
    Long = 1,
    Short = 2,
}
