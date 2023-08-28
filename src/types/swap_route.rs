use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct SwapAmountInRoute {
    pool_id: u64,
    token_out_denom: String,
}
