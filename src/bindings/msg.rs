use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Int128};

use crate::types::{Pool, SwapAmountInRoute};

#[cw_serde]
pub enum ElysMsg {
    MsgSwapExactAmountIn {
        sender: String,
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
        token_out_min_amount: Int128,
    },
}
///order amm routes
fn create_route(denom_in: &str, denom_out: &str, pools: &[Pool]) -> Vec<SwapAmountInRoute> {
    unimplemented!()
}

impl ElysMsg {
    pub fn swap_exact_amount_in(
        contract_addr: &str,
        token_in: Coin,
        pools: &[Pool],
        denom_out: &str,
    ) -> Self {
        let token_route = create_route(&token_in.denom, denom_out, pools);
        Self::MsgSwapExactAmountIn {
            sender: contract_addr.to_owned(),
            routes: token_route,
            token_in,
            token_out_min_amount: Int128::new(0),
        }
    }
}
