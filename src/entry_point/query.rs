use super::*;
use msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query;
    use QueryMsg::*;

    match msg {
        GetSpotOrder { order_id } => Ok(to_json_binary(&query::get_spot_order(deps, order_id)?)?),
        GetAllPrices { limit } => Ok(to_json_binary(&query::get_all_prices(deps, limit)?)?),
        AssetInfo { denom } => Ok(to_json_binary(&query::asset_info(deps, denom)?)?),
        GetMarginPosition { address, id } => Ok(to_json_binary(&query::get_margin_position(
            deps, address, id,
        )?)?),
        GetMarginPositions { pagination } => Ok(to_json_binary(&query::get_margin_positions(
            deps, pagination,
        )?)?),
        GetSpotOrders {
            pagination,
            order_owner,
            order_type,
            order_status,
        } => Ok(to_json_binary(&query::get_spot_orders(
            deps,
            pagination,
            order_owner,
            order_type,
            order_status,
        )?)?),
        GetMarginOrders {
            pagination,
            order_owner,
            order_type,
            order_status,
        } => Ok(to_json_binary(&query::get_margin_orders(
            deps,
            pagination,
            order_owner,
            order_type,
            order_status,
        )?)?),
        SwapEstimationByDenom {
            amount,
            denom_in,
            denom_out,
            user_address,
        } => Ok(to_json_binary(&query::swap_estimation_by_denom(
            deps,
            amount,
            denom_in,
            denom_out,
            user_address,
        )?)?),
        GetMarginOrder { id } => Ok(to_json_binary(&query::get_margin_order(deps, id)?)?),
        MarginOpenEstimation {
            position,
            leverage,
            trading_asset,
            collateral,
            take_profit_price,
            user_address,
        } => Ok(to_json_binary(&query::margin_open_estimation(
            deps,
            position,
            leverage,
            trading_asset,
            collateral,
            take_profit_price,
            user_address,
        )?)?),
        MarginGetPositionsForAddress {
            address,
            pagination,
        } => Ok(to_json_binary(&query::margin_get_position_for_address(
            deps, address, pagination,
        )?)?),
    }
}
