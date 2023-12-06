use super::*;
use crate::{action::reply::*, states::REPLY_INFO};
use cosmwasm_std::Reply;
use msg::ReplyType;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    msg: Reply,
) -> Result<Response<ElysMsg>, ContractError> {
    let module_resp = msg.result;
    let info = REPLY_INFO.load(deps.storage, msg.id)?;

    REPLY_INFO.remove(deps.storage, msg.id);

    match info.reply_type {
        ReplyType::SpotOrder => reply_to_spot_order(deps, info.data, module_resp),
        ReplyType::MarginBrokerMarketOpen => {
            reply_to_create_margin_market_open(deps, info.data, module_resp)
        }

        ReplyType::MarginBrokerMarketClose => {
            reply_to_create_margin_market_close(deps, info.data, module_resp)
        }

        ReplyType::MarginBrokerClose => reply_to_close_margin_order(deps, info.data, module_resp),
        ReplyType::SpotOrderMarketBuy => reply_to_spot_order_market(deps, info.data, module_resp),
        ReplyType::MarginBrokerOpen => reply_to_open_margin_position(deps, info.data, module_resp),
    }
}
