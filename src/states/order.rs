use crate::types::Order;
use cw_storage_plus::Item;

pub const ORDER: Item<Vec<Order>> = Item::new("order");
