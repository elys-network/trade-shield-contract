use cosmwasm_std::{coins, from_binary, BankMsg, CosmosMsg, Reply};

use crate::{bindings::msg_resp::MsgSwapExactAmountInResp, states::ORDER, types::Order};

use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    msg: Reply,
) -> Result<Response<ElysMsg>, ContractError> {
    let reply = match msg.result.into_result() {
        Ok(reply) => reply,
        Err(err) => {
            return Err(ContractError::StdError(
                cosmwasm_std::StdError::GenericErr { msg: err },
            ));
        }
    };

    let amm_response: MsgSwapExactAmountInResp = match reply.data {
        Some(resp) => from_binary(&resp)?,
        None => {
            return Err(ContractError::StdError(
                cosmwasm_std::StdError::GenericErr {
                    msg: "no data".to_string(),
                },
            ))
        }
    };

    let orders: Vec<Order> = ORDER.load(deps.storage)?;

    let order = match orders.iter().find(|order| order.order_id == msg.id) {
        Some(order) => order.to_owned(),
        None => return Err(ContractError::OrderNotFound { order_id: msg.id }),
    };

    let new_orders_list: Vec<Order> = orders
        .into_iter()
        .filter(|order| order.order_id != msg.id)
        .collect();

    let bank_msg = BankMsg::Send {
        to_address: order.owner_address.to_string(),
        amount: coins(amm_response.to_uint128(), order.order_target_denom),
    };

    ORDER.save(deps.storage, &new_orders_list)?;

    let resp: Response<ElysMsg> = Response::new()
        .add_attribute("order_processed", order.order_id.to_string())
        .add_message(CosmosMsg::Bank(bank_msg));

    Ok(resp)
}
