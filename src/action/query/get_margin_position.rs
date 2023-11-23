use cosmwasm_std::StdError;

use super::*;

pub fn get_margin_position(
    deps: Deps<ElysQuery>,
    address: String,
    id: u64,
) -> Result<MarginMtpResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp: MarginMtpResponse = querier.mtp(address, id)?;

    if let Some(_) = &resp.mtp {
        Ok(resp)
    } else {
        Err(StdError::not_found("margin trading prosition").into())
    }
}
