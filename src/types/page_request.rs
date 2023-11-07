use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct PageRequest {
    pub key: Option<Binary>,
    pub offset: Option<u64>,
    pub limit: u64,
    pub count_total: bool,
    pub reverse: bool,
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
