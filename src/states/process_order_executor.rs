use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const PROCESS_SPOT_ORDER_EXECUTOR: Item<Addr> = Item::new("process_order_executor");
