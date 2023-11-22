use cosmwasm_std::{to_json_binary, SubMsg};

use crate::msg::ReplyType;

use super::*;

pub fn cancel_margin_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_id: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    let orders = MARGIN_ORDER.load(deps.storage)?;

    let order: MarginOrder = match orders.iter().find(|order| order.order_id == order_id) {
        Some(order) => order.to_owned(),
        None => return Err(ContractError::OrderNotFound { order_id }),
    };

    if order.owner == info.sender.to_string() {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let orders: Vec<MarginOrder> = orders
        .iter()
        .filter(|order| order.order_id != order_id)
        .cloned()
        .collect();

    let return_msg = BankMsg::Send {
        to_address: order.owner,
        amount: vec![order.collateral],
    };

    MARGIN_ORDER.save(deps.storage, &orders)?;

    Ok(Response::new()
        .add_message(return_msg)
        .add_attribute("order_id", order.order_id.to_string()))
}
