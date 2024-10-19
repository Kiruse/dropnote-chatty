#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

type ContractResult<T> = Result<T, ContractError>;

#[allow(dead_code)]
struct ExecuteContext<'a> {
  deps: DepsMut<'a>,
  env: Env,
  info: MessageInfo,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  let state = State {
    owner: info.sender.clone(),
    encryption_key: None,
  };
  STATE.save(deps.storage, &state)?;

  Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  let ctx = ExecuteContext { deps, env, info };
  match msg {
    ExecuteMsg::Execute { message, recipient, encrypted } => exec_execute(ctx, message, recipient, encrypted),
    ExecuteMsg::Announce { message } => exec_announce(ctx, message),
    ExecuteMsg::SetEncryptionKey { key } => exec_set_encryption_key(ctx, key),
  }
}

fn exec_execute(ctx: ExecuteContext, message: String, recipient: String, encrypted: bool) -> ContractResult<Response> {
  // events created within a smart contract are prefixed with "wasm-" by the chain
  let event = Event::new("dropnote")
    .add_attribute("type", "message")
    .add_attribute("message", message)
    .add_attribute("recipient", recipient)
    .add_attribute("encrypted", encrypted.to_string())
    .add_attribute("sender", ctx.info.sender.to_string());

  Ok(Response::new()
    .add_attribute("action", "execute")
    .add_event(event)
  )
}

fn exec_announce(ctx: ExecuteContext, message: String) -> ContractResult<Response> {
  let event = Event::new("dropnote")
    .add_attribute("type", "announce")
    .add_attribute("message", message)
    .add_attribute("sender", ctx.info.sender.to_string());

  Ok(Response::new()
    .add_attribute("action", "announce")
    .add_event(event)
  )
}

fn exec_set_encryption_key(ctx: ExecuteContext, key: Option<String>) -> ContractResult<Response> {
  assert_owner(&ctx)?;

  let mut state = STATE.load(ctx.deps.storage)?;
  state.encryption_key = key.clone();
  STATE.save(ctx.deps.storage, &state)?;

  let event = Event::new("dropnote")
    .add_attribute("type", "pubkey")
    .add_attribute("key", key.unwrap_or_default());

  Ok(Response::new()
    .add_attribute("action", "set_encryption_key")
    .add_event(event)
  )
}

fn assert_owner(ctx: &ExecuteContext) -> ContractResult<()> {
  if ctx.info.sender != STATE.load(ctx.deps.storage)?.owner {
    Err(ContractError::Unauthorized {})
  } else {
    Ok(())
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    QueryMsg::Owner {} => {
      let state = STATE.load(deps.storage)?;
      to_json_binary(&OwnerResponse { owner: state.owner.to_string() })
    }
  }
}
