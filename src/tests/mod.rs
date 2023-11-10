use crate::{
    entry_point::{execute, query, reply},
    msg::*,
    types::*,
    ContractError,
};

use cosmwasm_std::{coin, coins, Addr, Decimal, Event, Uint128};
use cw_multi_test::ContractWrapper;
mod get_order_id_from_events;
mod read_processed_order_id;
use cw_multi_test::Executor;
use elys_bindings_test::*;

mod create_spot_order {
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

mod cancel_spot_order {
    use super::*;
    mod not_found;
    mod process_spot_order_processing;
    mod successful_cancel_order_with_created_order;
    mod successful_cancel_order_with_dummy_order;
    mod unauthorized;
}

mod cancel_spot_orders {
    use super::*;
    mod successfully_cancel_orders;
    mod successfully_cancel_orders_id;
    mod successfully_cancel_orders_type;
    mod unauthorize;
}

mod get_spot_order {
    use super::*;
    use cosmwasm_std::{Binary, StdError};
    mod not_found;
    mod successful_query_message;
}

mod process_spot_order {
    use super::*;
    mod succesful_process_limit_buy_order;
    mod successful_process_5_of_10_orders;
    mod successful_process_limit_sell_order;
    mod successful_process_stop_loss_order;
    mod unauthorize;
}

mod create_margin_order {
    use super::*;
    mod coin_number;
    mod collateral_amount;
    mod successful_create_long_order;
    mod succsessful_create_short_order;
}

pub use mock::instantiate::*;
mod mock {
    pub mod instantiate;
}
