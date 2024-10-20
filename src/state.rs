use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
  pub owner: Addr,
  pub encryption_key: Option<String>,
}

pub const STATE: Item<State> = Item::new("state");
