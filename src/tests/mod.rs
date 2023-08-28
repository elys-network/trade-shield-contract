use crate::{
    entry_point::*,
    msg::*,
    types::{Order, OrderType},
    ContractError,
};
use cosmwasm_std::{coin, coins, Addr, Event};
use cw_multi_test::{ContractWrapper, Executor};

mod get_order_id_from_events;

mod create_order {
    use super::*;
    mod coin_number;
    mod not_enough_fund;
    mod successful_create_order;
}

mod cancel_order {
    use super::*;
    mod not_found;
    mod successful_cancel_order_with_created_order;
    mod unauthorized;

    mod successful_cancel_order_with_dummy_order;
}

mod get_order {
    use super::*;
    use cosmwasm_std::{Binary, StdError};
    mod not_found;
    mod successful_query_message;
}

mod process_order {
    use super::*;
    mod successful_process_5_of_10_orders;
    mod successful_process_stop_loss_order;
    mod successful_process_take_profit_order;
    mod unauthorize;
}

mod mock {
    pub mod multitest;
    mod test;
}
