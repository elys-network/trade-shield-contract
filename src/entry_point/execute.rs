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
            trading_asset,
            take_profit_price,
            order_type,
            trigger_price,
            position_id,
        } => create_margin_order(
            info,
            deps,
            position,
            leverage,
            trading_asset,
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
        CloseMarginPosition { id, amount } => close_margin_position(info, id, amount),

        StakeRequest {
            amount,
            asset,
            validator_address,
        } => stake_request(info, deps, amount, asset, validator_address),
        UnstakeRequest {
            amount,
            asset,
            validator_address,
        } => unstake_request(info, deps, amount, asset, validator_address),
        ElysRedelegateRequest {
            validator_src_address,
            validator_dst_address,
            amount,
        } => elys_redelegation_request(
            info,
            deps,
            validator_src_address,
            validator_dst_address,
            amount,
        ),
        ElysCancelUnstakeRequest {
            validator_address,
            amount,
            creation_height,
        } => elys_cancel_unstake_request(info, deps, validator_address, amount, creation_height),
        EdenVestRequest { amount } => eden_vest_request(info, deps, amount),
        EdenCancelVestRequest { amount } => eden_cancel_vest_request(info, deps, amount),
        ClaimRewardsRequest { withdraw_type } => claim_rewards_request(info, deps, withdraw_type),
        ClaimValidatorCommissionRequest { validator_address } => {
            claim_validator_commission_request(info, deps, validator_address)
        },
        AmmJoinPoolRequest {
            pool_id,
            max_amounts_in,
            share_amount_out,
            no_remaining,
        } => join_amm_pool_request(info, deps, pool_id, max_amounts_in, share_amount_out, no_remaining),
        AmmExitPoolRequest {
            pool_id,
            min_amounts_out,
            share_amount_in,
            token_out_denom,
        }=> exit_amm_pool_request(info, deps, pool_id, min_amounts_out, share_amount_in, token_out_denom),
    }
}
