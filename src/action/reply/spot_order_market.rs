use cosmwasm_std::{from_json, Binary, DepsMut, StdError, SubMsgResult};

use super::*;

pub fn reply_to_spot_order_market(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    if let Err(err) = module_resp.into_result() {
        return Err(StdError::generic_err(err).into());
    };
    let mut orders: Vec<SpotOrder> = SPOT_ORDER.load(deps.storage)?;

    let order_id: u64 = match data {
        Some(order_id) => from_json(&order_id)?,
        None => return Err(StdError::generic_err("no meta_data".to_string()).into()),
    };

    let order: &mut SpotOrder = match orders.iter_mut().find(|order| order.order_id == order_id) {
        Some(order) => order,
        None => return Err(ContractError::OrderNotFound { order_id }),
    };

    order.status = Status::Processed;

    SPOT_ORDER.save(deps.storage, &orders)?;

    let resp: Response<ElysMsg> =
        Response::new().add_attribute("processed_order_id", order_id.to_string());

    Ok(resp)
}
