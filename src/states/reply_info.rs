use crate::types::ReplyInfo;
use cw_storage_plus::Item;

pub const REPLY_INFO: Item<Vec<ReplyInfo>> = Item::new("reply_info");
