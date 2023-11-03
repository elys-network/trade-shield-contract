use crate::types::SpotOrder;
use cw_storage_plus::Item;

pub const SPOT_ORDER: Item<Vec<SpotOrder>> = Item::new("spot order");
