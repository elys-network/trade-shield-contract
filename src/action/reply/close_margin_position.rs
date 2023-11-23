use cosmwasm_std::{from_json, StdError, SubMsgResult};

use super::*;

pub fn reply_to_close_margin_order(
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let response = match module_resp.into_result() {
        Ok(response) => response,
        Err(err) => return Err(StdError::generic_err(err).into()),
    };

    let data = match response.data {
        Some(data) => data,
        None => return Err(StdError::generic_err("No Data").into()),
    };

    let close_resp: MarginBrokerCloseResResponse = match from_json(&data) {
        Ok(resp) => resp,
        Err(err) => return Err(err.into()),
    };

    let resp = Response::new().add_attribute("order_id", close_resp.id.to_string());

    Ok(resp)
}
