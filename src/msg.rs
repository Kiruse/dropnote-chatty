use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
  Execute {
    message: String,
    recipient: String,
    encrypted: bool,
  },
  Announce {
    message: String,
  },
  /// Set the public encryption key for the contract.
  /// This is for future E2EE based on ECDH.
  /// Note that this is merely a declaration of intent, and does not actually enable encryption.
  /// This is because encryption in a public blockchain is simply non-sense. This key is intended
  /// to be used by clients to encrypt messages for the owner of this contract (or rather, the owner
  /// of the corresponding private key) before sending it to this contract.
  SetEncryptionKey {
    key: Option<String>,
  },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
  #[returns(OwnerResponse)]
  Owner {},
}

#[cw_serde]
pub struct OwnerResponse {
  pub owner: String,
}
