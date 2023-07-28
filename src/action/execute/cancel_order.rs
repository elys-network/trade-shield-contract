use super::*;

pub fn cancel_order(
    info: MessageInfo,
    deps: DepsMut,
    order_id: u128,
) -> Result<Response, ContractError> {
    let orders_list: Vec<Order> = ORDER.load(deps.storage)?;
    let order: Order = match orders_list.iter().find(|order| order.id == order_id) {
        Some(order) => order.to_owned(),
        None => return Err(ContractError::OrderNotFound { order_id }),
    };

    if order.user_address != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let refund_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![order.user_token],
    };

    let resp = Response::new().add_message(CosmosMsg::Bank(refund_msg));

    let new_orders_list: Vec<Order> = orders_list
        .into_iter()
        .filter(|order| order.id != order_id)
        .collect();

    ORDER.save(deps.storage, &new_orders_list)?;

    Ok(resp)
}
