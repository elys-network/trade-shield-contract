use crate::bindings::query::ElysQuery;

use super::*;

pub fn create_order(
    env: Env,
    deps: DepsMut<ElysQuery>,
    info: MessageInfo,
    order_type: SpotOrderType,
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
        &info.funds[0].denom,
    )?;

    let mut order_vec = SPOT_ORDER.load(deps.storage)?;

    let new_order: SpotOrder = SpotOrder::new(
        order_type,
        order_price,
        info.funds[0].clone(),
        info.sender.clone(),
        order_target_denom,
        order_amm_routes,
        &order_vec,
    );

    let bank_msg: BankMsg = BankMsg::Send {
        to_address: env.contract.address.to_string(),
        amount: info.funds.clone(),
    };

    cw_utils::must_pay(&info, &info.funds[0].denom)?;

    let resp = Response::new()
        .add_attribute("order_id", new_order.order_id.to_string())
        .add_message(bank_msg); // information message

    order_vec.push(new_order);

    SPOT_ORDER.save(deps.storage, &order_vec)?;
    Ok(resp)
}

fn check_denom_error(
    order_source_denom: &str,
    order_target_denom: &str,
    order_price: &SpotOrderPrice,
    funds_send_denom: &str,
) -> Result<(), ContractError> {
    if order_source_denom != funds_send_denom {
        return Err(ContractError::SpotOrderWrongFund);
    }

    if order_source_denom == order_target_denom {
        return Err(ContractError::SpotOrderSameDenom);
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
