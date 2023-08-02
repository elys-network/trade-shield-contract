pub mod entry_point {
    use crate::action;
    use crate::error::ContractError;
    use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use crate::msg;
    
    mod execute;
    mod instantiate;
    mod query;

    pub use execute::execute;
    pub use query::query;
    pub use instantiate::instantiate;
}

pub mod msg {
    mod execute_msg;
    mod instantiate_msg;
    mod query_msg;

    pub use execute_msg::ExecuteMsg;
    pub use instantiate_msg::InstantiateMsg;
    pub use query_msg::QueryMsg;
    pub mod query_resp {
        mod get_order_resp;
        pub use get_order_resp::GetOrderResp;
    }
}

pub mod types {
    mod order_type;
    mod order {
        pub mod order;
        mod impls {
            pub mod new;
            #[cfg(test)]
            pub mod new_dummy;
        }
    }

    pub use order_type::OrderType;
    pub use order::order::Order;
}

mod error;
mod state;

pub use error::ContractError;
pub use state::ORDER;

mod action {
    use crate::{
        ContractError,
        ORDER,
        types::*,
    };

    pub mod query {
        mod get_order;

        use super::*;
        use cosmwasm_std::Deps;

        pub use get_order::get_order;
    }

    pub mod execute {
        mod create_order;
        mod cancel_order;
        mod process_order;
        
        use super::*;
        use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response};
        
        pub use create_order::create_order;
        pub use cancel_order::cancel_order;
        pub use process_order::process_order;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        entry_point::*,
        msg::*,
        types::{Order, OrderType},
        ContractError,
    };
    use cosmwasm_std::{coin, coins, Addr, Event};
    use cw_multi_test::{App, ContractWrapper, Executor};
    
    mod get_user_id_from_events;
    
    mod create_order {
        use super::*;
        mod coin_number;
        mod successful_create_order;
    }

    mod cancel_order {
        use super::*;
        mod not_found;
        mod unauthorized;
        mod successful_cancel_order_with_created_order;

        mod successful_cancel_order_with_dummy_order;
    }
    

    mod get_order{
        use super::*;
        use cosmwasm_std::{Binary, StdError};
        mod not_found;
        mod successful_query_message;
    }
}

use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    entry_point::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    entry_point::execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    entry_point::query(deps, env, msg)
}
