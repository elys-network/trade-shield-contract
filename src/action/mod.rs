use crate::bindings::{msg::ElysMsg, query::ElysQuery};
use crate::{states::SPOT_ORDER, types::*, ContractError};
use cosmwasm_std::{BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response};

pub mod query {
    mod get_all_price;
    mod get_spot_order;
    use super::*;
    use crate::msg::query_resp::*;
    use cosmwasm_std::Deps;
    pub use get_all_price::get_all_prices;
    pub use get_spot_order::get_spot_order;
}

pub mod execute {
    mod cancel_spot_order;
    mod create_spot_order;
    mod process_spot_orders;

    use super::*;

    pub use cancel_spot_order::cancel_spot_order;
    pub use create_spot_order::create_spot_order;
    pub use process_spot_orders::process_spot_orders;
}

pub mod reply {
    use super::*;

    mod spot_order;

    pub use spot_order::reply_to_spot_order;
}
