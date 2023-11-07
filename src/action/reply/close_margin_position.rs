pub fn reply_to_close_margin_order(
    deps: DepsMut<ElysQuery>,
    data: Binary,
) -> Result<Response<ElysMsg>, ContractError> {
    let reply_msg: MsgOpenResponse = from_binary(&data)?;

    let meta_data = match reply_msg.meta_data {
        Some(meta_data) => meta_data,
        None => {
            return Err(ContractError::StdError(StdError::GenericErr {
                msg: "no metadata".to_string(),
            }))
        }
    };

    let mut order: MarginOrder = from_binary(&meta_data)?;
    order.order_id = reply_msg.id;

    let mut orders = MARGIN_ORDER.load(deps.storage)?;
    orders.push(order);

    let resp = Response::new().add_attribute("order_id", reply_msg.id.to_string());

    Ok(resp)
}
