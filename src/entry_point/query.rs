use super::*;
use msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query;
    use QueryMsg::*;

    match msg {
        GetSpotOrder { order_id } => Ok(to_binary(&query::get_spot_order(deps, order_id)?)?),
        GetAllPrices {} => Ok(to_binary(&query::get_all_prices(deps)?)?),
    }
}
