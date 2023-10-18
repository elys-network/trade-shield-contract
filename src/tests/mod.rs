use crate::{
    entry_point::{execute, query, reply},
    msg::*,
    types::{SpotOrder, SpotOrderPrice, SpotOrderType},
    ContractError,
};

use cosmwasm_std::{coin, coins, Addr, Decimal, Event, Uint128};
use cw_multi_test::{ContractWrapper, Executor};
use mock::multitest::ElysApp;

mod get_order_id_from_events;
mod create_order {
    use super::*;
    mod coin_number;
    mod not_enough_fund;
    mod order_price_denom;
    mod order_same_denom;
    mod order_wrong_fund;
    mod successful_create_limit_buy_order;
    mod successful_create_limit_sell_order;
    mod successful_create_stop_loss_order;
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
    mod succesful_process_limit_buy_order;
    mod successful_process_5_of_10_orders;
    mod successful_process_limit_sell_order;
    mod successful_process_stop_loss_order;
    mod unauthorize;
}

pub use mock::instantiate::*;
mod mock {
    pub mod instantiate;
    #[allow(dead_code, unused)]
    pub mod multitest;
    mod test;
}
