use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct PageResponse {
    pub next_key: Option<Binary>,
    pub total: Option<u64>,
}

impl PageResponse {
    pub fn new(next_key: Option<Binary>, total: Option<u64>) -> Self {
        Self { next_key, total }
    }

    pub fn empty(have_total: bool) -> Self {
        Self {
            next_key: None,
            total: if have_total { Some(0) } else { None },
        }
    }
}
