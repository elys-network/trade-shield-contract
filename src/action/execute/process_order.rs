use cosmwasm_std::{coin, coins, to_binary, SubMsg, WasmMsg};

use crate::{bindings::query::ElysQuery, msg::ExecuteMsg};

use super::*;

pub fn process_order(
    deps: DepsMut<ElysQuery>,
    _info: MessageInfo,
    env: Env,
) -> Result<Response<ElysMsg>, ContractError> {
    let orders = ORDER.load(deps.storage)?;
    let order = orders[0].clone();

    let swap_msg = ElysMsg::swap_exact_amount_in(
        env.contract.address.as_str(),
        order.order_amount,
        order.order_amm_routes,
    );

    let sub_msg = SubMsg::reply_always(swap_msg, 1);

    Ok(Response::new().add_submessage(sub_msg))
}
