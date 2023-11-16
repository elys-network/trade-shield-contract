use super::*;

pub fn get_spot_orders(
    deps: Deps<ElysQuery>,
    pagination: PageRequest,
    order_owner: Option<String>,
    order_type: Option<SpotOrderType>,
) -> Result<GetSpotOrdersResp, ContractError> {
    let orders = SPOT_ORDER.load(deps.storage)?;

    let (orders, page_response) = pagination.filter(orders)?;

    if orders.is_empty() {
        return Ok(GetSpotOrdersResp {
            page_response,
            orders,
        });
    };

    let orders = match (order_owner, order_type) {
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
        (None, None) => orders,
    };

    let page_response = match page_response.total {
        Some(_) => PageResponse {
            next_key: page_response.next_key,
            total: Some(orders.len() as u64),
        },
        None => page_response,
    };

    Ok(GetSpotOrdersResp {
        page_response,
        orders,
    })
}
