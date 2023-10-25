use crate::types::MarginOrder;
use cw_storage_plus::Item;

pub const ORDER: Item<Vec<MarginOrder>> = Item::new("order");
