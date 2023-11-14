use cosmwasm_std::{from_json, Response, SubMsgResult};
use elys_bindings::ElysMsg;
use serde::de::DeserializeOwned;

pub fn get_response_from_reply<T: DeserializeOwned>(
    module_resp: SubMsgResult,
) -> Result<T, Response<ElysMsg>> {
    let response = match module_resp.into_result() {
        Ok(response) => response,
        Err(err) => return Err(Response::new().add_attribute("error", err)),
    };

    let data = match response.data {
        Some(data) => data,
        None => return Err(Response::new().add_attribute("error", "No Data")),
    };

    match from_json::<T>(&data) {
        Ok(resp) => Ok(resp),
        Err(err) => Err(Response::new().add_attribute("error", err.to_string())),
    }
}
