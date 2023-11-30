use cosmwasm_std::{from_json, Binary, SubMsgResult};

use crate::helper::get_response_from_reply;

use super::*;

pub fn reply_to_open_margin_position(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let order_id: u64 = from_json(&data.unwrap()).unwrap();

    let mut orders: Vec<MarginOrder> = MARGIN_ORDER.load(deps.storage)?;

    let order: &mut MarginOrder = orders
        .iter_mut()
        .find(|order| order.order_id == order_id)
        .unwrap();

    let res: MarginBrokerOpenResResponse = match get_response_from_reply(module_resp) {
        Ok(expr) => expr,
        Err(err) => {
            order.status = Status::NotProcessed;
            MARGIN_ORDER.save(deps.storage, &orders)?;
            return Ok(err);
        }
    };

    order.status = Status::Processed;

    MARGIN_ORDER.save(deps.storage, &orders)?;

    let resp: Response<ElysMsg> = Response::new()
        .add_attribute("event_type", "reply_to_open_margin_position")
        .add_attribute("margin_order_id", order_id.to_string())
        .add_attribute("margin_trading_position_opened_id", res.id.to_string());

    Ok(resp)
}
