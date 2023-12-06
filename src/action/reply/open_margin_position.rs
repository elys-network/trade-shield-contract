use cosmwasm_std::{from_json, Binary, SubMsgResult};

use crate::helper::get_response_from_reply;

use super::*;

pub fn reply_to_open_margin_position(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let order_id: u64 = from_json(&data.unwrap()).unwrap();

    let mut order: MarginOrder = MARGIN_ORDER.load(deps.storage, order_id)?;

    let res: MarginOpenResponse = match get_response_from_reply(module_resp) {
        Ok(expr) => expr,
        Err(err) => {
            order.status = Status::Pending;
            MARGIN_ORDER.save(deps.storage, order_id, &order)?;
            return Ok(err);
        }
    };

    order.status = Status::Executed;

    MARGIN_ORDER.save(deps.storage, order_id, &order)?;

    let resp: Response<ElysMsg> = Response::new().add_event(
        Event::new("reply_to_open_margin_position")
            .add_attribute("margin_order_id", order_id.to_string())
            .add_attribute("margin_trading_position_opened_id", res.id.to_string()),
    );

    Ok(resp)
}
