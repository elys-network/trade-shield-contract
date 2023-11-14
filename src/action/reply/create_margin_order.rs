use crate::helper::get_response_from_reply;

use super::*;
use cosmwasm_std::{from_json, Binary, SubMsgResult};

pub fn reply_to_create_margin_order(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let open_resp: MarginOpenResponse = match get_response_from_reply(module_resp) {
        Ok(open_resp) => open_resp,
        Err(err) => return Ok(err),
    };

    let mut order: MarginOrder = match data {
        Some(ref data) => from_json(data)?,
        None => return Ok(Response::new().add_attribute("error", "no meta_data".to_string())),
    };

    order.order_id = open_resp.id;

    let mut orders = MARGIN_ORDER.load(deps.storage)?;
    orders.push(order);

    let resp = Response::new().add_attribute("order_id", open_resp.id.to_string());

    Ok(resp)
}
