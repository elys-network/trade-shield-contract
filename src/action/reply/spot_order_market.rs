use cosmwasm_std::{coins, from_json, Binary, DepsMut, StdError, SubMsgResult};

use super::*;

pub fn reply_to_spot_order_market(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let response = match module_resp.into_result() {
        Ok(response) => response,
        Err(err) => return Err(StdError::generic_err(err).into()),
    };

    let meta_data = match response.data {
        Some(data) => data,
        None => return Err(StdError::generic_err("No Data").into()),
    };

    let amm_response: AmmSwapExactAmountInResp = from_json(&meta_data)?;

    let orders: Vec<SpotOrder> = SPOT_ORDER.load(deps.storage)?;

    let order_id: u64 = match data {
        Some(order_id) => from_json(&order_id)?,
        None => return Err(StdError::generic_err("no meta_data".to_string()).into()),
    };

    let order: SpotOrder = match orders.iter().find(|order| order.order_id == order_id) {
        Some(order) => order.to_owned(),
        None => return Err(ContractError::OrderNotFound { order_id }),
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
