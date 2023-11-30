use cosmwasm_std::{from_json, Binary, DepsMut, SubMsgResult};

use crate::helper::get_response_from_reply;

use super::*;

pub fn reply_to_spot_order(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let order_id: u64 = from_json(&data.unwrap()).unwrap();

    let mut orders: Vec<SpotOrder> = SPOT_ORDER.load(deps.storage)?;

    let order: &mut SpotOrder = orders
        .iter_mut()
        .find(|order| order.order_id == order_id)
        .unwrap();

    let _: AmmSwapExactAmountInResp = match get_response_from_reply(module_resp) {
        Ok(expr) => expr,
        Err(err) => {
            order.status = Status::NotProcessed;
            SPOT_ORDER.save(deps.storage, &orders)?;
            return Ok(err);
        }
    };

    order.status = Status::Processed;

    SPOT_ORDER.save(deps.storage, &orders)?;

    let resp: Response<ElysMsg> = Response::new()
        .add_attribute("event_type", "reply_to_spot_order")
        .add_attribute("order_id", order_id.to_string());

    Ok(resp)
}
