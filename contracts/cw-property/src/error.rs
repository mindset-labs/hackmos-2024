use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Cw404 error: {0}")]
    Cw404(#[from] cw404::error::ContractError),

    #[error("No funds sent")]
    NoFundsSent {},

    #[error("Insufficient funds")]
    InsufficientFunds {
        required: Uint128,
    },

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
