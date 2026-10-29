//! In-process Python bindings. They wrap [`boxwren::Engine`] and
//! [`boxwren::RunRequest`]. They do not add capability kinds.

use std::time::Duration;

use boxwren::{DirGrant, Entry, EnvGrant, Grants, Limits, NetGrant, RunRequest};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyModule;

#[pyclass]
struct Engine {
    inner: boxwren::Engine,
}

#[pyclass]
struct KillHandle {
    inner: boxwren::KillHandle,
}

#[pymethods]
impl KillHandle {
    fn kill(&self) {
        self.inner.kill();
    }
}

#[pymethods]
impl Engine {
    #[new]
    fn new() -> PyResult<Self> {
        boxwren::Engine::new()
            .map(|inner| Self { inner })
            .map_err(runtime)
    }

    fn kill_handle(&self) -> KillHandle {
        KillHandle {
            inner: self.inner.kill_handle(),
        }
    }

    /// Run a module. `entry` is `_start` for a WASI command, or an export name.
    /// `dirs`, `net`, and `env` use the same grant syntax as the CLI.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        wasm,
        *,
        entry = "_start",
        args = None,
        fuel = None,
        memory_bytes = None,
        wall_ms = None,
        dirs = None,
        net = None,
        env = None,
        kill = None,
    ))]
    fn execute<'py>(
        &self,
        py: Python<'py>,
        wasm: &[u8],
        entry: &str,
        args: Option<Vec<String>>,
        fuel: Option<u64>,
        memory_bytes: Option<usize>,
        wall_ms: Option<u64>,
        dirs: Option<Vec<String>>,
        net: Option<Vec<String>>,
        env: Option<Vec<String>>,
        kill: Option<&KillHandle>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = request_from(
            wasm,
            entry,
            args,
            fuel,
            memory_bytes,
            wall_ms,
            dirs,
            net,
            env,
        )?;
        let report = match kill {
            Some(kill) => self.inner.execute_with(request, &kill.inner),
            None => self.inner.execute(request),
        }
        .map_err(runtime)?;
        let json = PyModule::import(py, "json")?;
        json.call_method1("loads", (report.to_json(),))
    }
}

#[pymodule]
fn boxwren_py(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Engine>()?;
    module.add_class::<KillHandle>()?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn request_from(
    wasm: &[u8],
    entry: &str,
    args: Option<Vec<String>>,
    fuel: Option<u64>,
    memory_bytes: Option<usize>,
    wall_ms: Option<u64>,
    dirs: Option<Vec<String>>,
    net: Option<Vec<String>>,
    env: Option<Vec<String>>,
) -> PyResult<RunRequest> {
    let defaults = Limits::default();
    let limits = Limits {
        fuel: fuel.unwrap_or(defaults.fuel),
        memory_bytes: memory_bytes.unwrap_or(defaults.memory_bytes),
        wall_time: wall_ms
            .map(Duration::from_millis)
            .unwrap_or(defaults.wall_time),
        ..defaults
    };
    let mut grants = Grants::default();
    for spec in dirs.unwrap_or_default() {
        grants.dirs.push(DirGrant::parse(&spec).map_err(runtime)?);
    }
    for spec in net.unwrap_or_default() {
        grants.net.push(NetGrant::parse(&spec).map_err(runtime)?);
    }
    for spec in env.unwrap_or_default() {
        grants.env.push(EnvGrant::parse(&spec).map_err(runtime)?);
    }
    let entry = if entry == "_start" {
        let mut argv = args.unwrap_or_default();
        if argv.is_empty() {
            argv.push("guest".into());
        }
        Entry::Command { args: argv }
    } else {
        Entry::Export(entry.to_string())
    };
    Ok(RunRequest {
        wasm: wasm.to_vec(),
        entry,
        limits,
        grants,
        args: Vec::new(),
    })
}

fn runtime(err: impl ToString) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn python_export_returns_four() {
        Python::attach(|py| {
            let engine = Engine::new().unwrap();
            let report = engine
                .execute(
                    py,
                    br#"
                    (module
                      (func (export "run") (result i32)
                        i32.const 2
                        i32.const 2
                        i32.add))
                    "#,
                    "run",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
                .unwrap();
            let value: i64 = report.get_item("return_value").unwrap().extract().unwrap();
            assert_eq!(value, 4);
        });
    }

    #[test]
    fn python_kill_stops_a_loop() {
        Python::attach(|py| {
            let engine = Engine::new().unwrap();
            let kill = engine.kill_handle();
            let flag = kill.inner.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(30));
                flag.kill();
            });
            let report = engine
                .execute(
                    py,
                    br#"
                    (module
                      (func (export "run")
                        (loop br 0)))
                    "#,
                    "run",
                    None,
                    Some(u64::MAX / 4),
                    None,
                    Some(3_000),
                    None,
                    None,
                    None,
                    Some(&kill),
                )
                .unwrap();
            let status = report.get_item("status").unwrap();
            let kind: String = status.get_item("kind").unwrap().extract().unwrap();
            assert_eq!(kind, "killed");
        });
    }
}
