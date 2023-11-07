use cosmwasm_std::{to_binary, SubMsg};

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

    if order.creator == info.sender.to_string() {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let meta_data = Some(to_binary(&(order_id, order.creator.clone()))?);

    let cancel_msg = ElysMsg::close_position(order.creator, order_id, meta_data);

    let resp = Response::new().add_submessage(SubMsg::reply_always(
        cancel_msg,
        ReplyType::MarginClosePosition as u64,
    ));

    Ok(resp)
}
