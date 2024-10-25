use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Code ID not provided: {contract_type}")]
    CodeIdNotProvided { contract_type: &'static str },

    #[error("Invalid reply message id: {msg_id}")]
    InvalidReplyMessageId { msg_id: u64 },

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
