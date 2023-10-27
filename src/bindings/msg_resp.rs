use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Int64};

#[cw_serde]
pub struct MsgSwapExactAmountInResp {
    token_out_amount: Int64,
    pub meta_data: Option<Binary>,
}

#[cw_serde]
pub struct MsgOpenResponse {
    meta_data: Option<Binary>,
}

#[cw_serde]
pub struct MsgCloseResponse {
    meta_data: Option<Binary>,
}

impl MsgSwapExactAmountInResp {
    pub fn token_out_amount(&self) -> u128 {
        self.token_out_amount.i64().clone() as u128
    }
    #[cfg(test)]
    pub fn new(token_out_amount: i64, meta_data: Option<Binary>) -> Self {
        Self {
            token_out_amount: Int64::new(token_out_amount),
            meta_data,
        }
    }
}
