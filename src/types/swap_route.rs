use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct SwapAmountInRoute {
    pool_id: u64,
    token_out_denom: String,
}

#[cfg(test)]
impl SwapAmountInRoute {
    pub fn pool_id(&self) -> u64 {
        self.pool_id
    }
    pub fn token_out_denom(&self) -> String {
        self.token_out_denom.clone()
    }
    pub fn new(pool_id: u64, token_out_denom: &str) -> Self {
        Self {
            pool_id,
            token_out_denom: token_out_denom.to_string(),
        }
    }
}
