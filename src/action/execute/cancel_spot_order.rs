use super::*;

pub fn cancel_spot_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_id: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    let mut orders_list: Vec<SpotOrder> = SPOT_ORDER.load(deps.storage)?;

    let order: &mut SpotOrder = match orders_list
        .iter_mut()
        .find(|order| order.order_id == order_id)
    {
        Some(order) => order,
        None => return Err(ContractError::OrderNotFound { order_id }),
    };

    if order.owner_address != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    if order.status != Status::NotProcessed {
        return Err(ContractError::CancelStatusError {
            order_id,
            status: order.status.clone(),
        });
    }

    order.status = Status::Canceled;
    let refund_msg = BankMsg::Send {
        to_address: order.owner_address.to_string(),
        amount: vec![order.order_amount.clone()],
    };

    let resp = Response::new()
        .add_message(CosmosMsg::Bank(refund_msg))
        .add_event(Event::new("cancel_spot_order").add_attribute("order_id", order_id.to_string()));

    SPOT_ORDER.save(deps.storage, &orders_list)?;

    Ok(resp)
}
