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
    OrderNotFound{order_id : String},
    #[error("{sender} is not the owner of the order")]
    Unauthorized { sender: Addr },
    #[error("Incorrect number of funds. Only one fund is allowed.")]
    CoinNumber,
    #[error ("stop price already been reached")]
    StopPriceReached,
    #[error("no such order type: {order_type}")]
    OrderType {order_type : String}
}