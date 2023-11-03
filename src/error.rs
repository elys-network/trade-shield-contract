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
    SpotOrderNotFound { order_id: u64 },
    #[error("{sender} is not the owner of the order")]
    Unauthorized { sender: Addr },
    #[error("Incorrect number of funds. Only one fund is allowed.")]
    CoinNumber,
    #[error("order price already been reached")]
    SpotOrderPriceReached,
    #[error("order_source_denom and order_target_denom cannot be the same")]
    SpotOrderSameDenom,
    #[error("denom in order_price not used")]
    SpotOrderPriceDenom,
    #[error("fund not used by the order")]
    SpotOrderWrongFund,
    #[error("{sender} is no autorized to use the process_orders endpoint")]
    ProcessSpotOrderAuth { sender: Addr },
    #[error("{order_id} is prossessing")]
    ProcessSpotOrderProcessing { order_id: u64 },
    #[error("not the same amount send as collateral")]
    CollateralAmount,
    #[error("invalid leverage amount")]
    Leverage,
}
