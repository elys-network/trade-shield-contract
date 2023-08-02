use super::*;
use msg::ExecuteMsg;

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use action::execute::*;
    use ExecuteMsg::*;

    match msg {
        CreateOrder {
            order_type,
            order_price,
        } => create_order(env, deps, info, order_type, order_price),
        CancelOrder { order_id } => cancel_order(info, deps, order_id),
        ProcessOrder {} => process_order(deps, info),
    }
}
