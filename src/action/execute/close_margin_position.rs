use super::*;

pub fn close_margin_position(
    info: MessageInfo,
    env: Env,
    id: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::margin_broker_close_position(env.contract.address, id, info.sender);

    let resp = Response::new().add_message(CosmosMsg::Custom(msg));

    Ok(resp)
}
