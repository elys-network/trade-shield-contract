use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, WasmMsg};

use super::*;

#[cw_serde]
pub enum ExecuteMsg {
    SetMsg { msg: String },
}

pub fn send_cosmos_msg(contract_addr: String, msg: String) -> Result<Response, ContractError> {
    let bin = to_binary(&ExecuteMsg::SetMsg { msg })?;
    let msg = WasmMsg::Execute {
        contract_addr,
        msg: bin,
        funds: vec![],
    };
    let resp: Response = Response::new().add_message(CosmosMsg::Wasm(msg));
    Ok(resp)
}
