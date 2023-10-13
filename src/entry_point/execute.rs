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
        CreateOrder {
            order_type,
            order_source_denom,
            order_target_denom,
            order_price,
            order_amm_routes,
        } => create_order(
            env,
            deps,
            info,
            order_type,
            order_source_denom,
            order_target_denom,
            order_price,
            order_amm_routes,
        ),
        CancelOrder { order_id } => cancel_order(info, deps, order_id),
        ProcessOrders {} => process_orders(deps, info, env),
    }
}
