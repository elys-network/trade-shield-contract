use crate::states::PROCESSED_SPOT_ORDER;

use super::*;

pub fn cancel_spot_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_id: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    let orders_list: Vec<SpotOrder> = SPOT_ORDER.load(deps.storage)?;
    let order: SpotOrder = match orders_list.iter().find(|order| order.order_id == order_id) {
        Some(order) => order.to_owned(),
        None => return Err(ContractError::SpotOrderNotFound { order_id }),
    };

    if order.owner_address != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let processed_spot_order = PROCESSED_SPOT_ORDER.load(deps.storage)?;
    for (id, _) in processed_spot_order {
        if id == order_id {
            return Err(ContractError::ProcessSpotOrderProcessing { order_id });
        }
    }

    let refund_msg = BankMsg::Send {
        to_address: order.owner_address.to_string(),
        amount: vec![order.order_amount],
    };

    let resp = Response::new().add_message(CosmosMsg::Bank(refund_msg));

    let new_orders_list: Vec<SpotOrder> = orders_list
        .into_iter()
        .filter(|order| order.order_id != order_id)
        .collect();

    SPOT_ORDER.save(deps.storage, &new_orders_list)?;

    Ok(resp)
}
