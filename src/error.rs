use cosmwasm_std::{Addr, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
    #[error("{order_id} : Not Found")]
    OrderNotFound { order_id: u128 },
    #[error("{sender} is not the owner of the order")]
    Unauthorized { sender: Addr },
    #[error("Incorrect number of funds. Only one fund is allowed.")]
    CoinNumber,
    #[error("order price already been reached")]
    OrderPriceReached,
    #[error("order_base_denom and order_target_denom cannot be the same")]
    OrderSameDenom,
    #[error("denom in order_price_pair not used")]
    OrderPricePair,
    #[error("fund not used by the order")]
    OrderWrongFund,
}
