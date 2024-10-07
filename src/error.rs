use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("Unauthorized")]
  Unauthorized {},

  #[error("Generic error: {msg}")]
  GenericError { msg: String },

  // Add any other custom errors you like here.
  // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}

impl ContractError {
  pub fn generic(msg: impl Into<String>) -> Self {
    ContractError::GenericError { msg: msg.into() }
  }
}
