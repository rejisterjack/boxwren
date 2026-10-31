use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use wasmtime::{Instance, Module, Store, UpdateDeadline, Val, ValType};

use crate::caps::Grants;
use crate::capture::Capture;
use crate::engine::{Engine, KillHandle};
use crate::error::Error;
use crate::limiter::RunLimiter;
use crate::limits::Limits;
use crate::report::{Event, Report, SCHEMA_VERSION, Status};

/// What to invoke after the module is instantiated.
#[derive(Clone, Debug)]
pub enum Entry {
    /// Call an export with no parameters. An `i32` or `i64` result is recorded.
    Export(String),
    /// Call WASI `_start` as a command. Arguments are the guest's argv.
    Command { args: Vec<String> },
}

/// One execution of untrusted code.
#[derive(Clone, Debug)]
pub struct RunRequest {
    pub wasm: Vec<u8>,
    pub entry: Entry,
    pub limits: Limits,
    pub grants: Grants,
    pub args: Vec<String>,
}

pub(crate) struct RunControl {
    pub limiter: RunLimiter,
    pub killed: Arc<AtomicBool>,
    /// Wall clock starts at guest entry (`begin_guest_wall`), not during compile/instantiate.
    pub started: Option<Instant>,
    pub wall: Duration,
    pub timed_out: bool,
    pub journal: Arc<Mutex<Vec<Event>>>,
    pub stdout: Capture,
    pub stderr: Capture,
}

impl RunControl {
    pub(crate) fn new(limits: &Limits, killed: Arc<AtomicBool>) -> Self {
        Self {
            limiter: RunLimiter::new(limits.memory_bytes),
            killed,
            started: None,
            wall: limits.wall_time,
            timed_out: false,
            journal: Arc::new(Mutex::new(Vec::new())),
            stdout: Capture::new(limits.max_stdio_bytes),
            stderr: Capture::new(limits.max_stdio_bytes),
        }
    }
}

pub(crate) struct StoreState {
    pub control: RunControl,
}

impl Engine {
    /// Run a module. Timeout still applies. Nothing is killed unless the
    /// process is torn down.
    pub fn execute(&self, request: RunRequest) -> Result<Report, Error> {
        let kill = self.kill_handle();
        self.execute_with(request, &kill)
    }

    /// Run a module. [`KillHandle::kill`] from another thread stops this run
    /// and leaves other runs on this engine alone.
    pub fn execute_with(&self, request: RunRequest, kill: &KillHandle) -> Result<Report, Error> {
        self.execute_with_preflight(request, kill, &AllowAll)
    }

    /// Like [`Self::execute_with`], but asks `preflight` before instantiation.
    /// Keywarden (or any policy) can deny the run without starting the guest.
    pub fn execute_with_preflight(
        &self,
        request: RunRequest,
        kill: &KillHandle,
        preflight: &dyn crate::hooks::Preflight,
    ) -> Result<Report, Error> {
        if !self.same_engine(kill) {
            return Err(Error::ForeignKillHandle);
        }
        preflight.allow(&request)?;
        let warnings = request.grants.validate()?;
        if request.wasm.len() > request.limits.max_module_bytes {
            return Ok(rejected(
                warnings,
                format!(
                    "module is {} bytes, over the {} byte cap",
                    request.wasm.len(),
                    request.limits.max_module_bytes
                ),
            ));
        }

        if crate::wasi::is_component(&request.wasm) {
            return crate::wasi::execute_component(self, request, kill, warnings);
        }

        let module = match self.module(&request.wasm) {
            Ok(module) => module,
            Err(err) => return Ok(rejected(warnings, err.to_string())),
        };

        if needs_wasi(&module, &request.entry) {
            return crate::wasi::execute_preview1(self, module, request, kill, warnings);
        }

        execute_raw(self, module, request, kill, warnings)
    }
}

struct AllowAll;

impl crate::hooks::Preflight for AllowAll {
    fn allow(&self, _request: &RunRequest) -> Result<(), crate::hooks::PolicyDeny> {
        Ok(())
    }
}

fn needs_wasi(module: &Module, entry: &Entry) -> bool {
    if matches!(entry, Entry::Command { .. }) {
        return true;
    }
    module.imports().any(|import| {
        let module = import.module();
        module.starts_with("wasi_") || module.starts_with("wasi:")
    })
}

fn execute_raw(
    engine: &Engine,
    module: Module,
    request: RunRequest,
    kill: &KillHandle,
    warnings: Vec<String>,
) -> Result<Report, Error> {
    let Entry::Export(name) = &request.entry else {
        return Ok(rejected(
            warnings,
            "WASI command entry requires the WASI linker".into(),
        ));
    };

    let mut store = open_store(engine, &request, kill)?;

    let instance = match Instance::new(&mut store, &module, &[]) {
        Ok(instance) => instance,
        Err(err) => {
            return Ok(finish(
                &store,
                &request,
                warnings,
                Instant::now(),
                None,
                Err(err),
            ));
        }
    };
    let Some(func) = instance.get_func(&mut store, name) else {
        return Ok(rejected(
            warnings,
            format!("module does not export `{name}`"),
        ));
    };
    let ty = func.ty(&store);
    if ty.params().len() != 0 {
        return Ok(rejected(
            warnings,
            format!("export `{name}` takes parameters; boxwren calls exports with none"),
        ));
    }
    if ty.results().any(|kind| !is_number(&kind)) {
        return Ok(rejected(
            warnings,
            format!("export `{name}` returns a non-numeric value"),
        ));
    }
    let mut results = ty.results().map(zero_val).collect::<Vec<_>>();
    let started = begin_guest_wall(&mut store);
    let result = func.call(&mut store, &[], &mut results);
    Ok(finish(
        &store,
        &request,
        warnings,
        started,
        results.first(),
        result.map(|_| ()),
    ))
}

fn is_number(kind: &ValType) -> bool {
    matches!(
        kind,
        ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64
    )
}

fn zero_val(kind: ValType) -> Val {
    match kind {
        ValType::I32 => Val::I32(0),
        ValType::I64 => Val::I64(0),
        ValType::F32 => Val::F32(0),
        ValType::F64 => Val::F64(0),
        _ => Val::I32(0),
    }
}

pub(crate) fn open_store(
    engine: &Engine,
    request: &RunRequest,
    kill: &KillHandle,
) -> Result<Store<StoreState>, Error> {
    let state = StoreState {
        control: RunControl::new(&request.limits, kill.flag()),
    };
    let mut store = Store::new(engine.wasm(), state);
    arm(&mut store, request.limits.fuel)?;
    Ok(store)
}

pub(crate) fn arm<T>(store: &mut Store<T>, fuel: u64) -> Result<(), Error>
where
    T: HasControl + Send + 'static,
{
    store.set_fuel(fuel)?;
    store.limiter(|state| &mut state.control_mut().limiter);
    // Wall clock stays unset until `begin_guest_wall` so Cranelift compile and
    // Wasm instantiate of large guests (e.g. QuickJS) do not burn the deadline.
    store.data_mut().control_mut().started = None;
    store.epoch_deadline_callback(|mut store| {
        let control = store.data_mut().control_mut();
        if control.killed.load(Ordering::Acquire) {
            return Ok(UpdateDeadline::Interrupt);
        }
        if let Some(started) = control.started {
            if started.elapsed() >= control.wall {
                control.timed_out = true;
                return Ok(UpdateDeadline::Interrupt);
            }
        }
        Ok(UpdateDeadline::Continue(1))
    });
    store.set_epoch_deadline(1);
    Ok(())
}

/// Start measuring wall time for guest entry (after instantiate).
pub(crate) fn begin_guest_wall<T>(store: &mut Store<T>) -> Instant
where
    T: HasControl,
{
    let now = Instant::now();
    store.data_mut().control_mut().started = Some(now);
    now
}

pub(crate) trait HasControl {
    fn control_mut(&mut self) -> &mut RunControl;
}

impl HasControl for StoreState {
    fn control_mut(&mut self) -> &mut RunControl {
        &mut self.control
    }
}

pub(crate) fn finish(
    store: &Store<StoreState>,
    request: &RunRequest,
    warnings: Vec<String>,
    started: Instant,
    first_result: Option<&Val>,
    result: Result<(), wasmtime::Error>,
) -> Report {
    report_from(
        store.get_fuel().unwrap_or(request.limits.fuel),
        &store.data().control,
        request,
        warnings,
        started,
        first_result,
        result,
    )
}

pub(crate) fn report_from(
    fuel_remaining: u64,
    control: &RunControl,
    request: &RunRequest,
    warnings: Vec<String>,
    started: Instant,
    first_result: Option<&Val>,
    result: Result<(), wasmtime::Error>,
) -> Report {
    let fuel_used = request.limits.fuel.saturating_sub(fuel_remaining);
    let peak_memory = control.limiter.peak_memory as u64;
    let memory_denied = control.limiter.memory_denied;
    let killed = control.killed.load(Ordering::Acquire);
    let timed_out = control.timed_out;
    let duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    let (stdout, stdout_truncated) = control.stdout.text();
    let (stderr, stderr_truncated) = control.stderr.text();
    let events = control
        .journal
        .lock()
        .unwrap_or_else(|poison| poison.into_inner())
        .clone();

    let (status, exit_code) = match result {
        Ok(()) if killed => (Status::Killed, None),
        Ok(()) if memory_denied => (Status::MemoryLimited, None),
        Ok(()) => {
            let code = matches!(request.entry, Entry::Command { .. }).then_some(0);
            (Status::Success, code)
        }
        Err(err) => classify(err, killed, timed_out, memory_denied),
    };

    Report {
        schema_version: SCHEMA_VERSION,
        status,
        exit_code,
        return_value: first_result.and_then(val_i64),
        duration_ms,
        fuel_used,
        peak_memory,
        stdout,
        stderr,
        stdout_truncated,
        stderr_truncated,
        events,
        warnings,
    }
}

fn classify(
    err: wasmtime::Error,
    killed: bool,
    timed_out: bool,
    memory_denied: bool,
) -> (Status, Option<i32>) {
    if killed {
        return (Status::Killed, None);
    }
    if timed_out {
        return (Status::TimedOut, None);
    }
    if let Some(trap) = err.downcast_ref::<wasmtime::Trap>()
        && matches!(trap, wasmtime::Trap::OutOfFuel)
    {
        return (Status::FuelExhausted, None);
    }
    if memory_denied || looks_like_memory_limit(&err) {
        return (Status::MemoryLimited, None);
    }
    if let Some(exit) = err.downcast_ref::<wasmtime_wasi::I32Exit>() {
        return (Status::Success, Some(exit.0));
    }
    if let Some(trap) = err.downcast_ref::<wasmtime::Trap>()
        && matches!(trap, wasmtime::Trap::Interrupt)
    {
        return (Status::TimedOut, None);
    }
    let message = trap_message(&err);
    (Status::Trapped { message }, None)
}

fn trap_message(err: &wasmtime::Error) -> String {
    let root = err.root_cause().to_string();
    let line = root.lines().next().unwrap_or(&root).trim();
    let line = if line.is_empty() || line.starts_with("error while executing") {
        err.to_string()
    } else {
        line.to_string()
    };
    const LIMIT: usize = 240;
    if line.len() <= LIMIT {
        line
    } else {
        line.chars().take(LIMIT).collect()
    }
}

fn looks_like_memory_limit(err: &wasmtime::Error) -> bool {
    let text = err.to_string().to_ascii_lowercase();
    text.contains("memory")
        && (text.contains("limit") || text.contains("exceed") || text.contains("maximum"))
}

fn val_i64(val: &Val) -> Option<i64> {
    match val {
        Val::I32(value) => Some(i64::from(*value)),
        Val::I64(value) => Some(*value),
        _ => None,
    }
}

pub(crate) fn rejected_public(warnings: Vec<String>, message: String) -> Report {
    rejected(warnings, message)
}

fn rejected(warnings: Vec<String>, message: String) -> Report {
    Report {
        schema_version: SCHEMA_VERSION,
        status: Status::Rejected { message },
        exit_code: None,
        return_value: None,
        duration_ms: 0,
        fuel_used: 0,
        peak_memory: 0,
        stdout: String::new(),
        stderr: String::new(),
        stdout_truncated: false,
        stderr_truncated: false,
        events: Vec::new(),
        warnings,
    }
}
