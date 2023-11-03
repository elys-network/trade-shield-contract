use cosmwasm_std::{to_binary, Coin, Decimal, Int128, SubMsg};

use crate::msg::ReplyType;

use super::*;

pub fn create_margin_order(
    info: MessageInfo,
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

    cw_utils::must_pay(&info, &info.funds[0].denom)?;

    let borrow_token = Coin {
        denom: borrow_asset.clone(),
        amount: (leverage - Decimal::one()) * collateral.amount,
    };

    let meta_data = to_binary(&MarginOrder::new(
        position.clone(),
        &info.sender,
        collateral.clone(),
        leverage,
        borrow_token,
        take_profit_price,
    ))?;

    let sub_msg = ElysMsg::open_position(
        &info.sender,
        &collateral.denom,
        Int128::from(collateral.amount.u128() as i128),
        &borrow_asset,
        position,
        leverage,
        take_profit_price,
        Some(meta_data),
    );

    Ok(Response::new().add_submessage(SubMsg::reply_always(
        sub_msg,
        ReplyType::MarginOpenPosition as u64,
    )))
}
