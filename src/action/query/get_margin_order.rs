use cosmwasm_std::StdError;

use super::*;

pub fn get_margin_order(
    deps: Deps<ElysQuery>,
    address: String,
    id: u64,
) -> Result<Mtp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp: MarginMtpResponse = querier.mtp(address, id)?;

    if let Some(mtp) = resp.mtp {
        Ok(mtp)
    } else {
        Err(StdError::not_found("margin trading prosition").into())
    }
}
