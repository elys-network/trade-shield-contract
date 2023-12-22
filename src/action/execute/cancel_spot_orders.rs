use std::collections::HashMap;

use cosmwasm_std::{to_json_binary, Coin, StdError};

use super::*;

pub fn cancel_spot_orders(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_ids: Option<Vec<u64>>,
    owner_address: String,
    order_type: Option<SpotOrderType>,
) -> Result<Response<ElysMsg>, ContractError> {
    if info.sender.as_str() != owner_address {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let orders: Vec<SpotOrder> = if let Some(ids) = &order_ids {
        if ids.is_empty() {
            return Err(StdError::generic_err("order_ids is defined empty").into());
        };
        let orders = ids
            .iter()
            .map(|id| SPOT_ORDER.load(deps.storage, *id))
            .collect::<Result<Vec<SpotOrder>, StdError>>()?;

        if orders
            .iter()
            .any(|order| order.owner_address != owner_address)
        {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        if let Some(order) = orders.iter().find(|order| order.status != Status::Pending) {
            return Err(ContractError::CancelStatusError {
                order_id: order.order_id,
                status: order.status.clone(),
            });
        }

        orders
    } else {
        let orders: Vec<SpotOrder> = SPOT_ORDER
            .prefix_range(deps.storage, None, None, Order::Ascending)
            .filter_map(|res| {
                if let Some(r) = res.ok() {
                    Some(r.1)
                } else {
                    None
                }
            })
            .filter(|order| {
                order.owner_address.as_str() == &owner_address && order.status == Status::Pending
            })
            .collect();

        if orders.is_empty() {
            return Err(ContractError::StdError(StdError::not_found(
                "no order found for this user",
            )));
        };

        orders
    };

    let mut orders = filter_order_by_type(orders, order_type)?;

    for order in orders.iter_mut() {
        order.status = Status::Canceled;
        SPOT_ORDER.save(deps.storage, order.order_id, &order)?;
        PENDING_SPOT_ORDER.remove(deps.storage, order.order_id);
    }

    let order_ids: Vec<u64> = orders.iter().map(|order| order.order_id).collect();

    let refund_msg = make_refund_msg(orders, owner_address);

    Ok(Response::new()
        .add_message(refund_msg)
        .set_data(to_json_binary(&order_ids)?))
}

fn filter_order_by_type(
    orders: Vec<SpotOrder>,
    order_type: Option<SpotOrderType>,
) -> Result<Vec<SpotOrder>, ContractError> {
    let order_type = match order_type {
        Some(order_type) => order_type,
        None => return Ok(orders),
    };

    let filtered_order: Vec<SpotOrder> = orders
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

fn make_refund_msg(orders: Vec<SpotOrder>, user: String) -> BankMsg {
    let orders_amount: Vec<Coin> = orders.into_iter().map(|order| order.order_amount).collect();

    let mut merged_amounts: HashMap<String, Coin> = HashMap::new();

    for order_amount in orders_amount {
        if let Some(entry) = merged_amounts.get_mut(&order_amount.denom) {
            entry.amount += order_amount.amount;
        } else {
            merged_amounts.insert(order_amount.denom.clone(), order_amount);
        }
    }

    let merged_amounts: Vec<Coin> = merged_amounts.values().cloned().collect();

    BankMsg::Send {
        to_address: user,
        amount: merged_amounts,
    }
}
