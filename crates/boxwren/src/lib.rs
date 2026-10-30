//! Sandboxed execution for untrusted WebAssembly.
//!
//! Boxwren runs a guest inside Wasmtime with deny-by-default limits.
//! Reuse an [`Engine`] across runs. Each [`RunRequest`] gets its own store,
//! fuel budget, memory cap, and wall-clock deadline.

#![forbid(unsafe_code)]

mod caps;
mod capture;
mod engine;
mod error;
mod limiter;
mod limits;
mod report;
mod run;
mod wasi;

pub use caps::{DirGrant, EnvGrant, Grants, NetGrant};
pub use engine::{Engine, KillHandle};
pub use error::Error;
pub use limits::Limits;
pub use report::{Decision, Event, EventKind, Report, SCHEMA_VERSION, Status};
pub use run::{Entry, RunRequest};

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn engine() -> Engine {
        Engine::new().expect("engine")
    }

    fn run(engine: &Engine, wat: &str, limits: Limits) -> Report {
        engine
            .execute(RunRequest {
                wasm: wat.as_bytes().to_vec(),
                entry: Entry::Export("run".into()),
                limits,
                grants: Grants::default(),
                args: Vec::new(),
            })
            .expect("execute")
    }

    #[test]
    fn add_returns_four() {
        let engine = engine();
        let report = run(
            &engine,
            r#"
            (module
              (func (export "run") (result i32)
                i32.const 2
                i32.const 2
                i32.add))
            "#,
            Limits::default(),
        );
        assert!(report.success(), "{report:?}");
        assert_eq!(report.return_value, Some(4));
        assert!(report.fuel_used > 0);
        assert!(report.events.is_empty());
        assert_eq!(report.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn fuel_starvation_is_reported() {
        let engine = engine();
        let limits = Limits {
            fuel: 50,
            wall_time: Duration::from_secs(5),
            ..Limits::default()
        };
        let report = run(
            &engine,
            r#"
            (module
              (func (export "run")
                (loop br 0)))
            "#,
            limits,
        );
        assert!(matches!(report.status, Status::FuelExhausted), "{report:?}");
    }

    #[test]
    fn oversized_memory_is_limited() {
        let engine = engine();
        let limits = Limits {
            memory_bytes: 64 * 1024,
            ..Limits::default()
        };
        let report = run(
            &engine,
            r#"
            (module
              (memory (export "mem") 2)
              (func (export "run")))
            "#,
            limits,
        );
        assert!(matches!(report.status, Status::MemoryLimited), "{report:?}");
    }

    #[test]
    fn module_over_the_size_cap_is_rejected() {
        let engine = engine();
        let limits = Limits {
            max_module_bytes: 8,
            ..Limits::default()
        };
        let report = run(&engine, "(module)", limits);
        assert!(
            matches!(report.status, Status::Rejected { .. }),
            "{report:?}"
        );
    }

    #[test]
    fn kill_is_not_a_timeout() {
        let engine = engine();
        let kill = engine.kill_handle();
        let killer = kill.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(30));
            killer.kill();
        });
        let started = std::time::Instant::now();
        let limits = Limits {
            wall_time: Duration::from_secs(3),
            fuel: u64::MAX / 4,
            ..Limits::default()
        };
        let report = engine
            .execute_with(
                RunRequest {
                    wasm: br#"
                    (module
                      (func (export "run")
                        (loop br 0)))
                    "#
                    .to_vec(),
                    entry: Entry::Export("run".into()),
                    limits,
                    grants: Grants::default(),
                    args: Vec::new(),
                },
                &kill,
            )
            .expect("execute");
        assert!(matches!(report.status, Status::Killed), "{report:?}");
        assert!(
            started.elapsed() < Duration::from_millis(500),
            "{:?}",
            started.elapsed()
        );
    }

    #[test]
    fn wall_clock_timeout_is_not_a_kill() {
        let engine = engine();
        let limits = Limits {
            wall_time: Duration::from_millis(200),
            fuel: u64::MAX / 4,
            ..Limits::default()
        };
        let started = std::time::Instant::now();
        let report = run(
            &engine,
            r#"
            (module
              (func (export "run")
                (loop br 0)))
            "#,
            limits,
        );
        assert!(matches!(report.status, Status::TimedOut), "{report:?}");
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    fn command(engine: &Engine, wasm: &[u8], grants: Grants) -> Report {
        engine
            .execute(RunRequest {
                wasm: wasm.to_vec(),
                entry: Entry::Command {
                    args: vec!["guest".into()],
                },
                limits: Limits::default(),
                grants,
                args: Vec::new(),
            })
            .expect("execute")
    }

    #[test]
    fn path_open_is_denied_and_named() {
        let engine = engine();
        let wat = r#"
            (module
              (import "wasi_snapshot_preview1" "path_open"
                (func $path_open (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
              (import "wasi_snapshot_preview1" "proc_exit"
                (func $exit (param i32)))
              (memory (export "memory") 1)
              (data (i32.const 8) "/etc/passwd")
              (func (export "_start")
                (call $path_open
                  (i32.const 3)
                  (i32.const 0)
                  (i32.const 8)
                  (i32.const 11)
                  (i32.const 0)
                  (i64.const 0)
                  (i64.const 0)
                  (i32.const 0)
                  (i32.const 0))
                drop
                (call $exit (i32.const 0))))
        "#;
        let report = command(&engine, wat.as_bytes(), Grants::default());
        assert!(
            report.events.iter().any(|event| {
                event.kind == EventKind::Filesystem
                    && event.target == "/etc/passwd"
                    && event.decision == Decision::Denied
            }),
            "{report:?}"
        );
        assert!(!report.stdout.contains("root:"));
    }
}
