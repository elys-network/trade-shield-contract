use crate::{bindings::msg_resp::MsgSwapExactAmountInResp, states::PROCESSED_SPOT_ORDER};
use cosmwasm_std::{coins, from_binary, Binary, DepsMut, StdError};

use super::*;

pub fn reply_to_spot_order(
    deps: DepsMut<ElysQuery>,
    data: Binary,
) -> Result<Response<ElysMsg>, ContractError> {
    let orders: Vec<SpotOrder> = SPOT_ORDER.load(deps.storage)?;
    let amm_response: MsgSwapExactAmountInResp = from_binary(&data)?;

    let order_id: u64 = match amm_response.meta_data {
        Some(ref order_id) => from_binary(&order_id)?,
        None => {
            return Err(ContractError::StdError(StdError::GenericErr {
                msg: "no metadata".to_string(),
            }))
        }
    };

    let order = match orders.iter().find(|order| order.order_id == order_id) {
        Some(order) => order.to_owned(),
        None => return Err(ContractError::SpotOrderNotFound { order_id }),
    };

    let bank_msg = BankMsg::Send {
        to_address: order.owner_address.to_string(),
        amount: coins(
            amm_response.token_out_amount(),
            order.order_target_denom.to_string(),
        ),
    };

    let mut processd_spot_orders = PROCESSED_SPOT_ORDER.load(deps.storage)?;
    processd_spot_orders.push((order_id, bank_msg));
    PROCESSED_SPOT_ORDER.save(deps.storage, &processd_spot_orders)?;

    let resp: Response<ElysMsg> = Response::new();

    Ok(resp)
}
