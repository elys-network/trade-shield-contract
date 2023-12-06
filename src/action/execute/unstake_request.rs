use super::*;
use cosmwasm_std::Int128;

pub fn unstake_request(
    info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    // the amount to be staked in base denomination.
    amount: u64,
    // The asset to be staked
    asset: String,
    // The validator Address is required only if the staked asset is
    // uelys.
    validator_address: Option<String>,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::unstake_token(
        info.sender.into_string(),
        Int128::from(amount),
        asset,
        validator_address,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
