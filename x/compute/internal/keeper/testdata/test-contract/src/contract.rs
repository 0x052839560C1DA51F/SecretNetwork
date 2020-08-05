use cosmwasm_storage::PrefixedStorage;

use cosmwasm_std::{
    generic_err, invalid_base64, invalid_utf8, log, not_found, null_pointer, parse_err,
    serialize_err, to_binary, unauthorized, underflow, Api, BankMsg, Binary, Coin, CosmosMsg, Env,
    Extern, HandleResponse, HandleResult, HumanAddr, InitResponse, InitResult, MigrateResponse,
    Querier, QueryRequest, QueryResult, ReadonlyStorage, StdError, StdResult, Storage, Uint128,
    WasmMsg, WasmQuery,
};

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
        code_hash: String,
    },
    CallbackContractError {
        contract_addr: HumanAddr,
        code_hash: String,
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
        code_hash: String,
    },
    Panic {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    A {
        contract_addr: HumanAddr,
        code_hash: String,
        x: u8,
        y: u8,
    },
    B {
        contract_addr: HumanAddr,
        code_hash: String,
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
        code_hash: String,
    },
    CallbackBadParams {
        contract_addr: HumanAddr,
        code_hash: String,
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
    SendExternalQuery {
        to: HumanAddr,
        code_hash: String,
    },
    SendExternalQueryPanic {
        to: HumanAddr,
        code_hash: String,
    },
    SendExternalQueryError {
        to: HumanAddr,
        code_hash: String,
    },
    SendExternalQueryBadAbi {
        to: HumanAddr,
        code_hash: String,
    },
    SendExternalQueryBadAbiReceiver {
        to: HumanAddr,
        code_hash: String,
    },
    LogMsgSender {},
    CallbackToLogMsgSender {
        to: HumanAddr,
        code_hash: String,
    },
    DepositToContract {},
    SendFunds {
        amount: u32,
        denom: String,
        to: HumanAddr,
        from: HumanAddr,
    },
    SendFundsToInitCallback {
        amount: u32,
        denom: String,
        code_id: u64,
    },
    SendFundsToExecCallback {
        amount: u32,
        denom: String,
        to: HumanAddr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ContractError { error_type: String },
    Panic {},
    ReceiveExternalQuery { num: u8 },
    SendExternalQueryInfiniteLoop { to: HumanAddr, code_hash: String },
    WriteToStorage {},
    RemoveFromStorage {},
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
            code_hash,
        } => Ok(init_with_callback(deps, env, contract_addr, code_hash)),
        InitMsg::ContractError { error_type } => Err(map_string_to_error(error_type)),
        InitMsg::NoLogs {} => Ok(InitResponse::default()),
        InitMsg::CallbackToInit { code_id, code_hash } => {
            Ok(init_callback_to_init(deps, env, code_id, code_hash))
        }
        InitMsg::CallbackContractError {
            contract_addr,
            code_hash,
        } => Ok(init_with_callback_contract_error(contract_addr, code_hash)),
        InitMsg::CallbackBadParams {
            contract_addr,
            code_hash,
        } => Ok(init_callback_bad_params(contract_addr, code_hash)),
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

fn init_with_callback_contract_error(contract_addr: HumanAddr, code_hash: String) -> InitResponse {
    InitResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            callback_code_hash: code_hash,
            msg: Binary::from(r#"{"contract_error":{"error_type":"generic_err"}}"#.as_bytes()),
            send: vec![],
        })],
        log: vec![log("init with a callback with contract error", "🤷‍♀️")],
    }
}

fn init_callback_bad_params(contract_addr: HumanAddr, code_hash: String) -> InitResponse {
    InitResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            callback_code_hash: code_hash,
            msg: Binary::from(r#"{"c":{"x":"banana","y":3}}"#.as_bytes().to_vec()),
            send: vec![],
        })],
        log: vec![],
    }
}

fn init_with_callback<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    contract_addr: HumanAddr,
    code_hash: String,
) -> InitResponse {
    InitResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            callback_code_hash: code_hash,
            contract_addr: contract_addr.clone(),
            msg: Binary::from("{\"c\":{\"x\":0,\"y\":13}}".as_bytes().to_vec()),
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
            msg: Binary::from("{\"nop\":{}}".as_bytes().to_vec()),
            callback_code_hash: code_hash,
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
            code_hash,
            x,
            y,
        } => Ok(a(deps, env, contract_addr, code_hash, x, y)),
        HandleMsg::B {
            contract_addr,
            code_hash,
            x,
            y,
        } => Ok(b(deps, env, contract_addr, code_hash, x, y)),
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
            code_hash,
        } => Ok(exec_callback_bad_params(contract_addr, code_hash)),
        HandleMsg::CallbackContractError {
            contract_addr,
            code_hash,
        } => Ok(exec_with_callback_contract_error(contract_addr, code_hash)),
        HandleMsg::SetState { key, value } => Ok(set_state(deps, key, value)),
        HandleMsg::GetState { key } => Ok(get_state(deps, key)),
        HandleMsg::RemoveState { key } => Ok(remove_state(deps, key)),
        HandleMsg::TestCanonicalizeAddressErrors {} => test_canonicalize_address_errors(deps),
        HandleMsg::Panic {} => panic!("panic in exec"),
        HandleMsg::AllocateOnHeap { bytes } => Ok(allocate_on_heap(bytes as usize)),
        HandleMsg::PassNullPointerToImportsShouldThrow { pass_type } => {
            Ok(pass_null_pointer_to_imports_should_throw(deps, pass_type))
        }
        HandleMsg::SendExternalQuery { to, code_hash } => send_external_query(deps, to, code_hash),
        HandleMsg::SendExternalQueryPanic { to, code_hash } => {
            send_external_query_panic(deps, to, code_hash)
        }
        HandleMsg::SendExternalQueryError { to, code_hash } => {
            send_external_query_stderror(deps, to, code_hash)
        }
        HandleMsg::SendExternalQueryBadAbi { to, code_hash } => {
            send_external_query_bad_abi(deps, to, code_hash)
        }
        HandleMsg::SendExternalQueryBadAbiReceiver { to, code_hash } => {
            send_external_query_bad_abi_receiver(deps, to, code_hash)
        }
        HandleMsg::LogMsgSender {} => Ok(HandleResponse {
            messages: vec![],
            log: vec![log(
                "msg.sender",
                deps.api
                    .human_address(&env.message.sender)
                    .unwrap()
                    .to_string(),
            )],
            data: None,
        }),
        HandleMsg::CallbackToLogMsgSender { to, code_hash } => Ok(HandleResponse {
            messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: to.clone(),
                callback_code_hash: code_hash,
                msg: Binary::from(r#"{"log_msg_sender":{}}"#.as_bytes().to_vec()),
                send: vec![],
            })],
            log: vec![log("hi", "hey")],
            data: None,
        }),
        HandleMsg::DepositToContract {} => Ok(HandleResponse {
            messages: vec![],
            log: vec![],
            data: Some(to_binary(&env.message.sent_funds).unwrap()),
        }),
        HandleMsg::SendFunds {
            amount,
            from,
            to,
            denom,
        } => Ok(HandleResponse {
            messages: vec![CosmosMsg::Bank(BankMsg::Send {
                from_address: from,
                to_address: to,
                amount: vec![Coin {
                    amount: Uint128(amount as u128),
                    denom: denom,
                }],
            })],
            log: vec![],
            data: None,
        }),
        HandleMsg::SendFundsToInitCallback {
            amount,
            denom,
            code_id,
        } => Ok(HandleResponse {
            messages: vec![CosmosMsg::Wasm(WasmMsg::Instantiate {
                msg: Binary("{\"nop\":{}}".as_bytes().to_vec()),
                code_id: code_id,
                label: None,
                send: vec![Coin {
                    amount: Uint128(amount as u128),
                    denom: denom,
                }],
            })],
            log: vec![],
            data: None,
        }),
        HandleMsg::SendFundsToExecCallback { amount, denom, to } => Ok(HandleResponse {
            messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
                msg: Binary("{\"no_data\":{}}".as_bytes().to_vec()),
                contract_addr: to,
                send: vec![Coin {
                    amount: Uint128(amount as u128),
                    denom: denom,
                }],
            })],
            log: vec![],
            data: None,
        }),
    }
}

fn send_external_query<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    contract_addr: HumanAddr,
    code_hash: String,
) -> HandleResult {
    let answer: u8 = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            callback_code_hash: code_hash,
            msg: Binary::from(r#"{"receive_external_query":{"num":2}}"#.as_bytes().to_vec()),
        }))
        .unwrap();

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(vec![answer].into()),
    })
}

fn send_external_query_panic<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    contract_addr: HumanAddr,
    code_hash: String,
) -> HandleResult {
    let err = deps
        .querier
        .query::<u8>(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            msg: Binary::from(r#"{"panic":{}}"#.as_bytes().to_vec()),
            callback_code_hash: code_hash,
        }))
        .unwrap_err();

    Err(err)
}

fn send_external_query_stderror<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    contract_addr: HumanAddr,
    code_hash: String,
) -> HandleResult {
    let answer = deps
        .querier
        .query::<Binary>(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            msg: Binary::from(
                r#"{"contract_error":{"error_type":"generic_err"}}"#
                    .as_bytes()
                    .to_vec(),
            ),
            callback_code_hash: code_hash,
        }));

    match answer {
        Ok(wtf) => Ok(HandleResponse {
            messages: vec![],
            log: vec![],
            data: Some(wtf),
        }),
        Err(e) => Err(e),
    }
}

fn send_external_query_bad_abi<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    contract_addr: HumanAddr,
    code_hash: String,
) -> HandleResult {
    let answer = deps
        .querier
        .query::<Binary>(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            callback_code_hash: code_hash,
            msg: Binary::from(
                r#""contract_error":{"error_type":"generic_err"}}"#.as_bytes().to_vec(),
            ),
        }));

    match answer {
        Ok(wtf) => Ok(HandleResponse {
            messages: vec![],
            log: vec![],
            data: Some(wtf),
        }),
        Err(e) => Err(e),
    }
}

fn send_external_query_bad_abi_receiver<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    contract_addr: HumanAddr,
    code_hash: String,
) -> HandleResult {
    let answer = deps
        .querier
        .query::<String>(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            msg: Binary::from(r#"{"receive_external_query":{"num":25}}"#.as_bytes().to_vec()),
            callback_code_hash: code_hash,
        }));

    match answer {
        Ok(wtf) => Ok(HandleResponse {
            messages: vec![],
            log: vec![log("wtf", wtf)],
            data: None,
        }),
        Err(e) => Err(e),
    }
}

fn exec_callback_bad_params(contract_addr: HumanAddr, code_hash: String) -> HandleResponse {
    HandleResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            callback_code_hash: code_hash,
            msg: Binary::from(r#"{"c":{"x":"banana","y":3}}"#.as_bytes().to_vec()),
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
    code_hash: String,
    x: u8,
    y: u8,
) -> HandleResponse {
    HandleResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            callback_code_hash: code_hash.clone(),
            msg: Binary::from(format!(
                "{{\"b\":{{\"x\":{} ,\"y\": {},\"contract_addr\": \"{}\",\"code_hash\": \"{}\" }}}}",
                x,
                y,
                contract_addr.as_str(),
                &code_hash
            )
                .as_bytes()
                .to_vec()),
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
    code_hash: String,
    x: u8,
    y: u8,
) -> HandleResponse {
    HandleResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            callback_code_hash: code_hash,
            msg: Binary::from(
                format!("{{\"c\":{{\"x\":{} ,\"y\": {} }}}}", x + 1, y + 1)
                    .as_bytes()
                    .to_vec(),
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
    code_hash: String,
) -> HandleResponse {
    HandleResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id,
            msg: Binary::from("{\"nop\":{}}".as_bytes().to_vec()),
            callback_code_hash: code_hash,
            send: vec![],
            label: None,
        })],
        log: vec![log("instantiating a new contract", "🪂")],
        data: None,
    }
}

fn exec_with_callback_contract_error(
    contract_addr: HumanAddr,
    code_hash: String,
) -> HandleResponse {
    HandleResponse {
        messages: vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.clone(),
            callback_code_hash: code_hash,
            msg: Binary::from(
                r#"{"contract_error":{"error_type":"generic_err"}}"#
                    .as_bytes()
                    .to_vec(),
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
        QueryMsg::ContractError { error_type } => Err(map_string_to_error(error_type)),
        QueryMsg::Panic {} => panic!("panic in query"),
        QueryMsg::ReceiveExternalQuery { num } => {
            Ok(Binary(serde_json_wasm::to_vec(&(num + 1)).unwrap()))
        }
        QueryMsg::SendExternalQueryInfiniteLoop { to, code_hash } => {
            send_external_query_infinite_loop(deps, to, code_hash)
        }
        QueryMsg::WriteToStorage {} => write_to_storage_in_query(deps),
        QueryMsg::RemoveFromStorage {} => remove_from_storage_in_query(deps),
    }
}

fn send_external_query_infinite_loop<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    contract_addr: HumanAddr,
    code_hash: String,
) -> QueryResult {
    let answer = deps
        .querier
        .query::<Binary>(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: contract_addr.clone(),
            callback_code_hash: code_hash.clone(),
            msg: Binary::from(
                format!(
                    r#"{{"send_external_query_infinite_loop":{{"to":"{}", "code_hash":"{}"}}}}"#,
                    contract_addr.clone().to_string(),
                    &code_hash
                )
                .as_bytes()
                .to_vec(),
            ),
        }));

    match answer {
        Ok(wtf) => Ok(Binary(wtf.into())),
        Err(e) => Err(e),
    }
}

fn write_to_storage_in_query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<Binary> {
    let deps = unsafe { &mut *(deps as *const _ as *mut Extern<S, A, Q>) };
    deps.storage.set(b"abcd", b"dcba");

    Ok(Binary(vec![]))
}

fn remove_from_storage_in_query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<Binary> {
    let deps = unsafe { &mut *(deps as *const _ as *mut Extern<S, A, Q>) };
    deps.storage.remove(b"abcd");

    Ok(Binary(vec![]))
}

/////////////////////////////// Migrate ///////////////////////////////

pub fn migrate<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: MigrateMsg,
) -> StdResult<MigrateResponse> {
    Ok(MigrateResponse::default())
}
