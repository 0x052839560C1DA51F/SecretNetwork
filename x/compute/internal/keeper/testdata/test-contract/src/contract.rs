use cosmwasm_storage::PrefixedStorage;

use cosmwasm_std::{
    generic_err, invalid_base64, invalid_utf8, log, not_found, null_pointer, parse_err,
    serialize_err, to_binary, unauthorized, underflow, Api, Binary, CosmosMsg, Env, Extern,
    HandleResponse, HandleResult, HumanAddr, InitResponse, InitResult, MigrateResponse, Querier,
    QueryResult, ReadonlyStorage, StdError, StdResult, Storage, WasmMsg,
};

use crate::state::config_read;

/////////////////////////////// Messages ///////////////////////////////

use mem::MaybeUninit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::mem;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InitMsg {
    Nop {},
    Callback {
        contract_addr: HumanAddr,
        contract_key: String,
    },
    CallbackContractError {
        contract_addr: HumanAddr,
        contract_key: String,
    },
    ContractError {
        error_type: String,
    },
    NoLogs {},
    CallbackToInit {
        code_id: u64,
        code_hash: String,
    },
    CallbackBadParams {
        contract_addr: HumanAddr,
        contract_key: String,
    },
    Panic {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    A {
        contract_addr: HumanAddr,
        contract_key: String,
        x: u8,
        y: u8,
    },
    B {
        contract_addr: HumanAddr,
        contract_key: String,
        x: u8,
        y: u8,
    },
    C {
        x: u8,
        y: u8,
    },
    UnicodeData {},
    EmptyLogKeyValue {},
    EmptyData {},
    NoData {},
    ContractError {
        error_type: String,
    },
    NoLogs {},
    CallbackToInit {
        code_id: u64,
        code_hash: String,
    },
    CallbackContractError {
        contract_addr: HumanAddr,
        contract_key: String,
    },
    CallbackBadParams {
        contract_addr: HumanAddr,
        contract_key: String,
    },
    SetState {
        key: String,
        value: String,
    },
    GetState {
        key: String,
    },
    RemoveState {
        key: String,
    },
    TestCanonicalizeAddressErrors {},
    Panic {},
    AllocateOnHeap {
        bytes: u32,
    },
    PassNullPointerToImportsShouldThrow {
        pass_type: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Owner {},
    ContractError { error_type: String },
    Panic {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
    pub owner: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}

/////////////////////////////// Init ///////////////////////////////

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> InitResult {
    match msg {
        InitMsg::Nop {} => Ok(InitResponse {
            messages: vec![],
            log: vec![log("init", "🌈")],
        }),
        InitMsg::Callback {
            contract_addr,
            contract_key,
        } => Ok(init_with_callback(deps, env, contract_addr, contract_key)),
        InitMsg::ContractError { error_type } => Err(map_string_to_error(error_type)),
        InitMsg::NoLogs {} => Ok(InitResponse::default()),
        InitMsg::CallbackToInit { code_id, code_hash } => {
            Ok(init_callback_to_init(deps, env, code_id, code_hash))
        }
        InitMsg::CallbackContractError {
            contract_addr,
            contract_key,
        } => Ok(init_with_callback_contract_error(
            contract_addr,
            contract_key,
        )),
        InitMsg::CallbackBadParams {
            contract_addr,
            contract_key,
        } => Ok(init_callback_bad_params(contract_addr, contract_key)),
        InitMsg::Panic {} => panic!("panic in init"),
    }
}

fn map_string_to_error(error_type: String) -> StdError {
    let as_str: &str = &error_type[..];
    match as_str {
        "generic_err" => generic_err("la la 🤯"),
        "invalid_base64" => invalid_base64("ra ra 🤯"),
        "invalid_utf8" => invalid_utf8("ka ka 🤯"),
        "not_found" => not_found("za za 🤯"),
        "null_pointer" => null_pointer(),
        "parse_err" => parse_err("na na 🤯", "pa pa 🤯"),
        "serialize_err" => serialize_err("ba ba 🤯", "ga ga 🤯"),
        "unauthorized" => unauthorized(),
        "underflow" => underflow("minuend 🤯", "subtrahend 🤯"),
        _ => generic_err("catch-all 🤯"),
    }
}

fn init_with_callback_contract_error(
    contract_addr: HumanAddr,
    contract_key: String,
) -> InitResponse {
    InitResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            msg: create_callback_msg(
                r#"{"contract_error":{"error_type":"generic_err"}}"#
                    .as_bytes()
                    .to_vec(),
                &contract_key,
            ),
            send: vec![],
        })],
        log: vec![log("init with a callback with contract error", "🤷‍♀️")],
    }
}

fn create_callback_msg(msg: Vec<u8>, contract_key: &String) -> Binary {
    let mut new_msg = contract_key.as_bytes().to_vec();

    new_msg.extend(msg);

    Binary(new_msg)
}

fn init_callback_bad_params(contract_addr: HumanAddr, contract_key: String) -> InitResponse {
    InitResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            msg: create_callback_msg(
                r#"{"c":{"x":"banana","y":3}}"#.as_bytes().to_vec(),
                &contract_key,
            ),
            send: vec![],
        })],
        log: vec![],
    }
}

fn init_with_callback<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    contract_addr: HumanAddr,
    contract_key: String,
) -> InitResponse {
    InitResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            msg: create_callback_msg(
                "{\"c\":{\"x\":0,\"y\":13}}".as_bytes().to_vec(),
                &contract_key,
            ),
            send: vec![],
        })],
        log: vec![log("init with a callback", "🦄")],
    }
}

pub fn init_callback_to_init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    code_id: u64,
    code_hash: String,
) -> InitResponse {
    InitResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id,
            msg: create_callback_msg("{\"nop\":{}}".as_bytes().to_vec(), &code_hash),
            send: vec![],
            label: None,
        })],
        log: vec![log("instantiating a new contract from init!", "🐙")],
    }
}

/////////////////////////////// Handle ///////////////////////////////

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> HandleResult {
    match msg {
        HandleMsg::A {
            contract_addr,
            contract_key,
            x,
            y,
        } => Ok(a(deps, env, contract_addr, contract_key, x, y)),
        HandleMsg::B {
            contract_addr,
            contract_key,
            x,
            y,
        } => Ok(b(deps, env, contract_addr, contract_key, x, y)),
        HandleMsg::C { x, y } => Ok(c(deps, env, x, y)),
        HandleMsg::UnicodeData {} => Ok(unicode_data(deps, env)),
        HandleMsg::EmptyLogKeyValue {} => Ok(empty_log_key_value(deps, env)),
        HandleMsg::EmptyData {} => Ok(empty_data(deps, env)),
        HandleMsg::NoData {} => Ok(no_data(deps, env)),
        HandleMsg::ContractError { error_type } => Err(map_string_to_error(error_type)),
        HandleMsg::NoLogs {} => Ok(HandleResponse::default()),
        HandleMsg::CallbackToInit { code_id, code_hash } => {
            Ok(exec_callback_to_init(deps, env, code_id, code_hash))
        }
        HandleMsg::CallbackBadParams {
            contract_addr,
            contract_key,
        } => Ok(exec_callback_bad_params(contract_addr, contract_key)),
        HandleMsg::CallbackContractError {
            contract_addr,
            contract_key,
        } => Ok(exec_with_callback_contract_error(
            contract_addr,
            contract_key,
        )),
        HandleMsg::SetState { key, value } => Ok(set_state(deps, key, value)),
        HandleMsg::GetState { key } => Ok(get_state(deps, key)),
        HandleMsg::RemoveState { key } => Ok(remove_state(deps, key)),
        HandleMsg::TestCanonicalizeAddressErrors {} => test_canonicalize_address_errors(deps),
        HandleMsg::Panic {} => panic!("panic in exec"),
        HandleMsg::AllocateOnHeap { bytes } => Ok(allocate_on_heap(bytes as usize)),
        HandleMsg::PassNullPointerToImportsShouldThrow { pass_type } => {
            Ok(pass_null_pointer_to_imports_should_throw(deps, pass_type))
        }
    }
}

fn exec_callback_bad_params(contract_addr: HumanAddr, contract_key: String) -> HandleResponse {
    HandleResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            msg: create_callback_msg(
                r#"{"c":{"x":"banana","y":3}}"#.as_bytes().to_vec(),
                &contract_key,
            ),
            send: vec![],
        })],
        log: vec![],
        data: None,
    }
}

pub fn a<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    contract_addr: HumanAddr,
    contract_key: String,
    x: u8,
    y: u8,
) -> HandleResponse {
    HandleResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            msg: create_callback_msg(
                format!(
                    "{{\"b\":{{\"x\":{} ,\"y\": {},\"contract_addr\": \"{}\" }}}}",
                    x,
                    y,
                    contract_addr.as_str()
                )
                .as_bytes()
                .to_vec(),
                &contract_key,
            ),
            send: vec![],
        })],
        log: vec![log("banana", "🍌")],
        data: Some(Binary(vec![x, y])),
    }
}

pub fn b<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    contract_addr: HumanAddr,
    contract_key: String,
    x: u8,
    y: u8,
) -> HandleResponse {
    HandleResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            msg: create_callback_msg(
                format!("{{\"c\":{{\"x\":{} ,\"y\": {} }}}}", x + 1, y + 1)
                    .as_bytes()
                    .to_vec(),
                &contract_key,
            ),
            send: vec![],
        })],
        log: vec![log("kiwi", "🥝")],
        data: Some(Binary(vec![x + y])),
    }
}

pub fn c<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    x: u8,
    y: u8,
) -> HandleResponse {
    HandleResponse {
        messages: vec![],
        log: vec![log("watermelon", "🍉")],
        data: Some(Binary(vec![x + y])),
    }
}

pub fn empty_log_key_value<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> HandleResponse {
    HandleResponse {
        messages: vec![],
        log: vec![log("my value is empty", ""), log("", "my key is empty")],
        data: None,
    }
}

pub fn empty_data<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> HandleResponse {
    HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(Binary(vec![])),
    }
}

pub fn unicode_data<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> HandleResponse {
    HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(Binary("🍆🥑🍄".as_bytes().to_vec())),
    }
}

pub fn no_data<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> HandleResponse {
    HandleResponse {
        messages: vec![],
        log: vec![],
        data: None,
    }
}

pub fn exec_callback_to_init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    code_id: u64,
    contract_key: String,
) -> HandleResponse {
    HandleResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id,
            msg: create_callback_msg("{\"nop\":{}}".as_bytes().to_vec(), &contract_key),
            send: vec![],
            label: None,
        })],
        log: vec![log("instantiating a new contract", "🪂")],
        data: None,
    }
}

fn exec_with_callback_contract_error(
    contract_addr: HumanAddr,
    contract_key: String,
) -> HandleResponse {
    HandleResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            msg: create_callback_msg(
                r#"{"contract_error":{"error_type":"generic_err"}}"#
                    .as_bytes()
                    .to_vec(),
                &contract_key,
            ),
            send: vec![],
        })],
        log: vec![log("exec with a callback with contract error", "🤷‍♂️")],
        data: None,
    }
}

fn allocate_on_heap(bytes: usize) -> HandleResponse {
    let mut values: Vec<u8> = vec![0; bytes];
    values[bytes - 1] = 1;

    HandleResponse {
        data: Some(Binary("😅".as_bytes().to_vec())),
        log: vec![],
        messages: vec![],
    }
}

fn get_state<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    key: String,
) -> HandleResponse {
    let store = PrefixedStorage::new(b"my_prefix", &mut deps.storage);

    match store.get(key.as_bytes()) {
        Some(value) => HandleResponse {
            data: Some(Binary(value)),
            log: vec![],
            messages: vec![],
        },
        None => HandleResponse::default(),
    }
}

fn set_state<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    key: String,
    value: String,
) -> HandleResponse {
    let mut store = PrefixedStorage::new(b"my_prefix", &mut deps.storage);
    store.set(key.as_bytes(), value.as_bytes());
    HandleResponse::default()
}

fn remove_state<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    key: String,
) -> HandleResponse {
    let mut store = PrefixedStorage::new(b"my_prefix", &mut deps.storage);
    store.remove(key.as_bytes());
    HandleResponse::default()
}

#[allow(invalid_value)]
#[allow(unused_must_use)]
fn pass_null_pointer_to_imports_should_throw<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    pass_type: String,
) -> HandleResponse {
    let null_ptr_slice: &[u8] = unsafe { MaybeUninit::zeroed().assume_init() };

    match &pass_type[..] {
        "read_db_key" => {
            deps.storage.get(null_ptr_slice);
        }
        "write_db_key" => {
            deps.storage.set(null_ptr_slice, b"write value");
        }
        "write_db_value" => {
            deps.storage.set(b"write key", null_ptr_slice);
        }
        "remove_db_key" => {
            deps.storage.remove(null_ptr_slice);
        }
        "canonicalize_address_input" => {
            deps.api
                .canonical_address(unsafe { MaybeUninit::zeroed().assume_init() });
        }
        "canonicalize_address_output" => { /* TODO */ }
        "humanize_address_input" => {
            deps.api
                .human_address(unsafe { MaybeUninit::zeroed().assume_init() });
        }
        "humanize_address_output" => { /* TODO */ }
        _ => {}
    };

    HandleResponse::default()
}

fn test_canonicalize_address_errors<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
) -> HandleResult {
    match deps.api.canonical_address(&HumanAddr(String::from(""))) {
        Err(StdError::GenericErr { msg, backtrace: _ }) => {
            if msg != String::from("canonicalize_address returned error") {
                return Err(generic_err("empty address should have failed with -2"));
            }
            // all is good, continue
        }
        _ => return Err(generic_err("empty address should have failed with -2")),
    }

    match deps.api.canonical_address(&HumanAddr(String::from("   "))) {
        Err(StdError::GenericErr { msg, backtrace: _ }) => {
            if msg != String::from("canonicalize_address returned error") {
                return Err(generic_err(
                    "empty trimmed address should have failed with -2",
                ));
            }
            // all is good, continue
        }
        _ => {
            return Err(generic_err(
                "empty trimmed address should have failed with -2",
            ))
        }
    }

    match deps
        .api
        .canonical_address(&HumanAddr(String::from("cosmos1h99hrcc54ms9lxxxx")))
    {
        Err(StdError::GenericErr { msg, backtrace: _ }) => {
            if msg != String::from("canonicalize_address returned error") {
                return Err(generic_err("bad bech32 should have failed with -3"));
            }
            // all is good, continue
        }
        _ => return Err(generic_err("bad bech32 should have failed with -3")),
    }

    match deps.api.canonical_address(&HumanAddr(String::from(
        "cosmos1h99hrcc54ms9luwpex9kw0rwdt7etvfdyxh6gu",
    ))) {
        Err(StdError::GenericErr { msg, backtrace: _ }) => {
            if msg != String::from("canonicalize_address returned error") {
                return Err(generic_err("bad prefix should have failed with -4"));
            }
            // all is good, continue
        }
        _ => return Err(generic_err("bad prefix should have failed with -4")),
    }

    Ok(HandleResponse {
        data: Some(Binary("🤟".as_bytes().to_vec())),
        log: vec![],
        messages: vec![],
    })
}

/////////////////////////////// Query ///////////////////////////////

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    _msg: QueryMsg,
) -> QueryResult {
    match _msg {
        QueryMsg::Owner {} => query_owner(deps),
        QueryMsg::ContractError { error_type } => Err(map_string_to_error(error_type)),
        QueryMsg::Panic {} => panic!("panic in query"),
    }
}

fn query_owner<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Binary> {
    let state = config_read(&deps.storage).load()?;

    let resp = OwnerResponse {
        owner: deps.api.human_address(&state.owner)?,
    };
    to_binary(&resp)
}

/////////////////////////////// Migrate ///////////////////////////////

pub fn migrate<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: MigrateMsg,
) -> StdResult<MigrateResponse> {
    Ok(MigrateResponse::default())
}
