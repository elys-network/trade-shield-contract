use super::*;
use cosmwasm_std::{Coin, Decimal, StdResult};
use elys_bindings::query_resp::MarginOpenEstimationResponse;

pub fn margin_open_estimation(
    deps: Deps<ElysQuery>,
    position: MarginPosition,
    leverage: Decimal,
    trading_asset: String,
    collateral: Coin,
    take_profit_price: Decimal,
    _user_address: String,
) -> StdResult<MarginOpenEstimationResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    querier.margin_open_estimation(
        position,
        leverage,
        trading_asset,
        collateral,
        take_profit_price,
        Decimal::zero(),
    )
}
