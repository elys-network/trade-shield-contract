use cosmwasm_std::Coin;

pub trait OracleQuery {
    fn price_all(&self) -> Vec<Coin>;
}

pub struct ElysOracle;

impl OracleQuery for ElysOracle {
    fn price_all(&self) -> Vec<Coin> {
        vec![]
    }
}

pub struct MockOracleQuery {
    pub values: Vec<Coin>,
}

impl OracleQuery for MockOracleQuery {
    fn price_all(&self) -> Vec<Coin> {
        self.values.clone()
    }
}
