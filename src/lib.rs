pub mod entry_point {
    use crate::action;
    use crate::error::ContractError;
    use crate::msg;
    use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    mod execute;
    mod instantiate;
    mod query;

    pub use execute::execute;
    pub use instantiate::instantiate;
    pub use query::query;
}

mod bindings {
    pub mod msg;
    pub mod querier;
    pub mod query;
    pub mod query_resp;
}

pub mod msg {
    mod execute_msg;
    mod instantiate_msg;
    mod query_msg;

    pub use execute_msg::ExecuteMsg;
    pub use instantiate_msg::InstantiateMsg;
    pub use query_msg::QueryMsg;
    pub mod query_resp {
        mod get_all_prices_resp;
        mod get_order_resp;
        pub use get_all_prices_resp::GetAllPricesResponse;
        pub use get_order_resp::GetOrderResp;
    }
}

pub mod types {
    mod order_type;
    mod order {
        pub mod order;
        mod impls {
            mod new;
            #[cfg(test)]
            mod new_dummy;
        }
    }
    mod pool;
    mod price;
    mod swap_route;

    pub use order::order::Order;
    mod page_request;
    pub use page_request::PageRequest;
    pub mod page_response;
    pub use order_type::OrderType;
    pub use pool::*;
    pub use price::Price;
    pub use swap_route::*;
}

mod error;
use bindings::query::ElysQuery;
pub use error::ContractError;

mod states {
    mod order;

    pub use order::ORDER;
}

mod action {
    use crate::{states::ORDER, types::*, ContractError};

    pub mod query {
        mod get_all_price;
        mod get_order;
        use super::*;
        use cosmwasm_std::Deps;
        pub use get_all_price::get_all_prices;
        pub use get_order::get_order;
    }

    pub mod execute {
        mod cancel_order;
        mod create_order;
        mod process_order;

        use super::*;
        use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response};

        pub use cancel_order::cancel_order;
        pub use create_order::create_order;
        pub use process_order::process_order;
    }
}

#[cfg(test)]
mod tests;

use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    entry_point::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ElysQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    entry_point::execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    entry_point::query(deps, env, msg)
}
