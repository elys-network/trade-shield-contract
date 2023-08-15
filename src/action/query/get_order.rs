use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::GetOrderResp};

pub fn get_order(deps: Deps<ElysQuery>, order_id: u128) -> Result<GetOrderResp, ContractError> {
    let orders_list: Vec<Order> = ORDER.load(deps.storage)?;
    let have_order = orders_list.iter().find(|order| order.order_id == order_id);

    let resp = GetOrderResp {
        order: match have_order {
            Some(order) => order.to_owned(),
            None => return Err(ContractError::OrderNotFound { order_id: order_id }),
        },
    };

    Ok(resp)
}
