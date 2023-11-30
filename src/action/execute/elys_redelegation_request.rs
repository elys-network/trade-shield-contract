use super::*;
use cosmwasm_std::Coin;

pub fn elys_redelegation_request(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    // the address of the current user.
    delegator_address: String,
    // the amount to be staked in base denomination.
    validator_src_address: String,
    // The asset to be staked
    validator_dst_address: String,
    // The validator Address is required only if the staked asset is
    // uelys.
    amount: Coin
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::begin_redelegate(
        delegator_address,
        validator_src_address,
        validator_dst_address,
        amount,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}