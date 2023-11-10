use cosmwasm_std::{Addr, Coin, StdError};

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
            sender: Addr::unchecked(owner_address),
        });
    }

    let orders: Vec<SpotOrder> = SPOT_ORDER.load(deps.storage)?;
    let processed_spot_order: Vec<(u64, BankMsg)> = PROCESSED_SPOT_ORDER.load(deps.storage)?;

    let user_orders: Vec<SpotOrder> = orders
        .iter()
        .filter(|order| order.owner_address == info.sender)
        .cloned()
        .collect();

    if user_orders.is_empty() {
        return Err(ContractError::StdError(StdError::not_found(
            "no order found for this user",
        )));
    }

    let filtered_order: Vec<SpotOrder> = filter_order_by_id(&user_orders, &order_ids)?;

    for (id, _) in processed_spot_order {
        if filtered_order.iter().any(|order| order.order_id == id) {
            return Err(ContractError::ProcessSpotOrderProcessing { order_id: id });
        }
    }

    let filtered_order = filter_order_by_type(filtered_order, order_type)?;

    let new_orders_list = orders
        .into_iter()
        .filter(|order| !filtered_order.contains(order))
        .collect();

    SPOT_ORDER.save(deps.storage, &new_orders_list)?;

    let refund_msg = make_refund_msg(filtered_order, owner_address);

    Ok(Response::new().add_message(refund_msg))
}

fn filter_order_by_id(
    orders: &[SpotOrder],
    order_ids: &Option<Vec<u64>>,
) -> Result<Vec<SpotOrder>, ContractError> {
    let order_ids: Vec<u64> = order_ids.as_ref().map_or_else(
        || orders.iter().map(|order| order.order_id).collect(),
        |ids| ids.clone(),
    );

    let filtered_order: Vec<SpotOrder> = orders
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

        return Err(ContractError::SpotOrdersNotFound {
            order_ids: missing_order_ids,
        });
    }

    Ok(filtered_order)
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
