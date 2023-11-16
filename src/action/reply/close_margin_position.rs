use cosmwasm_std::SubMsgResult;

use crate::helper::get_response_from_reply;

use super::*;

pub fn reply_to_close_margin_order(
    deps: DepsMut<ElysQuery>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let close_resp: MarginCloseResponse = match get_response_from_reply(module_resp) {
        Ok(close_resp) => close_resp,
        Err(err) => return Ok(err),
    };

    let orders: Vec<MarginOrder> = MARGIN_ORDER.load(deps.storage)?;

    let orders: Vec<MarginOrder> = orders
        .iter()
        .filter(|order| order.order_id != close_resp.id)
        .cloned()
        .collect();

    MARGIN_ORDER.save(deps.storage, &orders)?;

    let resp = Response::new().add_attribute("order_id", close_resp.id.to_string());

    Ok(resp)
}
