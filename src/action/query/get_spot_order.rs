use super::*;

pub fn get_spot_order(
    deps: Deps<ElysQuery>,
    order_id: u64,
) -> Result<GetSpotOrderResp, ContractError> {
    let order = SPOT_ORDER.may_load(deps.storage, order_id)?;

    let resp = GetSpotOrderResp {
        order: match order {
            Some(order) => order.to_owned(),
            None => return Err(ContractError::OrderNotFound { order_id }),
        },
    };

    Ok(resp)
}
