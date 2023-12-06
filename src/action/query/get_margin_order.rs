use super::*;

pub fn get_margin_order(
    deps: Deps<ElysQuery>,
    id: u64,
) -> Result<GetMarginOrderResp, ContractError> {
    let order = MARGIN_ORDER.may_load(deps.storage, id)?;

    match order {
        Some(order) => Ok(GetMarginOrderResp { order }),
        None => Err(ContractError::OrderNotFound { order_id: id }),
    }
}
