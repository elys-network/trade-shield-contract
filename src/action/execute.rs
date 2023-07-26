use cosmwasm_std::{BankMsg, DepsMut, Env, MessageInfo, Response, CosmosMsg};

use crate::{
    error::ContractError,
    state::{Order, OrderType, ORDER},
};

pub fn cancel_order(
    info: MessageInfo,
    deps: DepsMut,
    id: String,
) -> Result<Response, ContractError> {
    let orders_list: Vec<Order> = ORDER.load(deps.storage)?;
    let order: Order = match orders_list.iter().find(|order| order.id == id) {
        Some(order) => order.to_owned(),
        None => return Err(ContractError::OrderNotFound { order_id: id }),
    };

    if order.user != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let refund_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![order.user_token],
    };

    let resp = Response::new().add_message(CosmosMsg::Bank(refund_msg));

    let new_orders_list: Vec<Order> = orders_list
        .into_iter()
        .filter(|order| order.id != id)
        .collect();

    ORDER.save(deps.storage, &new_orders_list)?;

    Ok(resp)
}

pub fn create_order(
    env: Env,
    deps: DepsMut,
    info: MessageInfo,
    order_type: OrderType,
    stop_price: u128,
    selling_denom: String,
) -> Result<Response, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::CoinNumber);
    };
    
    // let price_of_token : u128 = CosmosMsg::Custom();
    // match order_type {
    //     OrderType::StopLoss => if stop_price >= price_of_token {
    //         return Err(ContractError::StopPriceReached);
    //     },
    //     OrderType::TakeProfit => if stop_price <= price_of_token {
    //         return Err(ContractError::StopPriceReached);
    //     },
    //     _ => return Err(ContractError::OrderType { order_type: order_type })
    // };

    let new_order: Order = Order::new(order_type, stop_price, info.funds[0].clone(), info.sender.clone(), selling_denom);

    let bank_msg: BankMsg = BankMsg::Send {
        to_address: env.contract.address.to_string(),
        amount: info.funds.clone(),
    };

    cw_utils::must_pay(&info, &info.funds[0].denom)?;

    let resp = Response::new()
        .add_attribute("action", "create an order")
        .add_attribute("order_id", new_order.id.clone())
        .add_message(bank_msg);

    let mut order_vec = ORDER.load(deps.storage)?;

    order_vec.push(new_order);

    ORDER.save(deps.storage, &order_vec)?;
    Ok(resp)
}

pub fn execute_order(_deps: DepsMut, _info: MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::new())
}
