use crate::bindings::{msg::ElysMsg, query::ElysQuery};
use crate::{states::SPOT_ORDER, types::*, ContractError};
use cosmwasm_std::{BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response};

pub mod query {
    mod asset_info;
    mod get_all_price;
    mod get_spot_order;

    use super::*;

    use crate::msg::query_resp::*;
    use cosmwasm_std::Deps;

    pub use asset_info::asset_info;
    pub use get_all_price::get_all_prices;
    pub use get_spot_order::get_spot_order;
}

pub mod execute {
    mod cancel_order;
    mod create_order;
    mod process_spot_orders;

    use super::*;

    pub use cancel_order::cancel_order;
    pub use create_order::create_order;
    pub use process_spot_orders::process_spot_orders;
}

pub mod reply {
    use super::*;

    mod spot_order;

    pub use spot_order::reply_to_spot_order;
}
