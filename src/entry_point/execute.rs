use super::*;
use msg::ExecuteMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ElysQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ElysMsg>, ContractError> {
    use action::execute::*;
    use ExecuteMsg::*;

    match msg {
        CreateSpotOrder {
            order_type,
            order_source_denom,
            order_target_denom,
            order_price,
            order_amm_routes,
        } => create_spot_order(
            env,
            deps,
            info,
            order_type,
            order_source_denom,
            order_target_denom,
            order_price,
            order_amm_routes,
        ),
        CancelSpotOrder { order_id } => cancel_spot_order(info, deps, order_id),

        CancelSpotOrders {
            order_ids,
            owner_address,
            order_type,
        } => cancel_spot_orders(info, deps, order_ids, owner_address, order_type),

        ProcessSpotOrders {} => process_spot_orders(deps, info, env),
        CreateMarginOrder {
            position,
            collateral,
            leverage,
            borrow_asset,
            take_profit_price,
            order_type,
        } => create_margin_order(
            info,
            deps,
            env,
            position,
            collateral,
            leverage,
            borrow_asset,
            take_profit_price,
            order_type,
        ),
        CancelMarginOrder { order_id } => cancel_margin_order(info, deps, order_id),
    }
}
