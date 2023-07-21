use cosmwasm_std::{coins, Addr, BankMsg, Coin, DepsMut, Event, MessageInfo, Response, StdResult};
use uuid::Uuid;

use crate::{
    error::ContractError,
    state::{Order, OrderType, ORDER},
};

use super::query::get_order;

pub fn cancel_order(
    info: MessageInfo,
    deps: DepsMut,
    id: String,
) -> Result<Response, ContractError> {
    let orders_list: Vec<Order> = ORDER.load(deps.storage)?;
    let order: Order = match orders_list.iter().find(|order| order.order_id == id) {
        Some(order) => order,
        None => return Err(ContractError::OrderNotFound { order_id: id }),
    };

    if order.user != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let new_orders_list: Vec<Order> = orders_list
        .into_iter()
        .filter(|order| order.id != id)
        .collect();

    ORDER.save(deps.storage, &new_orders_list)?;

    Ok(Response::new())
}

pub fn create_order(
    deps: DepsMut,
    info: MessageInfo,
    order_type: String,
    stop_price: u128,
    selling_denom: String,
) -> Result<Response, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::CoinNumber);
    }

    // let price_of_token : u128 = GETPRICEOFTOKEN()?; //not a real thingbtw
    // match order_type {
    //     OrderType::StopLoss => if stop_price >= price_of_token {
    //         return Err(ContractError::StopPriceReached);
    //     },
    //     OrderType::TakeProfit => if stop_price <= price_of_token {
    //         return Err(ContractError::StopPriceReached);
    //     },
    //     _ => return Err(ContractError::OrderType { order_type: order_type })
    // };

    let new_order = Order {
        order_type: order_type,
        stop_price: stop_price,
        user_token: info.funds[0],
        selling_denom: selling_denom,
        user: info.sender,
        id: "the_id".to_owned(),
    };

    let event = Event::new("order_added").add_attribute("order", new_order);

    let resp = Response::new()
        .add_event(event)
        .add_attribute("order_type", new_order.order_type.clone());

    let action: StdResult<Vec<Order>> = |list: Vec<Order>| {
        list = list.append(vec![new_order]);
        Ok(list)
    };

    ORDER.update(deps.storage, action)?;

    Ok(resp)
}


pub fn execute_order(_deps: DepsMut, _info : MessageInfo) -> Result<Response, ContractError>{

    Ok(Response::new())
}