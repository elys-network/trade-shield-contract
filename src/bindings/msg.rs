use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Int128};

use crate::types::SwapAmountInRoute;

#[cw_serde]
pub enum ElysMsg {
    MsgSwapExactAmountIn {
        sender: String,
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
        token_out_min_amount: Int128,
    },
}

#[allow(dead_code)]
impl ElysMsg {
    pub fn swap_exact_amount_in(
        contract_addr: &str,
        token_in: Coin,
        token_route: Vec<SwapAmountInRoute>,
    ) -> Self {
        Self::MsgSwapExactAmountIn {
            sender: contract_addr.to_owned(),
            routes: token_route,
            token_in,
            token_out_min_amount: Int128::new(0),
        }
    }
}

impl From<ElysMsg> for CosmosMsg<ElysMsg> {
    fn from(msg: ElysMsg) -> CosmosMsg<ElysMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ElysMsg {}
