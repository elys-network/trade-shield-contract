use super::*;

pub fn get_spot_order(
    deps: Deps<ElysQuery>,
    order_id: u64,
) -> Result<GetSpotOrderResp, ContractError> {
    let orders_list: Vec<SpotOrder> = SPOT_ORDER.load(deps.storage)?;
    let have_order = orders_list.iter().find(|order| order.order_id == order_id);

    let resp = GetSpotOrderResp {
        order: match have_order {
            Some(order) => order.to_owned(),
            None => return Err(ContractError::SpotOrderNotFound { order_id }),
        },
    };

    Ok(resp)
}
