use super::*;
use crate::states::*;
use cosmwasm_std::Addr;
use msg::InstantiateMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<ElysMsg>> {
    // Verify the existence of a real address by querying its balance
    deps.querier
        .query_balance(msg.process_order_executor.clone(), "usdc")?;

    SPOT_ORDER.save(deps.storage, &vec![])?;
    PROCESSED_SPOT_ORDER.save(deps.storage, &vec![])?;
    PROCESS_SPOT_ORDER_EXECUTOR.save(deps.storage, &Addr::unchecked(msg.process_order_executor))?;

    Ok(Response::new())
}
