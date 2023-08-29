use crate::bindings::query::ElysQuery;

use super::*;

pub fn create_order(
    env: Env,
    deps: DepsMut<ElysQuery>,
    info: MessageInfo,
    order_type: OrderType,
    order_price: Coin,
    order_amm_routes: Vec<SwapAmountInRoute>,
) -> Result<Response, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::CoinNumber);
    };

    let mut order_vec = ORDER.load(deps.storage)?;

    let new_order: Order = Order::new(
        order_type,
        order_price,
        info.funds[0].clone(),
        info.sender.clone(),
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

    ORDER.save(deps.storage, &order_vec)?;
    Ok(resp)
}
