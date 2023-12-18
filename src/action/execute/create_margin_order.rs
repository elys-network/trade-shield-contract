use crate::msg::ReplyType;

use super::*;
use cosmwasm_std::{
    to_json_binary, Decimal, OverflowError, OverflowOperation, StdError, StdResult, SubMsg,
};
use cw_utils;
use MarginOrderType::*;

pub fn create_margin_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    position: Option<MarginPosition>,
    leverage: Option<Decimal>,
    trading_asset: Option<String>,
    take_profit_price: Option<Decimal>,
    order_type: MarginOrderType,
    trigger_price: Option<OrderPrice>,
    position_id: Option<u64>,
) -> Result<Response<ElysMsg>, ContractError> {
    check_order_type(
        &position,
        &leverage,
        &trading_asset,
        &take_profit_price,
        &order_type,
        &trigger_price,
        &position_id,
    )?;

    if order_type == LimitOpen || order_type == MarketOpen {
        create_margin_open_order(
            info,
            deps,
            order_type,
            position.unwrap(),
            trading_asset.unwrap(),
            leverage.unwrap(),
            take_profit_price.unwrap(),
            trigger_price,
        )
    } else {
        create_margin_close_order(info, deps, order_type, position_id.unwrap(), trigger_price)
    }
}

fn check_order_type(
    position: &Option<MarginPosition>,
    leverage: &Option<Decimal>,
    trading_asset: &Option<String>,
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
        if trading_asset.is_none() {
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
    order_type: MarginOrderType,
    position: MarginPosition,
    trading_asset: String,
    leverage: Decimal,
    take_profit_price: Decimal,
    trigger_price: Option<OrderPrice>,
) -> Result<Response<ElysMsg>, ContractError> {
    let collateral = cw_utils::one_coin(&info)?;

    let orders: Vec<MarginOrder> = MARGIN_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok().map(|r| r.1))
        .collect();

    if position == MarginPosition::Unspecified {
        return Err(StdError::generic_err("margin position cannot be set at: Unspecified").into());
    }

    let querrier = ElysQuerier::new(&deps.querier);

    let open_estimation = querrier.margin_open_estimation(
        position.clone(),
        leverage.clone(),
        &trading_asset,
        collateral.clone(),
        take_profit_price.clone(),
        Decimal::zero(),
    )?;

    if !open_estimation.valid_collateral {
        return Err(StdError::generic_err("not valid collateral").into());
    }

    if let Some(price) = &trigger_price {
        if price.rate.is_zero() {
            return Err(StdError::generic_err("trigger_price: The rate cannot be zero").into());
        }
    }

    let order = MarginOrder::new_open(
        &info.sender,
        &position,
        &order_type,
        &collateral,
        &trading_asset,
        &leverage,
        &take_profit_price,
        &trigger_price,
        &orders,
    )?;

    let order_id = order.order_id;

    MARGIN_ORDER.save(deps.storage, order_id, &order)?;

    let resp = Response::new().add_event(
        Event::new("create_margin_open_order")
            .add_attribute("margin_order_id", order_id.to_string()),
    );

    if order_type != MarketOpen {
        return Ok(resp);
    }

    let msg = ElysMsg::margin_open_position(
        info.sender,
        collateral,
        trading_asset,
        position,
        leverage,
        take_profit_price,
    );

    let reply_info_max_id = MAX_REPLY_ID.load(deps.storage)?;

    let reply_id = match reply_info_max_id.checked_add(1) {
        Some(id) => id,
        None => {
            return Err(StdError::overflow(OverflowError::new(
                OverflowOperation::Add,
                "reply_info_max_id",
                "increment one",
            ))
            .into())
        }
    };
    MAX_REPLY_ID.save(deps.storage, &reply_id)?;

    let reply_info = ReplyInfo {
        id: reply_id,
        reply_type: ReplyType::MarginBrokerMarketOpen,
        data: Some(to_json_binary(&order_id)?),
    };

    REPLY_INFO.save(deps.storage, reply_id, &reply_info)?;

    let sub_msg = SubMsg::reply_always(msg, reply_id);

    Ok(resp.add_submessage(sub_msg))
}

fn create_margin_close_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,

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

    let orders: Vec<MarginOrder> = MARGIN_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok().map(|r| r.1))
        .collect();

    if orders
        .iter()
        .find(|order| order.position_id == Some(position_id) && order.status != Status::Canceled)
        .is_some()
    {
        return Err(StdError::generic_err("this position had an order already assigned").into());
    };

    if let Some(price) = &trigger_price {
        if price.rate.is_zero() {
            return Err(StdError::generic_err("trigger_price: The rate cannot be zero").into());
        }
    }

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

    MARGIN_ORDER.save(deps.storage, order_id, &order)?;

    let resp = Response::new().add_event(
        Event::new("create_margin_close_order")
            .add_attribute("margin_order_id", order_id.to_string()),
    );

    if order_type != MarketClose {
        return Ok(resp);
    }

    let msg = ElysMsg::margin_close_position(
        &info.sender,
        position_id,
        mtp.custodies[0].amount.u128() as i128,
    );

    let reply_info_max_id = MAX_REPLY_ID.load(deps.storage)?;

    let reply_id = match reply_info_max_id.checked_add(1) {
        Some(id) => id,
        None => {
            return Err(StdError::overflow(OverflowError::new(
                OverflowOperation::Add,
                "reply_info_max_id",
                "increment one",
            ))
            .into())
        }
    };
    MAX_REPLY_ID.save(deps.storage, &reply_id)?;

    let reply_info = ReplyInfo {
        id: reply_id,
        reply_type: ReplyType::MarginBrokerMarketClose,
        data: Some(to_json_binary(&order_id)?),
    };

    REPLY_INFO.save(deps.storage, reply_id, &reply_info)?;

    let sub_msg = SubMsg::reply_always(msg, reply_id);

    Ok(resp.add_submessage(sub_msg))
}
