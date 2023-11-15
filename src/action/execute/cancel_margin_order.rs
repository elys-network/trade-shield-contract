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

    if order.creator == info.sender.to_string() {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let meta_data = Some(to_json_binary(&order_id)?);

    let cancel_msg = ElysMsg::margin_close_position(order.creator, order_id);

    let mut reply_infos = REPLY_INFO.load(deps.storage)?;

    let reply_info_id =
        if let Some(reply_info) = reply_infos.iter().max_by_key(|reply_info| reply_info.id) {
            reply_info.id + 1
        } else {
            0
        };

    let reply_info = ReplyInfo {
        id: reply_info_id,
        reply_type: ReplyType::MarginClosePosition,
        data: meta_data,
    };

    reply_infos.push(reply_info);

    let resp = Response::new().add_submessage(SubMsg::reply_always(cancel_msg, reply_info_id));

    Ok(resp)
}
