use cosmwasm_std::{Addr, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

use crate::types::Status;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
    #[error("{order_id} : Not Found")]
    OrderNotFound { order_id: u64 },
    #[error("{order_ids:?} : Not Found")]
    OrdersNotFound { order_ids: Vec<u64> },
    #[error("{sender} is not the owner of the order")]
    Unauthorized { sender: Addr },
    #[error("Incorrect number of funds. Only one fund is allowed.")]
    CoinNumber,
    #[error("order price already been reached")]
    OrderPriceReached,
    #[error("order_source_denom and order_target_denom cannot be the same")]
    SpotOrderSameDenom,
    #[error("denom in order_price not used")]
    OrderPriceDenom,
    #[error("fund not used by the order")]
    SpotOrderWrongFund,
    #[error("{order_id} is prossessing")]
    ProcessSpotOrderProcessing { order_id: u64 },
    #[error("invalid leverage amount")]
    Leverage,
    #[error("cannot cancel order: {order_id}, status: {status:?}")]
    CancelStatusError { order_id: u64, status: Status },
    #[error("{balance} is smaller than {amount}")]
    InsufficientBalanceError {balance: u128, amount: u64},
}
