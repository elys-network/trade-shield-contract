use crate::bindings::query::ElysQuery;

use super::*;
use msg::QueryMsg;

pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query;
    use QueryMsg::*;

    match msg {
        GetOrder { order_id } => Ok(to_binary(&query::get_order(deps, order_id)?)?),
        GetAllPrices {} => Ok(to_binary(&query::get_all_prices(deps)?)?),
    }
}
