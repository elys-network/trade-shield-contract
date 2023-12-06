use crate::types::SpotOrder;
use cw_storage_plus::Map;

pub const SPOT_ORDER: Map<u64, SpotOrder> = Map::new("spot order");
