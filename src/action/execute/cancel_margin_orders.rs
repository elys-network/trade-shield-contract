use cosmwasm_std::{to_json_binary, Coin, StdError};

use super::*;

pub fn cancel_margin_orders(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_ids: Option<Vec<u64>>,
    owner_address: String,
    order_type: Option<MarginOrderType>,
) -> Result<Response<ElysMsg>, ContractError> {
    if info.sender.as_str() != owner_address {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let mut orders: Vec<MarginOrder> = MARGIN_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok().map(|r| r.1))
        .collect();

    let user_orders: Vec<MarginOrder> = orders
        .iter()
        .filter(|order| order.owner == info.sender)
        .cloned()
        .collect();

    if user_orders.is_empty() {
        return Err(ContractError::StdError(StdError::not_found(
            "no order found for this user",
        )));
    }

    let filtered_order: Vec<MarginOrder> = filter_order_by_id(&user_orders, &order_ids)?;

    let filtered_order = filter_order_by_type(filtered_order, order_type)?;

    if let Some(order) = filtered_order
        .iter()
        .find(|order| order.status != Status::NotProcessed)
    {
        return Err(ContractError::CancelStatusError {
            order_id: order.order_id,
            status: order.status.clone(),
        });
    }

    let order_ids: Vec<u64> = match order_ids {
        Some(order_ids) => order_ids,
        None => filtered_order.iter().map(|order| order.order_id).collect(),
    };

    for order in orders.iter_mut() {
        if order_ids.contains(&order.order_id) {
            order.status = Status::Canceled;
            MARGIN_ORDER.save(deps.storage, order.order_id, order)?;
        }
    }

    let refund_msg = make_refund_msg(filtered_order, owner_address);

    Ok(Response::new()
        .add_message(refund_msg)
        .set_data(to_json_binary(&order_ids)?))
}

fn filter_order_by_id(
    orders: &Vec<MarginOrder>,
    order_ids: &Option<Vec<u64>>,
) -> Result<Vec<MarginOrder>, ContractError> {
    let order_ids = match order_ids {
        Some(order_ids) => order_ids,
        None => return Ok(orders.to_owned()),
    };

    if order_ids.is_empty() {
        return Err(StdError::generic_err("order_ids is defined empty").into());
    }

    let filtered_order: Vec<MarginOrder> = orders
        .iter()
        .filter(|order| order_ids.contains(&order.order_id))
        .cloned()
        .collect();

    if order_ids.len() != filtered_order.len() {
        let missing_order_ids: Vec<u64> = order_ids
            .iter()
            .filter(|order_id| {
                !filtered_order
                    .iter()
                    .any(|order| order.order_id == **order_id)
            })
            .cloned()
            .collect();

        return Err(ContractError::OrdersNotFound {
            order_ids: missing_order_ids,
        });
    }

    Ok(filtered_order)
}

fn filter_order_by_type(
    orders: Vec<MarginOrder>,
    order_type: Option<MarginOrderType>,
) -> Result<Vec<MarginOrder>, ContractError> {
    let order_type = match order_type {
        Some(order_type) => order_type,
        None => return Ok(orders),
    };

    let filtered_order: Vec<MarginOrder> = orders
        .iter()
        .filter(|order| order.order_type == order_type)
        .cloned()
        .collect();

    if filtered_order.is_empty() {
        Err(ContractError::StdError(cosmwasm_std::StdError::not_found(
            "no order his this type",
        )))
    } else {
        Ok(filtered_order)
    }
}

fn make_refund_msg(orders: Vec<MarginOrder>, user: String) -> BankMsg {
    let orders_amount: Vec<Coin> = orders
        .into_iter()
        .filter_map(|order| {
            if order.order_type == MarginOrderType::LimitOpen {
                Some(order.collateral)
            } else {
                None
            }
        })
        .collect();

    let mut merged_amounts: Vec<Coin> = Vec::new();

    for amount in orders_amount {
        if let Some(existing_amount) = merged_amounts
            .iter_mut()
            .find(|coin| coin.denom == amount.denom)
        {
            existing_amount.amount += amount.amount;
        } else {
            merged_amounts.push(amount);
        }
    }

    BankMsg::Send {
        to_address: user,
        amount: merged_amounts,
    }
}
