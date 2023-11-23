use crate::{
    states::*,
    types::{MarginOrder, SpotOrder},
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult};
use elys_bindings::{ElysMsg, ElysQuery};

#[cw_serde]
pub struct InstantiateMockMsg {
    pub process_order_executor: String,
    pub spot_orders: Vec<SpotOrder>,
    pub margin_orders: Vec<MarginOrder>,
}

pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMockMsg,
) -> StdResult<Response<ElysMsg>> {
    SPOT_ORDER.save(deps.storage, &msg.spot_orders)?;
    deps.querier
        .query_balance(msg.process_order_executor.clone(), "usdc")?;
    PROCESS_SPOT_ORDER_EXECUTOR.save(deps.storage, &Addr::unchecked(msg.process_order_executor))?;
    PROCESSED_SPOT_ORDER.save(deps.storage, &vec![])?;
    MARGIN_ORDER.save(deps.storage, &msg.margin_orders)?;
    REPLY_INFO.save(deps.storage, &vec![])?;
    Ok(Response::new())
}
