use super::*;
use crate::action::reply::*;
use cosmwasm_std::Reply;
use msg::ReplyType;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    msg: Reply,
) -> Result<Response<ElysMsg>, ContractError> {
    let reply = match msg.result.into_result() {
        Ok(reply) => reply,
        Err(err) => {
            return Err(ContractError::StdError(
                cosmwasm_std::StdError::GenericErr { msg: err },
            ));
        }
    };

    let data: Binary = match reply.data {
        Some(data) => data,
        None => {
            return Err(ContractError::StdError(
                cosmwasm_std::StdError::GenericErr {
                    msg: "no data".to_string(),
                },
            ))
        }
    };

    match ReplyType::from(msg.id)? {
        ReplyType::SpotOrder => reply_to_spot_order(deps, data),
        ReplyType::MarginOpenPosition => reply_to_create_margin_order(deps, data),
        _ => unimplemented!(),
    }
}
