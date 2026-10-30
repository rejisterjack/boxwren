//! C ABI used by the Go binding. The sandbox still lives in `boxwren`.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::time::Duration;

use boxwren::{DirGrant, Entry, EnvGrant, Grants, Limits, NetGrant, RunRequest};
use serde::Deserialize;

struct Engine(boxwren::Engine);

/// Opaque per-run kill switch. Create one per run from an engine.
struct KillHandle(boxwren::KillHandle);

#[derive(Deserialize)]
struct RequestJson {
    #[serde(default = "command_entry")]
    entry: String,
    #[serde(default)]
    args: Vec<String>,
    fuel: Option<u64>,
    memory_bytes: Option<usize>,
    wall_ms: Option<u64>,
    #[serde(default)]
    dirs: Vec<String>,
    #[serde(default)]
    net: Vec<String>,
    #[serde(default)]
    env: Vec<String>,
}

fn command_entry() -> String {
    "_start".into()
}

/// Create an engine. NULL if Wasmtime could not start.
#[unsafe(no_mangle)]
pub extern "C" fn boxwren_engine_new() -> *mut c_void {
    match boxwren::Engine::new() {
        Ok(engine) => Box::into_raw(Box::new(Engine(engine))) as *mut c_void,
        Err(_) => std::ptr::null_mut(),
    }
}

/// # Safety
/// `engine` is null or a pointer returned by [`boxwren_engine_new`] and not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn boxwren_engine_free(engine: *mut c_void) {
    if !engine.is_null() {
        unsafe {
            drop(Box::from_raw(engine as *mut Engine));
        }
    }
}

/// Create a kill handle bound to this engine. NULL if `engine` is null.
///
/// Clone semantics: create one handle per run. Other runs on the same
/// engine keep their own deadlines.
///
/// # Safety
/// `engine` is a live engine pointer. Free the result with [`boxwren_kill_free`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn boxwren_kill_new(engine: *mut c_void) -> *mut c_void {
    if engine.is_null() {
        return std::ptr::null_mut();
    }
    let engine = unsafe { &*(engine as *const Engine) };
    Box::into_raw(Box::new(KillHandle(engine.0.kill_handle()))) as *mut c_void
}

/// Stop the run this handle is passed to. Safe to call from another thread.
///
/// # Safety
/// `handle` is null or a pointer returned by [`boxwren_kill_new`] and not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn boxwren_kill(handle: *mut c_void) {
    if !handle.is_null() {
        unsafe {
            (*(handle as *mut KillHandle)).0.kill();
        }
    }
}

/// # Safety
/// `handle` is null or a pointer returned by [`boxwren_kill_new`] and not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn boxwren_kill_free(handle: *mut c_void) {
    if !handle.is_null() {
        unsafe {
            drop(Box::from_raw(handle as *mut KillHandle));
        }
    }
}

/// Run a module with an external kill handle. Returns a newly allocated JSON
/// report, or NULL with a message in `err_out` when it is non-null.
///
/// Pass NULL for `kill` to run without an external kill switch. Free the
/// report string with [`boxwren_string_free`].
///
/// # Safety
/// `engine` is a live engine pointer. `wasm` points at `wasm_len` bytes.
/// `request_json` is null or a NUL-terminated UTF-8 string. `kill` is null or
/// a pointer returned by [`boxwren_kill_new`] for the same engine. `err_out`
/// is null or a writable pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn boxwren_execute_with(
    engine: *mut c_void,
    wasm: *const u8,
    wasm_len: usize,
    request_json: *const c_char,
    kill: *mut c_void,
    err_out: *mut *mut c_char,
) -> *mut c_char {
    if engine.is_null() {
        write_err(err_out, "null engine");
        return std::ptr::null_mut();
    }
    if kill.is_null() {
        return unsafe { boxwren_execute(engine, wasm, wasm_len, request_json, err_out) };
    }
    let engine_ptr = unsafe { &*(engine as *const Engine) };
    let kill_ptr = unsafe { &*(kill as *const KillHandle) };
    let bytes = unsafe { slice_from_raw(wasm, wasm_len) };
    let request = match request_json_or_default(request_json) {
        Ok(request) => request,
        Err(message) => {
            write_err(err_out, &message);
            return std::ptr::null_mut();
        }
    };
    let run = match request_from(bytes, request) {
        Ok(run) => run,
        Err(err) => {
            write_err(err_out, &err);
            return std::ptr::null_mut();
        }
    };
    match engine_ptr.0.execute_with(run, &kill_ptr.0) {
        Ok(report) => CString::new(report.to_json())
            .unwrap_or_else(|_| CString::new("{}").unwrap())
            .into_raw(),
        Err(err) => {
            write_err(err_out, &err.to_string());
            std::ptr::null_mut()
        }
    }
}

/// Run a module. Returns a newly allocated JSON report, or NULL.
///
/// On NULL, `err_out` receives a newly allocated message when it is non-null.
/// Free both strings with [`boxwren_string_free`].
///
/// # Safety
/// `engine` is a live engine pointer. `wasm` points at `wasm_len` bytes.
/// `request_json` is null or a NUL-terminated UTF-8 string.
/// `err_out` is null or a writable pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn boxwren_execute(
    engine: *mut c_void,
    wasm: *const u8,
    wasm_len: usize,
    request_json: *const c_char,
    err_out: *mut *mut c_char,
) -> *mut c_char {
    let failed = |message: String| -> *mut c_char {
        write_err(err_out, &message);
        std::ptr::null_mut()
    };
    if engine.is_null() || (wasm.is_null() && wasm_len != 0) {
        return failed("null engine or module".into());
    }
    let engine = unsafe { &*(engine as *const Engine) };
    let bytes = unsafe { slice_from_raw(wasm, wasm_len) };
    let request = match request_json_or_default(request_json) {
        Ok(request) => request,
        Err(message) => return failed(message),
    };
    let run = match request_from(bytes, request) {
        Ok(run) => run,
        Err(err) => return failed(err),
    };
    match engine.0.execute(run) {
        Ok(report) => CString::new(report.to_json())
            .unwrap_or_else(|_| CString::new("{}").unwrap())
            .into_raw(),
        Err(err) => failed(err.to_string()),
    }
}

/// # Safety
/// `wasm` is null or points at `wasm_len` bytes.
unsafe fn slice_from_raw(wasm: *const u8, wasm_len: usize) -> &'static [u8] {
    if wasm_len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(wasm, wasm_len) }
    }
}

fn request_json_or_default(request_json: *const c_char) -> Result<RequestJson, String> {
    if request_json.is_null() {
        return Ok(RequestJson {
            entry: command_entry(),
            args: Vec::new(),
            fuel: None,
            memory_bytes: None,
            wall_ms: None,
            dirs: Vec::new(),
            net: Vec::new(),
            env: Vec::new(),
        });
    }
    let text = match unsafe { CStr::from_ptr(request_json) }.to_str() {
        Ok(text) => text,
        Err(_) => return Err("request json is not utf-8".into()),
    };
    serde_json::from_str::<RequestJson>(text).map_err(|err| err.to_string())
}

/// # Safety
/// `ptr` is null or a pointer returned by this library and not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn boxwren_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}

fn write_err(err_out: *mut *mut c_char, message: &str) {
    if err_out.is_null() {
        return;
    }
    let text = CString::new(message).unwrap_or_else(|_| CString::new("error").unwrap());
    unsafe {
        *err_out = text.into_raw();
    }
}

fn request_from(wasm: &[u8], request: RequestJson) -> Result<RunRequest, String> {
    let defaults = Limits::default();
    let limits = Limits {
        fuel: request.fuel.unwrap_or(defaults.fuel),
        memory_bytes: request.memory_bytes.unwrap_or(defaults.memory_bytes),
        wall_time: request
            .wall_ms
            .map(Duration::from_millis)
            .unwrap_or(defaults.wall_time),
        ..defaults
    };
    let mut grants = Grants::default();
    for spec in request.dirs {
        grants
            .dirs
            .push(DirGrant::parse(&spec).map_err(|err| err.to_string())?);
    }
    for spec in request.net {
        grants
            .net
            .push(NetGrant::parse(&spec).map_err(|err| err.to_string())?);
    }
    for spec in request.env {
        grants
            .env
            .push(EnvGrant::parse(&spec).map_err(|err| err.to_string())?);
    }
    let entry = if request.entry == "_start" {
        let mut args = request.args;
        if args.is_empty() {
            args.push("guest".into());
        }
        Entry::Command { args }
    } else {
        Entry::Export(request.entry)
    };
    Ok(RunRequest {
        wasm: wasm.to_vec(),
        entry,
        limits,
        grants,
        args: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_abi_reports_an_export() {
        let engine = boxwren_engine_new();
        assert!(!engine.is_null());
        let wat = br#"
            (module
              (func (export "run") (result i32)
                i32.const 2
                i32.const 2
                i32.add))
        "#;
        let request = CString::new(r#"{"entry":"run"}"#).unwrap();
        let mut err = std::ptr::null_mut();
        let raw =
            unsafe { boxwren_execute(engine, wat.as_ptr(), wat.len(), request.as_ptr(), &mut err) };
        assert!(!raw.is_null(), "err {}", unsafe { c_string(err) });
        let text = unsafe { c_string(raw) };
        assert!(text.contains("success"), "{text}");
        assert!(text.contains("4"), "{text}");
        unsafe {
            boxwren_string_free(raw);
            boxwren_engine_free(engine);
        }
    }

    #[test]
    fn c_abi_kill_stops_a_loop() {
        let engine = boxwren_engine_new();
        let kill = unsafe { boxwren_kill_new(engine) };
        assert!(!kill.is_null());
        let handle = kill as *mut KillHandle;
        let inner = unsafe { (*handle).0.clone() };
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(30));
            inner.kill();
        });
        let wat = br#"
            (module
              (func (export "run")
                (loop br 0)))
        "#;
        let request = CString::new(r#"{"entry":"run","wall_ms":3000}"#).unwrap();
        let mut err = std::ptr::null_mut();
        let raw = unsafe {
            boxwren_execute_with(
                engine,
                wat.as_ptr(),
                wat.len(),
                request.as_ptr(),
                kill,
                &mut err,
            )
        };
        assert!(!raw.is_null(), "err {}", unsafe { c_string(err) });
        let text = unsafe { c_string(raw) };
        assert!(text.contains("killed"), "{text}");
        unsafe {
            boxwren_string_free(raw);
            boxwren_kill_free(kill);
            boxwren_engine_free(engine);
        }
    }

    #[test]
    fn c_abi_rejects_a_missing_directory() {
        let engine = boxwren_engine_new();
        let request = CString::new(r#"{"dirs":["/no/such/boxwren-dir:guest"]}"#).unwrap();
        let mut err = std::ptr::null_mut();
        let raw =
            unsafe { boxwren_execute(engine, std::ptr::null(), 0, request.as_ptr(), &mut err) };
        assert!(raw.is_null());
        let message = unsafe { c_string(err) };
        assert!(message.contains("no/such"), "{message}");
        unsafe {
            boxwren_string_free(err);
            boxwren_engine_free(engine);
        }
    }

    unsafe fn c_string(ptr: *mut c_char) -> String {
        if ptr.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned()
    }
}
