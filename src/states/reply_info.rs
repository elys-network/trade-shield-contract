use crate::types::ReplyInfo;
use cw_storage_plus::{Item, Map};

pub const REPLY_INFO: Map<u64, ReplyInfo> = Map::new("reply_info");

pub const MAX_REPLY_ID: Item<u64> = Item::new("reply_info_id");
