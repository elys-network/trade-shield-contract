use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Coin, CosmosMsg, CustomMsg, Int128};

use crate::types::SwapAmountInRoute;

#[cw_serde]
pub enum ElysMsg {
    MsgSwapExactAmountIn {
        sender: String,
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
        token_out_min_amount: Int128,
        meta_data: Option<Binary>,
    },
}

impl ElysMsg {
    pub fn swap_exact_amount_in(
        sender: &str,
        token_in: &Coin,
        token_route: &Vec<SwapAmountInRoute>,
        token_out_min_amount: Int128,
        meta_data: Option<Binary>,
    ) -> Self {
        Self::MsgSwapExactAmountIn {
            sender: sender.to_owned(),
            routes: token_route.to_owned(),
            token_in: token_in.to_owned(),
            token_out_min_amount,
            meta_data,
        }
    }
}

impl From<ElysMsg> for CosmosMsg<ElysMsg> {
    fn from(msg: ElysMsg) -> CosmosMsg<ElysMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ElysMsg {}
