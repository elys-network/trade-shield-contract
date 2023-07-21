use crate::error::ContractError;
use crate::{
    msg::{AllOrdersResp, GetOrderResp},
    state::{Order, ORDER},
};
use cosmwasm_std::{Deps, Response, StdResult};

pub fn get_order(deps: Deps, id: String) -> Response<GetOrderResp, ContractError> {
    let orders_list: Vec<Order> = ORDER.load(deps.storage)?;
    let have_order: Option<Order> = orders_list.iter().find(|order| order.order_id == id);

    let resp = GetOrderResp {
        order: match have_order {
            Some(order) => order,
            None => return Err(ContractError::OrderMissing { order_id: id }),
        },
    };

    Ok(resp)
}
