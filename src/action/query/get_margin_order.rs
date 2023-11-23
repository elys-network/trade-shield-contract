use super::*;

pub fn get_margin_order(
    deps: Deps<ElysQuery>,
    id: u64,
) -> Result<GetMarginOrderResp, ContractError> {
    let orders = MARGIN_ORDER.load(deps.storage)?;

    match orders.iter().find(|order| order.order_id == id).cloned() {
        Some(order) => Ok(GetMarginOrderResp { order }),
        None => Err(ContractError::OrderNotFound { order_id: id }),
    }
}
