use cosmwasm_std::{coins, from_binary, Binary, DepsMut, StdError};

use crate::bindings::msg_resp::MsgSwapExactAmountInResp;

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

    let new_orders_list: Vec<SpotOrder> = orders
        .into_iter()
        .filter(|order| order.order_id != order_id)
        .collect();

    let bank_msg = BankMsg::Send {
        to_address: order.owner_address.to_string(),
        amount: coins(amm_response.token_out_amount(), order.order_target_denom),
    };

    SPOT_ORDER.save(deps.storage, &new_orders_list)?;

    let resp: Response<ElysMsg> = Response::new()
        .add_attribute("order_processed", order.order_id.to_string())
        .add_message(CosmosMsg::Bank(bank_msg));

    Ok(resp)
}
