//! WASI linking for core modules (preview 1) and components (preview 2).
//!
//! Preview 1 `path_*` imports are wrapped so the report names the path, then
//! the real Wasmtime host runs. Preview 2 network attempts are named by the
//! socket address check. Nothing is inherited: no preopens, no environment,
//! and no `inherit_network`.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use wasmtime::component::{Component, Linker as ComponentLinker, ResourceTable};
use wasmtime::{Caller, Extern, Linker, Memory, Module, Store, Val, ValType};
use wasmtime_wasi::p1::WasiP1Ctx;
use wasmtime_wasi::p2::bindings::sync::Command;
use wasmtime_wasi::sockets::SocketAddrUse;
use wasmtime_wasi::{FsPerms, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use crate::caps::{Grants, NetGrant, event};
use crate::engine::{Engine, KillHandle};
use crate::error::Error;
use crate::report::{Event, EventKind, Report};
use crate::run::{Entry, HasControl, RunControl, RunRequest, arm, rejected_public, report_from};

const WASI_P1: &str = "wasi_snapshot_preview1";

struct P1State {
    control: RunControl,
    wasi: WasiP1Ctx,
    /// Wasm shims that call the real preview-1 imports. Filled in after the
    /// guest is instantiated, because the shim must export the guest memory.
    adapters: HashMap<&'static str, wasmtime::Func>,
}

impl HasControl for P1State {
    fn control_mut(&mut self) -> &mut RunControl {
        &mut self.control
    }
}

struct P2State {
    control: RunControl,
    ctx: WasiCtx,
    table: ResourceTable,
}

impl HasControl for P2State {
    fn control_mut(&mut self) -> &mut RunControl {
        &mut self.control
    }
}

impl WasiView for P2State {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

pub(crate) fn is_component(bytes: &[u8]) -> bool {
    const HEADER: [u8; 8] = [0x00, b'a', b's', b'm', 0x0d, 0x00, 0x01, 0x00];
    bytes.starts_with(&HEADER)
}

pub(crate) fn execute_preview1(
    engine: &Engine,
    module: Module,
    request: RunRequest,
    kill: &KillHandle,
    warnings: Vec<String>,
) -> Result<Report, Error> {
    let control = RunControl::new(&request.limits, kill.flag());
    let journal = Arc::clone(&control.journal);
    let wasi = build_p1(&request, &control, Arc::clone(&journal))?;
    record_env_grants(&request.grants, &journal);

    let mut store = Store::new(
        engine.wasm(),
        P1State {
            control,
            wasi,
            adapters: HashMap::new(),
        },
    );
    arm(&mut store, request.limits.fuel)?;
    let started = store.data().control.started;

    let mut linker = Linker::new(engine.wasm());
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |state: &mut P1State| &mut state.wasi)?;
    journal_paths(&mut linker, &mut store)?;

    let instance = match linker.instantiate(&mut store, &module) {
        Ok(instance) => instance,
        Err(err) => {
            return Ok(finish_p1(
                &store,
                &request,
                warnings,
                started,
                None,
                Err(err),
            ));
        }
    };
    if let Some(memory) = instance.get_memory(&mut store, "memory") {
        bind_path_adapters(engine, &linker, &mut store, memory)?;
    }

    let export = match &request.entry {
        Entry::Command { .. } => "_start",
        Entry::Export(name) => name.as_str(),
    };
    let Some(func) = instance.get_func(&mut store, export) else {
        return Ok(rejected_public(
            warnings,
            format!("module does not export `{export}`"),
        ));
    };
    let ty = func.ty(&store);
    if ty.params().len() != 0 {
        return Ok(rejected_public(
            warnings,
            format!("export `{export}` takes parameters; boxwren calls exports with none"),
        ));
    }
    let mut results = vec![Val::I32(0); ty.results().len()];
    let result = func.call(&mut store, &[], &mut results);
    let value = results.first().cloned();
    Ok(finish_p1(
        &store,
        &request,
        warnings,
        started,
        value.as_ref(),
        result.map(|_| ()),
    ))
}

pub(crate) fn execute_component(
    engine: &Engine,
    request: RunRequest,
    kill: &KillHandle,
    warnings: Vec<String>,
) -> Result<Report, Error> {
    let component = match Component::new(engine.wasm(), &request.wasm) {
        Ok(component) => component,
        Err(err) => return Ok(rejected_public(warnings, err.to_string())),
    };

    let control = RunControl::new(&request.limits, kill.flag());
    let journal = Arc::clone(&control.journal);
    let ctx = build_ctx(&request, &control, Arc::clone(&journal))?;
    record_env_grants(&request.grants, &journal);

    let mut store = Store::new(
        engine.wasm(),
        P2State {
            control,
            ctx,
            table: ResourceTable::new(),
        },
    );
    arm(&mut store, request.limits.fuel)?;
    let started = store.data().control.started;

    let mut linker = ComponentLinker::new(engine.wasm());
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    let command = match Command::instantiate(&mut store, &component, &linker) {
        Ok(command) => command,
        Err(err) => return Ok(finish_p2(&store, &request, warnings, started, Err(err))),
    };
    let result = match command.wasi_cli_run().call_run(&mut store) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(())) => Err(exit_error(1)),
        Err(err) => Err(err),
    };
    Ok(finish_p2(&store, &request, warnings, started, result))
}

fn finish_p1(
    store: &Store<P1State>,
    request: &RunRequest,
    warnings: Vec<String>,
    started: std::time::Instant,
    first: Option<&Val>,
    result: Result<(), wasmtime::Error>,
) -> Report {
    report_from(
        store.get_fuel().unwrap_or(request.limits.fuel),
        &store.data().control,
        request,
        warnings,
        started,
        first,
        result,
    )
}

fn finish_p2(
    store: &Store<P2State>,
    request: &RunRequest,
    warnings: Vec<String>,
    started: std::time::Instant,
    result: Result<(), wasmtime::Error>,
) -> Report {
    report_from(
        store.get_fuel().unwrap_or(request.limits.fuel),
        &store.data().control,
        request,
        warnings,
        started,
        None,
        result,
    )
}

fn exit_error(code: i32) -> wasmtime::Error {
    wasmtime_wasi::I32Exit(code).into()
}

fn build_p1(
    request: &RunRequest,
    control: &RunControl,
    journal: Arc<Mutex<Vec<Event>>>,
) -> Result<WasiP1Ctx, Error> {
    let mut builder = WasiCtxBuilder::new();
    configure(&mut builder, request, control, journal)?;
    Ok(builder.build_p1())
}

fn build_ctx(
    request: &RunRequest,
    control: &RunControl,
    journal: Arc<Mutex<Vec<Event>>>,
) -> Result<WasiCtx, Error> {
    let mut builder = WasiCtxBuilder::new();
    configure(&mut builder, request, control, journal)?;
    Ok(builder.build())
}

fn configure(
    builder: &mut WasiCtxBuilder,
    request: &RunRequest,
    control: &RunControl,
    journal: Arc<Mutex<Vec<Event>>>,
) -> Result<(), Error> {
    builder.stdout(control.stdout.clone());
    builder.stderr(control.stderr.clone());

    let args = command_args(request);
    builder.args(&args);

    for grant in &request.grants.env {
        builder.env(&grant.name, &grant.value);
    }
    if let Some(dir) = request.grants.dirs.first() {
        builder.initial_cwd(&dir.guest);
    }
    for dir in &request.grants.dirs {
        let perms = if dir.writable {
            FsPerms::ReadWrite
        } else {
            FsPerms::ReadOnly
        };
        builder
            .preopened_dir(&dir.host, &dir.guest, perms)
            .map_err(|err| {
                Error::BadGrant(format!("could not preopen {}: {err}", dir.host.display()))
            })?;
    }

    // TCP and UDP exist so the address check can see the destination.
    // The check denies every address that was not granted. This is not
    // `inherit_network`, which would allow every host address.
    builder.allow_tcp(true);
    builder.allow_udp(true);
    let grants = request.grants.net.clone();
    builder.socket_addr_check(move |addr, reason| {
        let allowed = address_allowed(&grants, addr, reason);
        if journal_address(addr, reason) {
            push_event(
                &journal,
                event(EventKind::Network, addr.to_string(), allowed),
            );
        }
        Box::pin(async move { allowed })
    });
    Ok(())
}

fn command_args(request: &RunRequest) -> Vec<String> {
    let from_entry = match &request.entry {
        Entry::Command { args } if !args.is_empty() => Some(args.clone()),
        _ => None,
    };
    from_entry.unwrap_or_else(|| {
        if request.args.is_empty() {
            vec!["boxwren".to_string()]
        } else {
            request.args.clone()
        }
    })
}

fn record_env_grants(grants: &Grants, journal: &Arc<Mutex<Vec<Event>>>) {
    let mut events = journal.lock().unwrap_or_else(|poison| poison.into_inner());
    for grant in &grants.env {
        events.push(event(EventKind::Env, grant.name.clone(), true));
    }
}

fn address_allowed(grants: &[NetGrant], addr: SocketAddr, reason: SocketAddrUse) -> bool {
    if is_implicit_bind(addr, reason) {
        return true;
    }
    grants.iter().any(|grant| grant.allows(addr))
}

fn is_implicit_bind(addr: SocketAddr, reason: SocketAddrUse) -> bool {
    matches!(reason, SocketAddrUse::TcpBind | SocketAddrUse::UdpBind)
        && addr.ip().is_unspecified()
        && addr.port() == 0
}

fn journal_address(addr: SocketAddr, reason: SocketAddrUse) -> bool {
    !is_implicit_bind(addr, reason)
}

fn push_event(journal: &Mutex<Vec<Event>>, event: Event) {
    journal
        .lock()
        .unwrap_or_else(|poison| poison.into_inner())
        .push(event);
}

fn journal_paths(linker: &mut Linker<P1State>, store: &mut Store<P1State>) -> Result<(), Error> {
    for (name, ptr_index, len_index) in PATH_IMPORTS {
        wrap_path(linker, store, name, *ptr_index, *len_index)?;
    }
    Ok(())
}

fn wrap_path(
    linker: &mut Linker<P1State>,
    store: &mut Store<P1State>,
    name: &'static str,
    ptr_index: usize,
    len_index: usize,
) -> Result<(), Error> {
    let original = match linker.get(&mut *store, WASI_P1, name) {
        Ok(Extern::Func(func)) => func,
        _ => return Ok(()),
    };
    let ty = original.ty(&*store);
    let params: Vec<ValType> = ty.params().collect();
    let results: Vec<ValType> = ty.results().collect();
    if params.len() <= len_index
        || !matches!(params[ptr_index], ValType::I32)
        || !matches!(params[len_index], ValType::I32)
        || !is_errno_result(&results)
    {
        return Ok(());
    }
    linker.allow_shadowing(true);
    // The real import is reached through a Wasm shim installed later. Calling
    // it directly from this host function hides the guest memory, and preview 1
    // then fails with "missing required memory export".
    linker
        .func_new(WASI_P1, name, ty, move |mut caller, params, results| {
            let path = match (params.get(ptr_index), params.get(len_index)) {
                (Some(Val::I32(ptr)), Some(Val::I32(len))) => {
                    read_guest_string(&mut caller, *ptr as u32, *len as u32)
                }
                _ => None,
            };
            let adapter = caller.data().adapters.get(name).cloned();
            let Some(adapter) = adapter else {
                return Err(wasmtime::format_err!(
                    "filesystem adapter for {name} was not installed"
                ));
            };
            let call = adapter.call(&mut caller, params, results);
            if let Some(path) = path {
                let allowed = call.is_ok()
                    && results
                        .first()
                        .is_some_and(|value| matches!(value, Val::I32(0)));
                push_event(
                    &caller.data().control.journal,
                    event(EventKind::Filesystem, path, allowed),
                );
            }
            call
        })
        .map_err(Error::from)?;
    Ok(())
}

fn bind_path_adapters(
    engine: &Engine,
    guest_linker: &Linker<P1State>,
    store: &mut Store<P1State>,
    memory: Memory,
) -> Result<(), Error> {
    let mut imports = String::new();
    let mut funcs = String::new();
    let mut names = Vec::new();
    for (name, _, _) in PATH_IMPORTS {
        let Ok(Extern::Func(func)) = guest_linker.get(&mut *store, WASI_P1, name) else {
            continue;
        };
        let ty = func.ty(&*store);
        let params: Vec<ValType> = ty.params().collect();
        let results: Vec<ValType> = ty.results().collect();
        if !is_errno_result(&results) {
            continue;
        }
        let Some(wat_params) = wasm_params(&params) else {
            continue;
        };
        let gets = (0..params.len())
            .map(|index| format!("(local.get {index})"))
            .collect::<String>();
        imports.push_str(&format!(
            r#"(import "{WASI_P1}" "{name}" (func $raw_{name} (param {wat_params}) (result i32)))"#
        ));
        funcs.push_str(&format!(
            r#"
            (func (export "{name}") (param {wat_params}) (result i32)
              {gets}
              call $raw_{name})
            "#
        ));
        names.push(*name);
    }
    if names.is_empty() {
        return Ok(());
    }
    let wat = format!(
        r#"(module
            (import "guest" "memory" (memory 1))
            {imports}
            (export "memory" (memory 0))
            {funcs})"#
    );
    let module = Module::new(engine.wasm(), wat).map_err(Error::from)?;
    let mut linker = Linker::new(engine.wasm());
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |state: &mut P1State| &mut state.wasi)?;
    linker
        .define(&mut *store, "guest", "memory", memory)
        .map_err(Error::from)?;
    let instance = linker
        .instantiate(&mut *store, &module)
        .map_err(Error::from)?;
    for name in names {
        let Some(func) = instance.get_func(&mut *store, name) else {
            continue;
        };
        store.data_mut().adapters.insert(name, func);
    }
    Ok(())
}

fn is_errno_result(results: &[ValType]) -> bool {
    results.len() == 1 && ValType::eq(&results[0], &ValType::I32)
}

fn wasm_params(params: &[ValType]) -> Option<String> {
    let mut out = String::new();
    for ty in params {
        let name = match ty {
            ValType::I32 => "i32",
            ValType::I64 => "i64",
            ValType::F32 => "f32",
            ValType::F64 => "f64",
            _ => return None,
        };
        out.push_str(name);
        out.push(' ');
    }
    Some(out)
}

const PATH_IMPORTS: &[(&str, usize, usize)] = &[
    ("path_open", 2, 3),
    ("path_create_directory", 1, 2),
    ("path_filestat_get", 2, 3),
    ("path_filestat_set_times", 2, 3),
    ("path_link", 2, 3),
    ("path_readlink", 1, 2),
    ("path_remove_directory", 1, 2),
    ("path_rename", 1, 2),
    ("path_symlink", 0, 1),
    ("path_unlink_file", 1, 2),
];

fn read_guest_string(caller: &mut Caller<'_, P1State>, ptr: u32, len: u32) -> Option<String> {
    let memory = caller.get_export("memory")?.into_memory()?;
    let start = ptr as usize;
    let end = start.checked_add(len as usize)?;
    let bytes = memory.data(caller).get(start..end)?;
    Some(String::from_utf8_lossy(bytes).into_owned())
}
