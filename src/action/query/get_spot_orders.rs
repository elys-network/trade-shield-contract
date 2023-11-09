use super::*;
use cosmwasm_std::{from_binary, to_binary};

pub fn get_spot_orders(
    deps: Deps<ElysQuery>,
    pagination: PageRequest,
    order_owner: Option<String>,
    order_type: Option<SpotOrderType>,
) -> Result<GetSpotOrdersResp, ContractError> {
    let static_orders = SPOT_ORDER.load(deps.storage)?;
    let mut orders = static_orders.clone();

    let mut orders = filter_orders(orders, order_owner, order_type);

    if orders.is_empty() {
        return Ok(GetSpotOrdersResp::empty(pagination.count_total));
    }

    let _ = orders.split_off(pagination.limit as usize);

    if orders.is_empty() {
        return Ok(GetSpotOrdersResp::empty(pagination.count_total));
    }

    let total = if pagination.count_total {
        Some(orders.len() as u64)
    } else {
        None
    };

    let next_key = if orders.last() == static_orders.last() {
        None
    } else {
        Some(to_binary(
            &(static_orders
                .iter()
                .position(|order| order.order_id == order.order_id)
                .unwrap() as u32),
        )?)
    };

    Ok(GetSpotOrdersResp {
        pagination: PageResponse::new(next_key, total),
        orders,
    })
}

fn filter_orders(
    orders: Vec<SpotOrder>,
    order_owner: Option<String>,
    order_type: Option<SpotOrderType>,
) -> Vec<SpotOrder> {
    match (order_owner, order_type) {
        (None, Some(order_type)) => orders
            .iter()
            .filter(|order| order.order_type == order_type)
            .cloned()
            .collect(),
        (Some(owner), None) => orders
            .iter()
            .filter(|order| order.owner_address == owner)
            .cloned()
            .collect(),
        (Some(owner), Some(order_type)) => orders
            .iter()
            .filter(|order| order.owner_address == owner && order.order_type == order_type)
            .cloned()
            .collect(),
        _ => orders,
    }
}
