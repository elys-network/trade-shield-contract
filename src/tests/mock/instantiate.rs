use crate::{
    states::*,
    types::{MarginOrder, SpotOrder},
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use elys_bindings::{ElysMsg, ElysQuery};

#[cw_serde]
pub struct InstantiateMockMsg {
    pub spot_orders: Vec<SpotOrder>,
    pub margin_orders: Vec<MarginOrder>,
}

pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMockMsg,
) -> StdResult<Response<ElysMsg>> {
    if !msg.spot_orders.is_empty() {
        for order in msg.spot_orders.iter() {
            SPOT_ORDER.save(deps.storage, order.order_id, order)?;
        }
    }
    if !msg.margin_orders.is_empty() {
        for order in msg.margin_orders.iter() {
            MARGIN_ORDER.save(deps.storage, order.order_id, order)?;
        }
    }
    MAX_REPLY_ID.load(deps.storage)?;
    Ok(Response::new())
}
