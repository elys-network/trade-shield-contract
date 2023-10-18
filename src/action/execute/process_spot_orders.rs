use crate::msg::ReplyType;
use cosmwasm_std::{to_binary, Decimal, Int128, SubMsg};
use std::ops::Div;

use crate::{
    bindings::{querier::ElysQuerier, query::ElysQuery},
    states::PROCESS_SPOT_ORDER_EXECUTOR,
};

use super::*;

pub fn process_spot_orders(
    deps: DepsMut<ElysQuery>,
    info: MessageInfo,
    env: Env,
) -> Result<Response<ElysMsg>, ContractError> {
    let process_order_executor = PROCESS_SPOT_ORDER_EXECUTOR.load(deps.storage)?;

    if process_order_executor != info.sender {
        return Err(ContractError::ProcessSpotOrderAuth {
            sender: info.sender,
        });
    }
    let orders = SPOT_ORDER.load(deps.storage)?;
    let querier = ElysQuerier::new(&deps.querier);
    let mut submsgs: Vec<SubMsg<ElysMsg>> = vec![];

    for order in &orders {
        if check_order(order, &querier) {
            process_order(order, &mut submsgs, env.contract.address.as_str())
        }
    }

    Ok(Response::new().add_submessages(submsgs))
}

fn check_order(order: &SpotOrder, querier: &ElysQuerier) -> bool {
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
        SpotOrderType::LimitBuy => order_token_out <= amm_swap_estimation.token_out.amount,

        SpotOrderType::LimitSell => order_token_out <= amm_swap_estimation.token_out.amount,

        SpotOrderType::StopLoss => order_token_out >= amm_swap_estimation.token_out.amount,
    }
}

fn process_order(order: &SpotOrder, submsgs: &mut Vec<SubMsg<ElysMsg>>, sender: &str) {
    let token_out_min_amount: Int128 = match order.order_type {
        SpotOrderType::LimitBuy => calculate_token_out_min_amount(order),
        SpotOrderType::LimitSell => calculate_token_out_min_amount(order),
        SpotOrderType::StopLoss => Int128::zero(),
    };

    let msg = ElysMsg::swap_exact_amount_in(
        sender,
        &order.order_amount,
        &order.order_amm_routes,
        token_out_min_amount,
        Some(to_binary(&order.order_id).unwrap()),
    );

    submsgs.push(SubMsg::reply_on_success(msg, ReplyType::SpotOrder as u64))
}

fn calculate_token_out_min_amount(order: &SpotOrder) -> Int128 {
    let SpotOrder {
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
