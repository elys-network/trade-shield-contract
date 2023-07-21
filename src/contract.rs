use crate::action;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::*;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ORDER.save(deps.storage, &msg.orders)?;
    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use QueryMsg::*;

    match msg {
        GetOrder { order_id } => to_binary(&action::query::get_order(deps, order_id)?),
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        CreateOrder {
            order_type,
            stop_price,
            selling_denom,
        } => action::execute::create_order(deps, info, order_type, stop_price, selling_denom),
        CancelOrder { order_id: String } => action::execute::cancel_order(info, deps, order_id),
        ExecuteOrder {} => action::execute::execute_order(deps, info),
    }
}
