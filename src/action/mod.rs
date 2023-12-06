use crate::{states::*, types::*, ContractError};
use cosmwasm_std::Event;
use cosmwasm_std::{BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Order, Response};
use elys_bindings::*;

pub mod query {
    mod asset_info;
    mod get_all_price;
    mod get_margin_order;
    mod get_margin_orders;
    mod get_margin_position;
    mod get_margin_positions;
    mod get_spot_order;
    mod get_spot_orders;
    mod swap_estimation_by_denom;

    use super::*;

    use crate::msg::query_resp::*;
    use cosmwasm_std::Deps;
    use elys_bindings::query_resp::*;

    pub use asset_info::asset_info;
    pub use get_all_price::get_all_prices;
    pub use get_margin_order::get_margin_order;
    pub use get_margin_orders::get_margin_orders;
    pub use get_margin_position::get_margin_position;
    pub use get_margin_positions::get_margin_positions;
    pub use get_spot_order::get_spot_order;
    pub use get_spot_orders::get_spot_orders;
    pub use swap_estimation_by_denom::swap_estimation_by_denom;
}

pub mod execute {
    mod cancel_margin_order;
    mod cancel_margin_orders;
    mod cancel_spot_order;
    mod cancel_spot_orders;
    mod close_margin_position;
    mod create_margin_order;
    mod create_spot_order;

    mod stake_request;
    mod unstake_request;

    mod claim_rewards_request;
    mod claim_validator_commission_request;
    mod eden_cancel_vest_request;
    mod eden_vest_request;
    mod elys_cancel_unstake_request;
    mod elys_redelegation_request;

    use super::*;

    pub use cancel_margin_order::cancel_margin_order;
    pub use cancel_margin_orders::cancel_margin_orders;
    pub use cancel_spot_order::cancel_spot_order;
    pub use cancel_spot_orders::cancel_spot_orders;
    pub use close_margin_position::close_margin_position;
    pub use create_margin_order::create_margin_order;
    pub use create_spot_order::create_spot_order;

    pub use claim_rewards_request::claim_rewards_request;
    pub use claim_validator_commission_request::claim_validator_commission_request;
    pub use eden_cancel_vest_request::eden_cancel_vest_request;
    pub use eden_vest_request::eden_vest_request;
    pub use elys_cancel_unstake_request::elys_cancel_unstake_request;
    pub use elys_redelegation_request::elys_redelegation_request;
    pub use stake_request::stake_request;
    pub use unstake_request::unstake_request;
}

pub mod reply {
    use super::*;
    use elys_bindings::msg_resp::*;

    mod close_margin_position;
    mod create_margin_order_market_close;
    mod create_margin_order_market_open;
    mod open_margin_position;
    mod spot_order;
    mod spot_order_market;

    pub use close_margin_position::reply_to_close_margin_order;
    pub use create_margin_order_market_close::reply_to_create_margin_market_close;
    pub use create_margin_order_market_open::reply_to_create_margin_market_open;
    pub use open_margin_position::reply_to_open_margin_position;
    pub use spot_order::reply_to_spot_order;
    pub use spot_order_market::reply_to_spot_order_market;
}

pub mod sudo {
    use super::*;

    mod process_orders;
    pub use process_orders::process_orders;
}
