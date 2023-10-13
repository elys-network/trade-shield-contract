use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    bindings::{msg::ElysMsg, query::ElysQuery},
    states::*,
    types::Order,
};

#[cw_serde]
pub struct InstantiateMockMsg {
    pub process_order_executor: String,
    pub orders: Vec<Order>,
}

pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMockMsg,
) -> StdResult<Response<ElysMsg>> {
    ORDER.save(deps.storage, &msg.orders)?;
    deps.querier
        .query_balance(msg.process_order_executor.clone(), "usdc")?;
    PROCESS_ORDER_EXECUTOR.save(deps.storage, &Addr::unchecked(msg.process_order_executor))?;
    Ok(Response::new())
}
