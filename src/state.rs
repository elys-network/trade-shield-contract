use cw_storage_plus::Item;
use crate::types::Order;

pub const ORDER: Item<Vec<Order>> = Item::new("order");
