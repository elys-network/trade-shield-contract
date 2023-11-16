use crate::{states::*, types::*, ContractError};
use cosmwasm_std::{BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response};
use elys_bindings::*;

pub mod query {
    mod asset_info;
    mod get_all_price;
    mod get_spot_order;
    mod get_spot_orders;

    use super::*;

    use crate::msg::query_resp::*;
    use cosmwasm_std::Deps;
    use elys_bindings::query_resp::*;

    pub use asset_info::asset_info;
    pub use get_all_price::get_all_prices;
    pub use get_spot_order::get_spot_order;
    pub use get_spot_orders::get_spot_orders;
}

pub mod execute {
    mod cancel_spot_order;
    mod cancel_spot_orders;
    mod create_margin_order;
    mod create_spot_order;
    mod process_spot_orders;
    use super::*;

    pub use cancel_spot_order::cancel_spot_order;
    pub use cancel_spot_orders::cancel_spot_orders;
    pub use create_margin_order::create_margin_order;
    pub use create_spot_order::create_spot_order;
    pub use process_spot_orders::process_spot_orders;
}

pub mod reply {
    use super::*;
    use elys_bindings::msg_resp::*;

    mod create_margin_order;
    mod spot_order;

    pub use create_margin_order::reply_to_create_margin_order;
    pub use spot_order::reply_to_spot_order;
}
