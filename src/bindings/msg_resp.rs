use cosmwasm_schema::cw_serde;
use cosmwasm_std::Int64;

#[cw_serde]
pub struct MsgSwapExactAmountInResp {
    token_out_amount: Int64,
}
impl MsgSwapExactAmountInResp {
    pub fn to_uint128(&self) -> u128 {
        self.token_out_amount.i64() as u128
    }
}
