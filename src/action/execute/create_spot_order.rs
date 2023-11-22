use cosmwasm_std::{to_json_binary, Int128, StdResult, Storage, SubMsg};

use crate::msg::ReplyType;

use super::*;

pub fn create_spot_order(
    env: Env,
    deps: DepsMut<ElysQuery>,
    info: MessageInfo,
    order_type: OrderType,
    order_source_denom: String,
    order_target_denom: String,
    order_price: SpotOrderPrice,
    order_amm_routes: Vec<SwapAmountInRoute>,
) -> Result<Response<ElysMsg>, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::CoinNumber);
    };

    check_denom_error(
        &order_source_denom,
        &order_target_denom,
        &order_price,
        &order_type,
        &info.funds[0].denom,
    )?;

    let mut order_vec = SPOT_ORDER.load(deps.storage)?;

    let new_order: SpotOrder = SpotOrder::new(
        order_type.clone(),
        order_price,
        info.funds[0].clone(),
        info.sender.clone(),
        order_target_denom,
        order_amm_routes.clone(),
        &order_vec,
    );

    let bank_msg: BankMsg = BankMsg::Send {
        to_address: env.contract.address.to_string(),
        amount: info.funds.clone(),
    };

    cw_utils::must_pay(&info, &info.funds[0].denom)?;

    let resp = create_resp(
        env.contract.address.as_str(),
        &new_order,
        bank_msg,
        deps.storage,
    )?;

    order_vec.push(new_order);
    SPOT_ORDER.save(deps.storage, &order_vec)?;

    Ok(resp)
}

fn check_denom_error(
    order_source_denom: &str,
    order_target_denom: &str,
    order_price: &SpotOrderPrice,
    order_type: &OrderType,
    funds_send_denom: &str,
) -> Result<(), ContractError> {
    if order_source_denom != funds_send_denom {
        return Err(ContractError::SpotOrderWrongFund);
    }

    if order_source_denom == order_target_denom {
        return Err(ContractError::SpotOrderSameDenom);
    }

    if order_type == &OrderType::MarketBuy {
        return Ok(());
    }

    if (order_price.base_denom != order_source_denom
        && order_price.base_denom != order_target_denom)
        || (order_price.quote_denom != order_source_denom
            && order_price.quote_denom != order_target_denom)
    {
        return Err(ContractError::SpotOrderPriceDenom);
    }

    Ok(())
}

fn create_resp(
    sender: &str,
    new_order: &SpotOrder,
    bank_msg: BankMsg,
    storage: &mut dyn Storage,
) -> StdResult<Response<ElysMsg>> {
    let resp = Response::new()
        .add_attribute("order_id", new_order.order_id.to_string())
        .add_message(bank_msg); // information message

    if new_order.order_type != OrderType::MarketBuy {
        return Ok(resp);
    }

    let mut reply_infos = REPLY_INFO.load(storage)?;

    let swap_msg = ElysMsg::amm_swap_exact_amount_in(
        sender,
        &new_order.order_amount,
        &new_order.order_amm_routes,
        Int128::zero(),
    );

    let info_id = if let Some(max_info) = reply_infos.iter().max_by_key(|info| info.id) {
        max_info.id + 1
    } else {
        0
    };

    reply_infos.push(ReplyInfo {
        id: info_id,
        reply_type: ReplyType::SpotOrder,
        data: Some(to_json_binary(&new_order.order_id)?),
    });

    REPLY_INFO.save(storage, &reply_infos)?;

    let sub_msg = SubMsg::reply_always(swap_msg, info_id);

    Ok(resp.add_submessage(sub_msg))
}
