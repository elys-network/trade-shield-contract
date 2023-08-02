use super::*;
use crate::msg::query_resp::GetOrderResp;

pub fn get_order(deps: Deps, order_id: u128) -> Result<GetOrderResp, ContractError> {
    let orders_list: Vec<Order> = ORDER.load(deps.storage)?;
    let have_order = orders_list.iter().find(|order| order.id == order_id);

    let resp = GetOrderResp {
        order: match have_order {
            Some(order) => order.to_owned(),
            None => return Err(ContractError::OrderNotFound { order_id: order_id }),
        },
    };

    Ok(resp)
}
