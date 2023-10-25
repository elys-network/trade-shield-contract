use cosmwasm_std::BankMsg;
use cw_storage_plus::Item;

pub const PROCESSED_SPOT_ORDER: Item<Vec<(u64, BankMsg)>> = Item::new("processed order");
