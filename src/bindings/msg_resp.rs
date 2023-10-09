use cosmwasm_schema::cw_serde;
use cosmwasm_std::Int64;

#[cw_serde]
pub struct MsgSwapExactAmountInResp {
    token_out_amount: Int64,
}
