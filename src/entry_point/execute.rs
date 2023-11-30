use super::*;
use msg::ExecuteMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ElysQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ElysMsg>, ContractError> {
    use action::execute::*;
    use ExecuteMsg::*;

    match msg {
        CreateSpotOrder {
            order_type,
            order_source_denom,
            order_target_denom,
            order_price,
        } => create_spot_order(
            env,
            deps,
            info,
            order_type,
            order_source_denom,
            order_target_denom,
            order_price,
        ),
        CancelSpotOrder { order_id } => cancel_spot_order(info, deps, order_id),
        CancelSpotOrders {
            order_ids,
            owner_address,
            order_type,
        } => cancel_spot_orders(info, deps, order_ids, owner_address, order_type),

        CreateMarginOrder {
            position,
            leverage,
            borrow_asset,
            take_profit_price,
            order_type,
            trigger_price,
            position_id,
        } => create_margin_order(
            info,
            deps,
            env,
            position,
            leverage,
            borrow_asset,
            take_profit_price,
            order_type,
            trigger_price,
            position_id,
        ),
        CancelMarginOrder { order_id } => cancel_margin_order(info, deps, order_id),
        CancelMarginOrders {
            order_ids,
            owner_address,
            order_type,
        } => cancel_margin_orders(info, deps, order_ids, owner_address, order_type),
        CloseMarginPosition { id } => close_margin_position(info, env, id),

        StakeRequest { address, amount, asset, validator_address } => stake_request(env, info, deps, address, amount, asset, validator_address),
        UnstakeRequest { address, amount, asset, validator_address } => unstake_request(env, info, deps, address, amount, asset, validator_address),
        ElysRedelegateRequest { delegator_address, validator_src_address, validator_dst_address, amount} => elys_redelegation_request(env, info, deps, delegator_address, validator_src_address, validator_dst_address, amount),
        ElysCancelUnstakeRequest { delegator_address, validator_address, amount, creation_height } => elys_cancel_unstake_request(env, info, deps, delegator_address, validator_address, amount, creation_height),
        EdenVestRequest { creator, amount} => eden_vest_request(env, info, deps, creator, amount),
        EdenCancelVestRequest { creator, amount  } => eden_cancel_vest_request(env, info, deps, creator, amount),
        ClaimRewardsRequest { delegator_address, withdraw_type } => claim_rewards_request(env, info, deps, delegator_address, withdraw_type),
        ClaimValidatorCommissionRequest { delegator_address, validator_address } => claim_validator_commission_request(env, info, deps, delegator_address, validator_address),
    }
}
