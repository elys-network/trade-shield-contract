use cosmwasm_std::{Int128, SubMsg, Uint128};

use crate::{bindings::query::ElysQuery, states::PROCESS_ORDER_EXECUTOR};

use super::*;

pub fn process_orders(
    deps: DepsMut<ElysQuery>,
    info: MessageInfo,
    env: Env,
) -> Result<Response<ElysMsg>, ContractError> {
    println!("AAAAAA");
    let process_order_executor = PROCESS_ORDER_EXECUTOR.load(deps.storage)?;
    println!("OK");
    if process_order_executor != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }
    let orders = ORDER.load(deps.storage)?;
    let mut submsgs: Vec<SubMsg<ElysMsg>> = vec![];

    for order in &orders {
        if check_order(order) {
            process_order(order, &mut submsgs, env.contract.address.to_string())
        }
    }

    Ok(Response::new().add_submessages(submsgs))
}

fn check_order(order: &Order) -> bool {
    let amm_price_rate: Uint128 = todo!("implent amm query module"); // implement get price here

    match order.order_type {
        OrderType::LimitBuy => {
            order.order_amount.denom == order.order_price.base_denom
                && amm_price_rate >= order.order_price.rate
                || order.order_amount.denom != order.order_price.base_denom
                    && amm_price_rate <= order.order_price.rate
        }

        OrderType::LimitSell => {
            order.order_amount.denom == order.order_price.base_denom
                && amm_price_rate >= order.order_price.rate
                || order.order_amount.denom != order.order_price.base_denom
                    && amm_price_rate <= order.order_price.rate
        }
        OrderType::StopLoss => {
            order.order_amount.denom == order.order_price.base_denom
                && amm_price_rate <= order.order_price.rate
                || order.order_amount.denom != order.order_price.base_denom
                    && amm_price_rate >= order.order_price.rate
        }
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
        order_amount.amount / order_price.rate
    };
    Int128::new(amount.u128() as i128)
}
