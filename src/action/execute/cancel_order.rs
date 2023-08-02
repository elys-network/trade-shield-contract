use super::*;

pub fn cancel_order(
    info: MessageInfo,
    deps: DepsMut,
    order_id: u128,
) -> Result<Response, ContractError> {
    let orders_list: Vec<Order> = ORDER.load(deps.storage)?;
    let order: Order = match orders_list.iter().find(|order| order.order_id == order_id) {
        Some(order) => order.to_owned(),
        None => return Err(ContractError::OrderNotFound { order_id }),
    };

    if order.owner_address != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let refund_msg = BankMsg::Send {
        to_address: order.owner_address.to_string(),
        amount: vec![order.order_amount],
    };

    let resp = Response::new().add_message(CosmosMsg::Bank(refund_msg));

    let new_orders_list: Vec<Order> = orders_list
        .into_iter()
        .filter(|order| order.order_id != order_id)
        .collect();

    ORDER.save(deps.storage, &new_orders_list)?;

    Ok(resp)
}
