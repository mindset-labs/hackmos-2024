use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid category")]
    InvalidCategory {},

    #[error("Invalid royalty fee")]
    InvalidRoyaltyFee {},

    #[error("Property contract code ID not provided")]
    PropertyCodeIdNotProvided {},

    #[error("Unknown reply message ID")]
    UnknownReplyMessageId {},

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
