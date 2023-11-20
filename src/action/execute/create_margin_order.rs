use cosmwasm_std::{to_json_binary, Coin, Decimal, Int128, StdError, SubMsg};

use crate::msg::ReplyType;

use super::*;

pub fn create_margin_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    position: MarginPosition,
    collateral: Coin,
    leverage: Decimal,
    borrow_asset: String,
    take_profit_price: Decimal,
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

    let meta_data = to_json_binary(&MarginOrder::new(
        position.clone(),
        &info.sender,
        collateral.clone(),
        leverage,
        borrow_token,
        take_profit_price,
    ))?;

    let sub_msg = ElysMsg::margin_open_position(
        &info.sender,
        &collateral.denom,
        Int128::from(collateral.amount.u128() as i128),
        &borrow_asset,
        position,
        leverage,
        take_profit_price,
    );

    let mut reply_info = REPLY_INFO.load(deps.storage)?;

    let new_info_id = match reply_info.iter().max_by_key(|info| info.id) {
        Some(max_info) => max_info.id + 1,
        None => 0,
    };

    reply_info.push(ReplyInfo {
        id: new_info_id,
        reply_type: ReplyType::MarginOpenPosition,
        data: Some(meta_data),
    });

    REPLY_INFO.save(deps.storage, &reply_info)?;

    Ok(Response::new().add_submessage(SubMsg::reply_always(sub_msg, new_info_id)))
}
