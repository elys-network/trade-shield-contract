use std::ops::Div;

use cosmwasm_std::{Decimal, Int128, SubMsg};

use crate::{
    bindings::{querier::ElysQuerier, query::ElysQuery},
    states::PROCESS_ORDER_EXECUTOR,
};

use super::*;

pub fn process_orders(
    deps: DepsMut<ElysQuery>,
    info: MessageInfo,
    env: Env,
) -> Result<Response<ElysMsg>, ContractError> {
    let process_order_executor = PROCESS_ORDER_EXECUTOR.load(deps.storage)?;

    if process_order_executor != info.sender {
        return Err(ContractError::ProcessOrderAuth {
            sender: info.sender,
        });
    }
    let orders = ORDER.load(deps.storage)?;
    let querier = ElysQuerier::new(&deps.querier);
    let mut submsgs: Vec<SubMsg<ElysMsg>> = vec![];

    for order in &orders {
        if check_order(order, &querier) {
            process_order(order, &mut submsgs, env.contract.address.to_string())
        }
    }

    Ok(Response::new().add_submessages(submsgs))
}

fn check_order(order: &Order, querier: &ElysQuerier) -> bool {
    let amm_swap_estimation =
        match querier.swap_estimation(&order.order_amm_routes, &order.order_amount) {
            Ok(res) => res,
            Err(_) => return false,
        };

    let order_spot_price = match order.order_amount.denom == order.order_price.base_denom {
        true => order.order_price.rate,
        false => Decimal::one().div(order.order_price.rate),
    };

    let order_token_out = order_spot_price * order.order_amount.amount;

    match order.order_type {
        OrderType::LimitBuy => order_token_out <= amm_swap_estimation.token_out.amount,

        OrderType::LimitSell => order_token_out <= amm_swap_estimation.token_out.amount,

        OrderType::StopLoss => order_token_out >= amm_swap_estimation.token_out.amount,
    }
}

fn process_order(order: &Order, submsgs: &mut Vec<SubMsg<ElysMsg>>, sender: String) {
    let token_out_min_amount: Int128 = match order.order_type {
        OrderType::LimitBuy => calculate_token_out_min_amount(order),
        OrderType::LimitSell => calculate_token_out_min_amount(order),
        OrderType::StopLoss => Int128::zero(),
    };

    let msg = ElysMsg::MsgSwapExactAmountIn {
        sender,
        routes: order.order_amm_routes.clone(),
        token_in: order.order_amount.clone(),
        token_out_min_amount,
    };

    submsgs.push(SubMsg::reply_on_success(msg, order.order_id))
}

fn calculate_token_out_min_amount(order: &Order) -> Int128 {
    let Order {
        order_amount,
        order_price,
        ..
    } = order;

    let amount = if order_amount.denom == order_price.base_denom {
        order_amount.amount * order_price.rate
    } else {
        order_amount.amount * Decimal::one().div(order_price.rate)
    };

    Int128::new(amount.u128() as i128)
}
