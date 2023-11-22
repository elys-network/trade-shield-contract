use super::*;
use crate::msg::ReplyType;
use cosmwasm_std::{to_json_binary, Coin, Decimal, Int128, StdError, StdResult, Storage, SubMsg};

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
) -> Result<Response<ElysMsg>, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::CoinNumber);
    }

    if collateral != info.funds[0] {
        return Err(ContractError::CollateralAmount);
    }

    if position == MarginPosition::Short && collateral.denom == "uusdc" {
        return Err(
            StdError::generic_err("the collateral asset for a short can only be USDC").into(),
        );
    }

    cw_utils::must_pay(&info, &info.funds[0].denom)?;

    let borrow_token = Coin {
        denom: borrow_asset.clone(),
        amount: (leverage - Decimal::one()) * collateral.amount,
    };

    let mut order_vec = MARGIN_ORDER.load(deps.storage)?;

    let order = MarginOrder::new(
        &position,
        &collateral,
        borrow_asset,
        &info.sender,
        &leverage,
        &take_profit_price,
        &order_type,
        &order_vec,
    );

    let resp = create_response(deps.storage, &order, env.contract.address)?;

    order_vec.push(order);

    MARGIN_ORDER.save(deps.storage, &order_vec)?;

    Ok(resp)
}

fn create_response(
    storage: &mut dyn Storage,
    order: &MarginOrder,
    contract_addr: impl Into<String>,
) -> StdResult<Response<ElysMsg>> {
    let mut resp: Response<ElysMsg> =
        Response::new().add_attribute("order_id", order.order_id.to_string());

    if order.order_type == OrderType::MarketBuy {
        return Ok(resp);
    }

    let mut reply_infos = REPLY_INFO.load(storage)?;

    let reply_info_id = match reply_infos.iter().max_by_key(|info| info.id) {
        Some(info) => info.id + 1,
        None => 0,
    };

    let reply_info = ReplyInfo {
        id: reply_info_id,
        reply_type: ReplyType::MarginOpenPosition,
        data: Some(to_json_binary(&order.order_id)?),
    };

    reply_infos.push(reply_info);

    let submsg: SubMsg<ElysMsg> = SubMsg::reply_always(
        ElysMsg::margin_broker_open_position(
            contract_addr,
            &order.collateral.denom,
            Int128::new(order.collateral.amount.u128() as i128),
            &order.borrow_asset,
            order.position.clone() as i32,
            order.leverage,
            order.take_profit_price,
            &order.owner,
        ),
        reply_info_id,
    );

    Ok(resp)
}
