use super::*;

pub fn get_spot_orders(
    deps: Deps<ElysQuery>,
    pagination: PageRequest,
    order_owner: Option<String>,
    order_type: Option<SpotOrderType>,
    order_status: Option<Status>,
) -> Result<GetSpotOrdersResp, ContractError> {
    let orders: Vec<SpotOrder> = SPOT_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok().map(|r| r.1))
        .collect();

    let (orders, page_response) = pagination.filter(orders)?;

    if orders.is_empty() {
        return Ok(GetSpotOrdersResp {
            page_response,
            orders,
        });
    };

    let orders: Vec<SpotOrder> = orders
        .iter()
        .filter(|order| {
            order_owner
                .as_ref()
                .map_or(true, |owner| owner == order.owner_address.as_str())
                && order_type
                    .as_ref()
                    .map_or(true, |order_type| order_type == &order.order_type)
                && order_status
                    .as_ref()
                    .map_or(true, |status| &order.status == status)
        })
        .cloned()
        .collect();

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
