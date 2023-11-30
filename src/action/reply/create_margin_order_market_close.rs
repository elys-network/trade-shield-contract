use super::*;
use cosmwasm_std::{from_json, Binary, StdError, SubMsgResult};

pub fn reply_to_create_margin_market_close(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let resp_data = match module_resp.into_result() {
        Ok(resp) => resp.data,
        Err(err) => return Err(StdError::generic_err(err).into()),
    };

    let mut orders: Vec<MarginOrder> = MARGIN_ORDER.load(deps.storage)?;

    let order_id: u64 = match data {
        Some(order_id) => from_json(&order_id)?,
        None => return Err(StdError::generic_err("no meta_data").into()),
    };

    if resp_data.is_none() {
        return Err(StdError::generic_err("no data from response").into());
    }

    let order: &mut MarginOrder = match orders.iter_mut().find(|order| order.order_id == order_id) {
        Some(order) => order,
        None => return Err(ContractError::OrderNotFound { order_id }),
    };

    let margin_resp: MarginBrokerCloseResResponse = match from_json(&resp_data.unwrap()) {
        Ok(resp) => resp,
        Err(err) => return Err(err.into()),
    };

    order.status = Status::Processed;

    let resp =
        Response::new().add_attribute("margin_trading_position_id", margin_resp.id.to_string());

    Ok(resp)
}
