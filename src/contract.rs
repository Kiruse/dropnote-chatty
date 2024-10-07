#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{BankMsg, Binary, Deps, DepsMut, Env, Event, MessageInfo, Reply, Response, StdResult, SubMsg};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const REPLY_ID_EXECUTE: u64 = 1;

type ContractResult<T> = Result<T, ContractError>;

struct ExecuteContext<'a> {
  deps: DepsMut<'a>,
  #[allow(dead_code)]
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
    message: None,
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
    ExecuteMsg::Execute { message } =>
      exec_execute(ctx, message),
    ExecuteMsg::EmergencyReset {} =>
      exec_emergency_reset(ctx),
  }
}

fn exec_execute(ctx: ExecuteContext, message: String) -> ContractResult<Response> {
  let mut state = STATE.load(ctx.deps.storage)?;
  if state.message.is_some() {
    return Err(ContractError::generic("message already set"));
  }

  if ctx.info.funds.len() != 1 {
    return Err(ContractError::generic("expected 1 coin"));
  }

  state.message = Some(message);
  STATE.save(ctx.deps.storage, &state)?;

  let submsg = SubMsg::reply_always(
    BankMsg::Send {
      to_address: ctx.info.sender.to_string(),
      amount: vec![ctx.info.funds[0].clone()],
    },
    REPLY_ID_EXECUTE
  );

  Ok(Response::new()
    .add_attribute("action", "execute")
    .add_submessage(submsg)
  )
}

fn exec_emergency_reset(ctx: ExecuteContext) -> ContractResult<Response> {
  let mut state = STATE.load(ctx.deps.storage)?;
  if ctx.info.sender != state.owner {
    return Err(ContractError::Unauthorized {});
  }

  state.message = None;
  STATE.save(ctx.deps.storage, &state)?;

  Ok(Response::new().add_attribute("action", "emergency_reset"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
  match msg {
    Reply { id: REPLY_ID_EXECUTE, result } => {
      let mut state = STATE.load(deps.storage)?;
      let message = state.message.ok_or(ContractError::generic("message not set"))?;

      state.message = None;
      STATE.save(deps.storage, &state)?;

      let mut e = Event::new("dropnote");
      if result.is_ok() {
        e = e.add_attribute("message", message);
      }

      Ok(Response::new().add_event(e))
    }
    Reply { id: _, .. } => {
      Err(ContractError::generic("unknown reply id"))
    }
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
  unimplemented!()
}
