use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct AssetInfo {
    pub denom: String,
    pub display: String,
    pub band_ticker: String,
    pub elys_ticker: String,
    pub decimal: u64,
}

#[cfg(test)]
impl AssetInfo {
    pub fn new(
        denom: String,
        display: String,
        band_ticker: String,
        elys_ticker: String,
        decimal: u64,
    ) -> Self {
        Self {
            denom,
            display,
            band_ticker,
            elys_ticker,
            decimal,
        }
    }

    pub fn default(denom: String, decimal: u64) -> Self {
        Self {
            denom,
            display: "".to_string(),
            band_ticker: "".to_string(),
            elys_ticker: "".to_string(),
            decimal,
        }
    }
}
