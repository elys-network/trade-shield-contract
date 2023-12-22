use cosmwasm_std::{from_json, Binary, DepsMut, SubMsgResult};

use crate::helper::get_response_from_reply;

use super::*;

pub fn reply_to_spot_order(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let order_id: u64 = from_json(&data.unwrap()).unwrap();

    let mut order = SPOT_ORDER.load(deps.storage, order_id)?;

    let _: AmmSwapExactAmountInResp = match get_response_from_reply(module_resp) {
        Ok(expr) => expr,
        Err(err) => {
            return Ok(err);
        }
    };

    order.status = Status::Executed;

    SPOT_ORDER.save(deps.storage, order_id, &order)?;
    PENDING_SPOT_ORDER.remove(deps.storage, order.order_id);

    let resp: Response<ElysMsg> = Response::new().add_event(
        Event::new("reply_to_spot_order").add_attribute("order_id", order_id.to_string()),
    );

    Ok(resp)
}
