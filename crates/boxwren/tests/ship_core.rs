//! Contract proofs the library unit tests do not name on their own.

use std::time::Duration;

use boxwren::{Engine, Entry, Grants, Limits, RunRequest, Status};

fn engine() -> Engine {
    Engine::new().expect("engine")
}

fn execute(engine: &Engine, wat: &str, limits: Limits) -> boxwren::Report {
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
fn one_engine_runs_twice() {
    let engine = engine();
    let wat = r#"
        (module
          (func (export "run") (result i32)
            i32.const 2
            i32.const 2
            i32.add))
    "#;
    let first = execute(&engine, wat, Limits::default());
    let second = execute(&engine, wat, Limits::default());
    assert_eq!(first.return_value, Some(4), "{first:?}");
    assert_eq!(second.return_value, Some(4), "{second:?}");
    assert!(first.fuel_used > 0 && second.fuel_used > 0);
}

#[test]
fn wasi_preview1_command_records_exit_code() {
    let engine = engine();
    let report = engine
        .execute(RunRequest {
            wasm: br#"
                (module
                  (import "wasi_snapshot_preview1" "proc_exit" (func $exit (param i32)))
                  (memory (export "memory") 1)
                  (func (export "_start")
                    (call $exit (i32.const 7))))
            "#
            .to_vec(),
            entry: Entry::Command {
                args: vec!["guest".into()],
            },
            limits: Limits::default(),
            grants: Grants::default(),
            args: Vec::new(),
        })
        .expect("execute");
    assert!(report.success(), "{report:?}");
    assert_eq!(report.exit_code, Some(7));
}

#[test]
fn unreachable_is_trapped() {
    let report = execute(
        &engine(),
        r#"
        (module
          (func (export "run")
            unreachable))
        "#,
        Limits::default(),
    );
    assert!(
        matches!(report.status, Status::Trapped { .. }),
        "{report:?}"
    );
}

#[test]
fn stdio_past_the_cap_is_truncated() {
    let engine = engine();
    let limits = Limits {
        max_stdio_bytes: 4,
        ..Limits::default()
    };
    let report = engine
        .execute(RunRequest {
            wasm: br#"
                (module
                  (import "wasi_snapshot_preview1" "fd_write"
                    (func $fd_write (param i32 i32 i32 i32) (result i32)))
                  (import "wasi_snapshot_preview1" "proc_exit"
                    (func $exit (param i32)))
                  (memory (export "memory") 1)
                  (data (i32.const 8) "abcdefghijklmnopqrstuvwxyz")
                  (data (i32.const 64) "\08\00\00\00\1a\00\00\00")
                  (func (export "_start")
                    (call $fd_write
                      (i32.const 1)
                      (i32.const 64)
                      (i32.const 1)
                      (i32.const 128))
                    drop
                    (call $exit (i32.const 0))))
            "#
            .to_vec(),
            entry: Entry::Command {
                args: vec!["guest".into()],
            },
            limits,
            grants: Grants::default(),
            args: Vec::new(),
        })
        .expect("execute");
    assert!(report.success(), "{report:?}");
    assert!(report.stdout_truncated, "{report:?}");
    assert!(report.stdout.len() <= 4, "{report:?}");
    assert!(!report.stdout.contains("uvwxyz"), "{report:?}");
}

#[test]
fn five_memories_are_refused() {
    let report = execute(
        &engine(),
        r#"
        (module
          (memory 1)
          (memory 1)
          (memory 1)
          (memory 1)
          (memory 1)
          (func (export "run")))
        "#,
        Limits::default(),
    );
    assert!(matches!(report.status, Status::MemoryLimited), "{report:?}");
}

#[test]
fn report_json_is_schema_v1() {
    let report = execute(
        &engine(),
        r#"
        (module
          (func (export "run") (result i32)
            i32.const 4))
        "#,
        Limits::default(),
    );
    let value: serde_json::Value = serde_json::from_str(&report.to_json()).expect("json");
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["status"]["kind"], "success");
    assert!(value["fuel_used"].as_u64().unwrap() > 0);
    assert!(value.get("peak_memory").is_some());
    assert!(value.get("duration_ms").is_some());
    assert!(value["events"].as_array().unwrap().is_empty());
    assert!(value["warnings"].as_array().unwrap().is_empty());
    assert_eq!(value["return_value"], 4);
}

#[test]
fn killing_one_run_leaves_its_sibling() {
    let engine = engine();
    let kill = engine.kill_handle();
    let killer = kill.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(40));
        killer.kill();
    });
    let sibling_engine = engine.clone();
    let sibling = std::thread::spawn(move || {
        sibling_engine
            .execute(RunRequest {
                wasm: br#"
                    (module
                      (func (export "run") (result i32)
                        i32.const 2
                        i32.const 2
                        i32.add))
                "#
                .to_vec(),
                entry: Entry::Export("run".into()),
                limits: Limits::default(),
                grants: Grants::default(),
                args: Vec::new(),
            })
            .expect("sibling")
    });
    let killed = engine
        .execute_with(
            RunRequest {
                wasm: br#"
                    (module
                      (func (export "run")
                        (loop br 0)))
                "#
                .to_vec(),
                entry: Entry::Export("run".into()),
                limits: Limits {
                    wall_time: Duration::from_secs(5),
                    fuel: u64::MAX / 4,
                    ..Limits::default()
                },
                grants: Grants::default(),
                args: Vec::new(),
            },
            &kill,
        )
        .expect("killed run");
    let sibling_report = sibling.join().expect("sibling thread");
    assert!(matches!(killed.status, Status::Killed), "{killed:?}");
    assert!(sibling_report.success(), "{sibling_report:?}");
    assert_eq!(sibling_report.return_value, Some(4));
}
