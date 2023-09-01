use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct PageRequest {
    key: Option<Binary>,
    offset: Option<u64>,
    limit: u64,
    count_total: bool,
    reverse: bool,
}

impl PageRequest {
    pub fn new(limit: u64) -> Self {
        Self {
            key: None,
            limit,
            offset: None,
            count_total: false,
            reverse: false,
        }
    }

    pub fn update(&mut self, key: Option<Binary>) -> () {
        self.key = key;
    }
}
