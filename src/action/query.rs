use crate::error::ContractError;
use crate::{
    msg::GetOrderResp,
    state::{Order, ORDER},
};
use cosmwasm_std::Deps;

pub fn get_order(deps: Deps, id: String) -> Result<GetOrderResp, ContractError> {
    let orders_list: Vec<Order> = ORDER.load(deps.storage)?;
    let have_order = orders_list.iter().find(|order| order.id == id);

    let resp = GetOrderResp {
        order: match have_order {
            Some(order) => order.to_owned(),
            None => return Err(ContractError::OrderNotFound{ order_id: id }),
        },
    };

    Ok(resp)
}
