use super::*;

pub fn cancel_margin_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_id: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    let orders = MARGIN_ORDER.load(deps.storage)?;

    let order: MarginOrder = match orders.iter().find(|order| order.order_id == order_id) {
        Some(order) => order.to_owned(),
        None => return Err(ContractError::OrderNotFound { order_id }),
    };

    if order.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let orders: Vec<MarginOrder> = orders
        .iter()
        .filter(|order| order.order_id != order_id)
        .cloned()
        .collect();

    let refund_msg = BankMsg::Send {
        to_address: order.owner,
        amount: vec![order.collateral],
    };

    let resp = Response::new()
        .add_message(CosmosMsg::Bank(refund_msg))
        .add_attribute("order_id", order.order_id.to_string());

    MARGIN_ORDER.save(deps.storage, &orders)?;

    Ok(resp)
}
