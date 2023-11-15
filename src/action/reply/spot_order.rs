use cosmwasm_std::{coins, from_json, Binary, DepsMut, SubMsgResult};

use crate::helper::get_response_from_reply;

use super::*;

pub fn reply_to_spot_order(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let amm_response: AmmSwapExactAmountInResp = match get_response_from_reply(module_resp) {
        Ok(expr) => expr,
        Err(err) => return Ok(err),
    };

    let orders: Vec<SpotOrder> = SPOT_ORDER.load(deps.storage)?;

    let order_id: u64 = match data {
        Some(ref order_id) => from_json(&order_id)?,
        None => return Ok(Response::new().add_attribute("error", "no meta_data".to_string())),
    };

    let order: SpotOrder = match orders.iter().find(|order| order.order_id == order_id) {
        Some(order) => order.to_owned(),
        None => {
            return Ok(Response::new().add_attribute(
                "error",
                format!("{:?}", ContractError::OrderNotFound { order_id }),
            ))
        }
    };

    let bank_msg = BankMsg::Send {
        to_address: order.owner_address.to_string(),
        amount: coins(
            amm_response.token_out_amount.i64() as u128,
            order.order_target_denom.to_string(),
        ),
    };

    let mut processd_spot_orders = PROCESSED_SPOT_ORDER.load(deps.storage)?;
    processd_spot_orders.push((order_id, bank_msg));
    PROCESSED_SPOT_ORDER.save(deps.storage, &processd_spot_orders)?;

    let resp: Response<ElysMsg> = Response::new();

    Ok(resp)
}
