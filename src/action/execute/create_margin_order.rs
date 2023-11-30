use crate::msg::ReplyType;

use super::*;
use cosmwasm_std::{to_json_binary, Decimal, Int128, StdError, StdResult, SubMsg};
use cw_utils;
use MarginOrderType::*;

pub fn create_margin_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    env: Env,
    position: Option<MarginPosition>,
    leverage: Option<Decimal>,
    borrow_asset: Option<String>,
    take_profit_price: Option<Decimal>,
    order_type: MarginOrderType,
    trigger_price: Option<OrderPrice>,
    position_id: Option<u64>,
) -> Result<Response<ElysMsg>, ContractError> {
    check_order_type(
        &position,
        &leverage,
        &borrow_asset,
        &take_profit_price,
        &order_type,
        &trigger_price,
        &position_id,
    )?;

    if order_type == LimitOpen || order_type == MarketOpen {
        create_margin_open_order(
            info,
            deps,
            env,
            order_type,
            position.unwrap(),
            borrow_asset.unwrap(),
            leverage.unwrap(),
            take_profit_price.unwrap(),
            trigger_price,
        )
    } else {
        create_margin_close_order(
            info,
            deps,
            env,
            order_type,
            position_id.unwrap(),
            trigger_price,
        )
    }
}

fn check_order_type(
    position: &Option<MarginPosition>,
    leverage: &Option<Decimal>,
    borrow_asset: &Option<String>,
    take_profit_price: &Option<Decimal>,
    order_type: &MarginOrderType,
    trigger_price: &Option<OrderPrice>,
    position_id: &Option<u64>,
) -> StdResult<()> {
    let mut not_found: Vec<&str> = vec![];

    if order_type != &MarketOpen && order_type != &MarketClose && trigger_price.is_none() {
        not_found.push("trigger price");
    }

    if (order_type == &LimitClose || order_type == &MarketClose || order_type == &StopLoss)
        && position_id.is_none()
    {
        not_found.push("position id");
    }

    if order_type == &LimitOpen || order_type == &MarketOpen {
        if position.is_none() {
            not_found.push("position");
        }
        if leverage.is_none() {
            not_found.push("leverage");
        }
        if borrow_asset.is_none() {
            not_found.push("borrow asset");
        }
        if take_profit_price.is_none() {
            not_found.push("take profit price");
        }
    }

    if not_found.is_empty() {
        Ok(())
    } else {
        let missing_fields = not_found.join(", ");
        Err(StdError::generic_err(format!(
            "Missing fields: {}",
            missing_fields
        )))
    }
}

fn create_margin_open_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    env: Env,
    order_type: MarginOrderType,
    position: MarginPosition,
    borrow_asset: String,
    leverage: Decimal,
    take_profit_price: Decimal,
    trigger_price: Option<OrderPrice>,
) -> Result<Response<ElysMsg>, ContractError> {
    let collateral = cw_utils::one_coin(&info)?;
    let mut orders = MARGIN_ORDER.load(deps.storage)?;

    if position == MarginPosition::Unspecified {
        return Err(StdError::generic_err("margin position cannot be set at: Unspecified").into());
    }

    //TODO: include query to test if the collateral is valid

    let order = MarginOrder::new_open(
        &info.sender,
        &position,
        &order_type,
        &collateral,
        &borrow_asset,
        &leverage,
        &take_profit_price,
        &trigger_price,
        &orders,
    );

    let order_id = order.order_id;

    orders.push(order);

    MARGIN_ORDER.save(deps.storage, &orders)?;

    let resp = Response::new()
        .add_attribute("event_type", "create_margin_open_order")
        .add_attribute("margin_order_id", order_id.to_string());

    if order_type != MarketOpen {
        return Ok(resp);
    }

    let msg = ElysMsg::margin_broker_open_position(
        env.contract.address,
        collateral.denom,
        Int128::new(collateral.amount.u128() as i128),
        borrow_asset,
        position as i32,
        leverage,
        take_profit_price,
        info.sender,
    );

    let mut reply_infos = REPLY_INFO.load(deps.storage)?;

    let info_id = if let Some(max_info) = reply_infos.iter().max_by_key(|info| info.id) {
        max_info.id + 1
    } else {
        0
    };

    reply_infos.push(ReplyInfo {
        id: info_id,
        reply_type: ReplyType::MarginBrokerMarketOpen,
        data: Some(to_json_binary(&order_id)?),
    });

    REPLY_INFO.save(deps.storage, &reply_infos)?;

    let sub_msg = SubMsg::reply_always(msg, info_id);

    Ok(resp.add_submessage(sub_msg))
}

fn create_margin_close_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    env: Env,
    order_type: MarginOrderType,
    position_id: u64,
    trigger_price: Option<OrderPrice>,
) -> Result<Response<ElysMsg>, ContractError> {
    cw_utils::nonpayable(&info)?;

    let querier = ElysQuerier::new(&deps.querier);

    let mtp_resp = querier.mtp(info.sender.to_string(), position_id)?;

    let mtp = if let Some(mtp) = mtp_resp.mtp {
        mtp
    } else {
        return Err(StdError::not_found("margin trading position").into());
    };

    let mut orders = MARGIN_ORDER.load(deps.storage)?;

    if orders
        .iter()
        .find(|order| order.position_id == Some(position_id) && order.status != Status::Canceled)
        .is_some()
    {
        return Err(StdError::generic_err("this position had an order already assigned").into());
    };

    let order = MarginOrder::new_close(
        &info.sender,
        mtp.position,
        &order_type,
        &mtp.collaterals[0],
        &mtp.custodies[0].denom,
        &mtp.leverages[0],
        position_id,
        &trigger_price,
        &mtp.take_profit_price,
        &orders,
    )?;

    let order_id = order.order_id;

    orders.push(order);

    MARGIN_ORDER.save(deps.storage, &orders)?;

    let resp = Response::new()
        .add_attribute("event_type", "create_margin_close_order")
        .add_attribute("margin_order_id", order_id.to_string());

    if order_type != MarketClose {
        return Ok(resp);
    }

    let mut reply_infos = REPLY_INFO.load(deps.storage)?;

    let msg =
        ElysMsg::margin_broker_close_position(env.contract.address, position_id, &info.sender);

    let info_id = if let Some(max_info) = reply_infos.iter().max_by_key(|info| info.id) {
        max_info.id + 1
    } else {
        0
    };

    reply_infos.push(ReplyInfo {
        id: info_id,
        reply_type: ReplyType::MarginBrokerMarketClose,
        data: Some(to_json_binary(&order_id)?),
    });

    REPLY_INFO.save(deps.storage, &reply_infos)?;

    let sub_msg = SubMsg::reply_always(msg, info_id);

    Ok(resp.add_submessage(sub_msg))
}
