use crate::bindings::query::ElysQuery;

use super::*;

pub fn process_order(
    _deps: DepsMut<ElysQuery>,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
