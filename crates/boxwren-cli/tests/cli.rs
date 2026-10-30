use boxwren::{Decision, EventKind, Status};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_boxwren")
}

fn write_module(name: &str, wat: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "boxwren-cli-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(wat.as_bytes()).unwrap();
    path
}

#[test]
fn json_report_matches_a_wasi_exit() {
    let module = write_module(
        "exit.wat",
        r#"
        (module
          (import "wasi_snapshot_preview1" "proc_exit" (func $exit (param i32)))
          (memory (export "memory") 1)
          (func (export "_start")
            (call $exit (i32.const 0))))
        "#,
    );
    let output = Command::new(bin())
        .args(["run", module.to_str().unwrap(), "--report", "json"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "status {:?} stdout {} stderr {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report: boxwren::Report = serde_json::from_slice(&output.stdout).unwrap();
    assert!(report.success(), "{report:?}");
    assert_eq!(report.exit_code, Some(0));
    assert_eq!(report.schema_version, 1);
}

#[test]
fn invalid_grant_fails_before_the_module_runs() {
    let module = write_module("(module)", "(module)");
    let output = Command::new(bin())
        .args([
            "run",
            module.to_str().unwrap(),
            "--allow-dir",
            "/no/such/boxwren-dir:guest",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("no/such"), "{stderr}");
    assert!(output.stdout.is_empty());
}

#[test]
fn ctrl_c_kills_a_spinning_module() {
    let module = write_module(
        "spin.wat",
        r#"
        (module
          (func (export "_start")
            (loop br 0)))
        "#,
    );
    let mut child = Command::new(bin())
        .args([
            "run",
            module.to_str().unwrap(),
            "--report",
            "json",
            "--timeout",
            "8s",
            "--fuel",
            "4611686018427387903",
        ])
        .env("BOXWREN_READY", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let stderr = child.stderr.take().unwrap();
    let mut lines = BufReader::new(stderr).lines();
    let ready = lines.next().transpose().unwrap();
    assert_eq!(ready.as_deref(), Some("boxwren: ready"));
    drop(lines);
    let status = Command::new("kill")
        .args(["-INT", &child.id().to_string()])
        .status()
        .unwrap();
    assert!(status.success(), "kill failed");
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let report: boxwren::Report = serde_json::from_slice(stdout.as_bytes())
        .unwrap_or_else(|err| panic!("json {err}: stdout {stdout} stderr {stderr}"));
    assert!(
        matches!(report.status, Status::Killed),
        "{report:?} stderr {stderr}"
    );
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn demo_blocks_secret_network_and_memory() {
    let output = Command::new(bin())
        .args(["demo", "--report", "json"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stdout {stdout} stderr {stderr}");
    let mut reports = Vec::new();
    for chunk in stdout.split("# ").skip(1) {
        let json = chunk.split_once('\n').map(|(_, rest)| rest).unwrap_or("");
        let report: boxwren::Report =
            serde_json::from_str(json.trim()).unwrap_or_else(|err| panic!("json {err} in {chunk}"));
        reports.push(report);
    }
    assert_eq!(reports.len(), 3, "{stdout}");
    assert!(reports[0].events.iter().any(|event| {
        event.kind == EventKind::Filesystem
            && event.target.contains("passwd")
            && event.decision == Decision::Denied
    }));
    assert!(reports[1].events.iter().any(|event| {
        event.kind == EventKind::Network
            && event.target == "127.0.0.1:1"
            && event.decision == Decision::Denied
    }));
    assert!(matches!(reports[2].status, Status::MemoryLimited));
}
