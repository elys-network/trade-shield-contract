use super::*;
use cosmwasm_std::{Coin, Decimal, Int128, StdError, StdResult};

pub fn create_margin_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    env: Env,
    position: MarginPosition,
    collateral: Coin,
    leverage: Decimal,
    borrow_asset: String,
    take_profit_price: Decimal,
    order_type: OrderType,
    trigger_price: Option<OrderPrice>,
) -> Result<Response<ElysMsg>, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::CoinNumber);
    }

    if collateral != info.funds[0] {
        return Err(ContractError::CollateralAmount);
    }

    if trigger_price.is_none() && order_type != OrderType::MarketBuy {
        return Err(StdError::not_found("order price").into());
    }

    if position == MarginPosition::Short && collateral.denom != "uusdc" {
        return Err(
            StdError::generic_err("the collateral asset for a short can only be UUSDC").into(),
        );
    }

    cw_utils::must_pay(&info, &info.funds[0].denom)?;

    let mut order_vec = MARGIN_ORDER.load(deps.storage)?;

    let order = MarginOrder::new(
        &position,
        &collateral,
        borrow_asset,
        &info.sender,
        &leverage,
        &take_profit_price,
        &order_type,
        &trigger_price,
        &order_vec,
    );

    let resp = create_response(&order, env.contract.address)?;

    if order.order_type != OrderType::MarketBuy {
        order_vec.push(order);

        MARGIN_ORDER.save(deps.storage, &order_vec)?;
    }

    Ok(resp)
}

fn create_response(
    order: &MarginOrder,
    contract_addr: impl Into<String>,
) -> StdResult<Response<ElysMsg>> {
    if order.order_type != OrderType::MarketBuy {
        return Ok(Response::new().add_attribute("order_id", order.order_id.to_string()));
    }

    let msg: ElysMsg = ElysMsg::margin_broker_open_position(
        contract_addr,
        &order.collateral.denom,
        Int128::new(order.collateral.amount.u128() as i128),
        &order.borrow_asset,
        order.position.clone() as i32,
        order.leverage,
        order.take_profit_price,
        &order.owner,
    );

    Ok(Response::new().add_message(msg))
}
