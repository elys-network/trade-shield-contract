use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin, CosmosMsg, CustomMsg, Decimal, Int128};

use crate::types::{MarginPosition, SwapAmountInRoute};

#[cw_serde]
pub enum ElysMsg {
    MsgSwapExactAmountIn {
        sender: String,
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
        token_out_min_amount: Int128,
        meta_data: Option<Binary>,
    },
    MsgOpen {
        creator: String,
        collateral_asset: String,
        collateral_amount: Int128,
        borrow_asset: String,
        position: i32,
        leverage: Decimal,
        take_profit_price: Decimal,
        meta_data: Option<Binary>,
    },
    MsgClose {
        creator: String,
        id: u64,
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

    pub fn open_position(
        creator: &str,
        collateral_asset: &str,
        collateral_amount: Int128,
        borrow_asset: &str,
        position: MarginPosition,
        leverage: Decimal,
        take_profit_price: Decimal,
        meta_data: Option<Binary>,
    ) -> Self {
        Self::MsgOpen {
            creator: creator.to_owned(),
            collateral_asset: collateral_asset.to_owned(),
            collateral_amount,
            borrow_asset: borrow_asset.to_owned(),
            position: position as i32,
            leverage,
            take_profit_price,
            meta_data,
        }
    }

    pub fn close_position(creator: &str, id: u64, meta_data: Option<Binary>) -> Self {
        Self::MsgClose {
            creator: creator.to_owned(),
            id,
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
