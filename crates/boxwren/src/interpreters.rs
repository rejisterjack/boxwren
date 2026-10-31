//! Optional WASI guest interpreters for Python and JavaScript source.
//!
//! These are **guest** modules under the same deny-by-default limits. They are
//! not a host-embedded CPython or V8. Enable with Cargo feature `interpreters`.

use std::fs;
use std::path::{Path, PathBuf};

use wasmtime::{Caller, Linker, Store};
use wasmtime_wasi::p1::WasiP1Ctx;
use wasmtime_wasi::{FsPerms, WasiCtxBuilder};

use crate::caps::{DirGrant, Grants};
use crate::engine::{Engine, KillHandle};
use crate::error::Error;
use crate::limits::Limits;
use crate::report::Report;
use crate::run::{Entry, HasControl, RunControl, RunRequest, arm, report_from};

const MICROPYTHON: &[u8] = include_bytes!("../../../guests/interpreters/micropython-wasi.wasm");
const QUICKJS: &[u8] = include_bytes!("../../../guests/interpreters/qjs-wasi.wasm");

/// MicroPython WASI module bytes (feature `interpreters`).
pub fn micropython_wasm() -> &'static [u8] {
    MICROPYTHON
}

/// QuickJS WASI module bytes (feature `interpreters`).
pub fn quickjs_wasm() -> &'static [u8] {
    QUICKJS
}

/// Run a Python source string via the bundled MicroPython WASI guest.
pub fn execute_python(
    engine: &Engine,
    source: &str,
    limits: Limits,
    grants: Grants,
) -> Result<Report, Error> {
    execute_micropython(engine, &engine.kill_handle(), source, limits, grants)
}

/// Run a JavaScript source file via the bundled QuickJS WASI guest.
pub fn execute_javascript(
    engine: &Engine,
    source: &str,
    limits: Limits,
    mut grants: Grants,
) -> Result<Report, Error> {
    let dir = tempfile_dir()?;
    let script = dir.join("script.js");
    fs::write(&script, source).map_err(|err| Error::BadGrant(format!("write script: {err}")))?;
    grants.dirs.push(DirGrant {
        host: dir.clone(),
        guest: "/work".into(),
        writable: false,
    });
    let report = engine.execute(RunRequest {
        wasm: QUICKJS.to_vec(),
        entry: Entry::Command {
            args: vec!["qjs".into(), "/work/script.js".into()],
        },
        limits,
        grants,
        args: Vec::new(),
    })?;
    let _ = fs::remove_dir_all(&dir);
    Ok(report)
}

/// Run a Python file from the host (file is read on the host, then `-c`).
pub fn execute_python_file(
    engine: &Engine,
    path: &Path,
    limits: Limits,
    grants: Grants,
) -> Result<Report, Error> {
    let source = fs::read_to_string(path)
        .map_err(|err| Error::BadGrant(format!("read {}: {err}", path.display())))?;
    execute_python(engine, &source, limits, grants)
}

/// Run a JavaScript file from the host.
pub fn execute_javascript_file(
    engine: &Engine,
    path: &Path,
    limits: Limits,
    grants: Grants,
) -> Result<Report, Error> {
    let source = fs::read_to_string(path)
        .map_err(|err| Error::BadGrant(format!("read {}: {err}", path.display())))?;
    execute_javascript(engine, &source, limits, grants)
}

struct MpState {
    control: RunControl,
    wasi: WasiP1Ctx,
}

impl HasControl for MpState {
    fn control_mut(&mut self) -> &mut RunControl {
        &mut self.control
    }
}

/// The simonw/micropython-wasm artifact imports two host helpers used by its
/// session API. We stub them so `-c` one-shots run as a normal WASI command.
fn execute_micropython(
    engine: &Engine,
    kill: &KillHandle,
    source: &str,
    limits: Limits,
    grants: Grants,
) -> Result<Report, Error> {
    let warnings = grants.validate()?;
    if MICROPYTHON.len() > limits.max_module_bytes {
        return Ok(crate::run::rejected_public(
            warnings,
            format!(
                "module is {} bytes, over the {} byte cap",
                MICROPYTHON.len(),
                limits.max_module_bytes
            ),
        ));
    }

    // Compile before arming the store so wall-clock does not include Cranelift
    // time for the ~370 KiB MicroPython guest (epoch ticks only after arm).
    let module = engine.module(MICROPYTHON).map_err(Error::from)?;

    let control = RunControl::new(&limits, kill.flag());
    let stdout = control.stdout.clone();
    let stderr = control.stderr.clone();
    let mut builder = WasiCtxBuilder::new();
    builder.stdout(stdout);
    builder.stderr(stderr);
    builder.args(&["micropython", "-c", source]);
    for grant in &grants.env {
        builder.env(&grant.name, &grant.value);
    }
    if let Some(dir) = grants.dirs.first() {
        builder.initial_cwd(&dir.guest);
    }
    for dir in &grants.dirs {
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
    builder.allow_tcp(false);
    builder.allow_udp(false);
    builder.allow_ip_name_lookup(false);

    let mut store = Store::new(
        engine.wasm(),
        MpState {
            control,
            wasi: builder.build_p1(),
        },
    );
    arm(&mut store, limits.fuel)?;
    let started = store.data().control.started;

    let mut linker = Linker::new(engine.wasm());
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |state: &mut MpState| &mut state.wasi)?;
    // Stubs for the simonw host-call user module. One-shot `-c` runs never
    // need them; signatures must still match the Wasm imports.
    linker.func_wrap("micropython_wasm", "host_result_cap", || -> i32 {
        256 * 1024
    })?;
    linker.func_wrap(
        "micropython_wasm",
        "host_call",
        |_caller: Caller<'_, MpState>,
         _name_ptr: i32,
         _name_len: i32,
         _payload_ptr: i32,
         _payload_len: i32,
         _result_ptr: i32,
         _result_cap: i32|
         -> i32 { -1 },
    )?;

    let instance = match linker.instantiate(&mut store, &module) {
        Ok(instance) => instance,
        Err(err) => {
            return Ok(report_from(
                store.get_fuel().unwrap_or(limits.fuel),
                &store.data().control,
                &RunRequest {
                    wasm: MICROPYTHON.to_vec(),
                    entry: Entry::Command {
                        args: vec!["micropython".into()],
                    },
                    limits,
                    grants,
                    args: Vec::new(),
                },
                warnings,
                started,
                None,
                Err(err),
            ));
        }
    };
    let Some(start) = instance.get_func(&mut store, "_start") else {
        return Ok(crate::run::rejected_public(
            warnings,
            "micropython module does not export `_start`".into(),
        ));
    };
    let result = start.call(&mut store, &[], &mut []);
    Ok(report_from(
        store.get_fuel().unwrap_or(limits.fuel),
        &store.data().control,
        &RunRequest {
            wasm: MICROPYTHON.to_vec(),
            entry: Entry::Command {
                args: vec!["micropython".into()],
            },
            limits,
            grants,
            args: Vec::new(),
        },
        warnings,
        started,
        None,
        result.map(|_| ()),
    ))
}

fn tempfile_dir() -> Result<PathBuf, Error> {
    let dir = std::env::temp_dir().join(format!(
        "boxwren-js-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&dir).map_err(|err| Error::BadGrant(format!("temp dir: {err}")))?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn python_hello() {
        let engine = Engine::new().unwrap();
        let report = execute_python(
            &engine,
            "print('hello-from-py')",
            Limits {
                wall_time: Duration::from_secs(5),
                ..Limits::default()
            },
            Grants::default(),
        )
        .unwrap();
        assert!(report.success(), "{report:?}");
        assert!(report.stdout.contains("hello-from-py"), "{report:?}");
    }

    #[test]
    fn python_secret_read_stays_blocked() {
        let engine = Engine::new().unwrap();
        let report = execute_python(
            &engine,
            "print(open('/etc/passwd').read())",
            Limits {
                wall_time: Duration::from_secs(5),
                ..Limits::default()
            },
            Grants::default(),
        )
        .unwrap();
        assert!(!report.stdout.contains("root:"), "{report:?}");
    }

    #[test]
    fn javascript_hello() {
        let engine = Engine::new().unwrap();
        let report = execute_javascript(
            &engine,
            "console.log('hello-from-js');",
            Limits {
                wall_time: Duration::from_secs(5),
                ..Limits::default()
            },
            Grants::default(),
        )
        .unwrap();
        assert!(report.success(), "{report:?}");
        assert!(report.stdout.contains("hello-from-js"), "{report:?}");
    }
}
