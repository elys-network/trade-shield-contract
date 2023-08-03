use cosmwasm_std::Coin;
use cw_storage_plus::Item;

pub const PRICES: Item<Vec<Coin>> = Item::new("prices");
