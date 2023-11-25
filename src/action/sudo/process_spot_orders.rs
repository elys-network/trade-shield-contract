use crate::{msg::ReplyType, states::PROCESSED_SPOT_ORDER};
use cosmwasm_std::{to_json_binary, Decimal, Int128, StdResult, Storage, SubMsg};
use std::ops::Div;

use super::*;

pub fn process_spot_orders(
    deps: DepsMut<ElysQuery>,
    env: Env,
) -> Result<Response<ElysMsg>, ContractError> {
    let mut orders = SPOT_ORDER.load(deps.storage)?;
    let mut reply_info = REPLY_INFO.load(deps.storage)?;

    let (send_msg, processed_order_ids) = send_token(&mut orders, deps.storage)?;

    let querier = ElysQuerier::new(&deps.querier);
    let mut submsgs: Vec<SubMsg<ElysMsg>> = vec![];

    for order in &orders {
        if check_order(order, &querier) {
            process_order(
                order,
                &mut submsgs,
                env.contract.address.as_str(),
                &mut reply_info,
            )?;
        }
    }

    REPLY_INFO.save(deps.storage, &reply_info)?;

    let mut resp = Response::new().add_submessages(submsgs);
    if !send_msg.is_empty() {
        resp = resp
            .add_messages(send_msg)
            .add_attribute("spot_order_processed", format!("{:?}", processed_order_ids));
    }

    Ok(resp)
}

fn send_token(
    unprocessed_orders: &mut Vec<SpotOrder>,
    store: &mut dyn Storage,
) -> StdResult<(Vec<BankMsg>, Vec<u64>)> {
    let process_spot_orders = PROCESSED_SPOT_ORDER.load(store)?;
    let processed_order_ids: Vec<u64> = process_spot_orders
        .iter()
        .map(|&(processed_order_id, _)| processed_order_id)
        .collect();
    let bank_msgs: Vec<BankMsg> = process_spot_orders
        .iter()
        .map(|(_, bank_msg)| bank_msg.to_owned())
        .collect();

    unprocessed_orders.retain(|order| !processed_order_ids.contains(&order.order_id));

    PROCESSED_SPOT_ORDER.save(store, &vec![])?;
    Ok((bank_msgs, processed_order_ids))
}

fn check_order(order: &SpotOrder, querier: &ElysQuerier) -> bool {
    if order.order_type == OrderType::MarketBuy {
        return true;
    }

    let amm_swap_estimation =
        match querier.amm_swap_estimation(&order.order_amm_routes, &order.order_amount) {
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
        _ => false,
    }
}

fn process_order(
    order: &SpotOrder,
    submsgs: &mut Vec<SubMsg<ElysMsg>>,
    sender: &str,
    reply_infos: &mut Vec<ReplyInfo>,
) -> StdResult<()> {
    let token_out_min_amount: Int128 = match order.order_type {
        OrderType::LimitBuy => calculate_token_out_min_amount(order),
        OrderType::LimitSell => calculate_token_out_min_amount(order),
        OrderType::StopLoss => Int128::zero(),
        OrderType::MarketBuy => Int128::zero(),
    };

    let msg = ElysMsg::amm_swap_exact_amount_in(
        sender,
        &order.order_amount,
        &order.order_amm_routes,
        token_out_min_amount,
        Decimal::zero(),
    );

    let info_id = if let Some(max_info) = reply_infos.iter().max_by_key(|info| info.id) {
        max_info.id + 1
    } else {
        0
    };
    reply_infos.push(ReplyInfo {
        id: info_id,
        reply_type: ReplyType::SpotOrder,
        data: Some(to_json_binary(&order.order_id)?),
    });
    submsgs.push(SubMsg::reply_on_success(msg, info_id));
    Ok(())
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

    Int128::new((amount.u128() - 1) as i128) //slippage integration
}
