use std::io::Write;
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use boxwren::{Decision, DirGrant, Entry, EventKind, Grants, Limits, RunRequest, Status};

fn engine() -> &'static boxwren::Engine {
    static ENGINE: OnceLock<boxwren::Engine> = OnceLock::new();
    ENGINE.get_or_init(|| boxwren::Engine::new().expect("engine"))
}

fn guest_wasm(dir: &str, name: &str, target: &str) -> Vec<u8> {
    // Parallel cargo builds into the same target-dir race and flake on CI.
    static GUEST_BUILD: Mutex<()> = Mutex::new(());
    let _guard = GUEST_BUILD.lock().unwrap_or_else(|poison| poison.into_inner());

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let guest_dir = manifest_dir.join("../../tests").join(dir);
    let target_dir = manifest_dir.join("../../target/guests").join(target);
    let status = Command::new("cargo")
        .args(["build", "--target", target, "--target-dir"])
        .arg(&target_dir)
        .args(["--bin", name])
        .current_dir(&guest_dir)
        .status()
        .expect("spawn guest build");
    assert!(status.success(), "build {name} for {target}");
    let wasm = target_dir
        .join(target)
        .join("debug")
        .join(format!("{name}.wasm"));
    std::fs::read(&wasm).unwrap_or_else(|err| panic!("read {}: {err}", wasm.display()))
}

fn p1(name: &str) -> Vec<u8> {
    guest_wasm("guests", name, "wasm32-wasip1")
}

fn p2(name: &str) -> Vec<u8> {
    guest_wasm("guests-p2", name, "wasm32-wasip2")
}

fn run(wasm: Vec<u8>, grants: Grants, args: Vec<String>, limits: Limits) -> boxwren::Report {
    // Guest builds under a mutex still leave CI runners busy; give WASI guests
    // more wall than the library default so epoch lag is not a flake.
    let limits = Limits {
        wall_time: limits.wall_time.max(Duration::from_secs(10)),
        ..limits
    };
    engine()
        .execute(RunRequest {
            wasm,
            entry: Entry::Command {
                args: {
                    let mut all = vec!["guest".into()];
                    all.extend(args);
                    all
                },
            },
            limits,
            grants,
            args: Vec::new(),
        })
        .expect("execute")
}

#[test]
fn happy_path_prints_and_exits() {
    let report = run(
        p1("happy"),
        Grants::default(),
        Vec::new(),
        Limits::default(),
    );
    assert!(report.success(), "{report:?}");
    assert_eq!(report.exit_code, Some(0));
    assert!(report.stdout.contains("hello-from-guest"), "{report:?}");
}

#[test]
fn secret_read_is_blocked_and_named() {
    let report = run(
        p1("secret"),
        Grants::default(),
        Vec::new(),
        Limits::default(),
    );
    assert!(!report.stdout.contains("root:"), "{report:?}");
    assert!(
        report.events.iter().any(|event| {
            event.kind == EventKind::Filesystem
                && event.target.contains("passwd")
                && event.decision == Decision::Denied
        }),
        "{report:?}"
    );
}

#[test]
fn granted_dir_can_be_read_and_not_escaped() {
    let root = tempfile_dir();
    let sandbox = root.join("box");
    let outside = root.join("secret.txt");
    std::fs::create_dir_all(&sandbox).unwrap();
    std::fs::write(sandbox.join("hello.txt"), "hello-from-host").unwrap();
    std::fs::write(&outside, "super-secret").unwrap();

    let mut grants = Grants::default();
    grants.dirs.push(DirGrant {
        host: sandbox.clone(),
        guest: ".".into(),
        writable: false,
    });
    let report = run(p1("read"), grants, Vec::new(), Limits::default());
    assert!(report.stdout.contains("hello-from-host"), "{report:?}");
    assert!(
        report.events.iter().any(|event| {
            event.kind == EventKind::Filesystem
                && event.target.contains("hello.txt")
                && event.decision == Decision::Allowed
        }),
        "{report:?}"
    );

    let mut grants = Grants::default();
    grants.dirs.push(DirGrant {
        host: sandbox,
        guest: ".".into(),
        writable: false,
    });
    let report = run(p1("escape"), grants, Vec::new(), Limits::default());
    assert!(!report.stdout.contains("super-secret"), "{report:?}");
    assert!(
        report.events.iter().any(|event| {
            event.kind == EventKind::Filesystem && event.decision == Decision::Denied
        }),
        "{report:?}"
    );
}

#[test]
fn read_only_grant_rejects_writes() {
    let dir = tempfile_dir();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("note.txt"), "original").unwrap();
    let mut grants = Grants::default();
    grants.dirs.push(DirGrant {
        host: dir.clone(),
        guest: ".".into(),
        writable: false,
    });
    let report = run(p1("write_ro"), grants, Vec::new(), Limits::default());
    assert!(!report.stdout.contains("WROTE"), "{report:?}");
    assert_eq!(
        std::fs::read_to_string(dir.join("note.txt")).unwrap(),
        "original"
    );
}

#[test]
fn read_write_grant_allows_writes() {
    let dir = tempfile_dir();
    std::fs::create_dir_all(&dir).unwrap();
    let mut grants = Grants::default();
    grants.dirs.push(DirGrant {
        host: dir.clone(),
        guest: ".".into(),
        writable: true,
    });
    let report = run(p1("write_ro"), grants, Vec::new(), Limits::default());
    assert!(report.stdout.contains("WROTE"), "{report:?}");
    assert_eq!(
        std::fs::read_to_string(dir.join("note.txt")).unwrap(),
        "pwned"
    );
}

#[test]
fn environment_is_not_inherited_unless_granted() {
    // SAFETY: the test process is single-threaded for this variable.
    unsafe { std::env::set_var("BOXWREN_SECRET", "s3cret-value") };
    let report = run(p1("env"), Grants::default(), Vec::new(), Limits::default());
    assert!(!report.stdout.contains("s3cret-value"), "{report:?}");
    assert!(report.stdout.contains("absent"), "{report:?}");

    let mut grants = Grants::default();
    grants.env.push(boxwren::EnvGrant {
        name: "BOXWREN_SECRET".into(),
        value: "s3cret-value".into(),
    });
    let report = run(p1("env"), grants, Vec::new(), Limits::default());
    assert!(report.stdout.contains("LEAKED s3cret-value"), "{report:?}");
    assert!(
        report.events.iter().any(|event| {
            event.kind == EventKind::Env
                && event.target == "BOXWREN_SECRET"
                && event.decision == Decision::Allowed
        }),
        "{report:?}"
    );
}

#[test]
fn memory_growth_is_limited() {
    let limits = Limits {
        memory_bytes: 4 * 1024 * 1024,
        wall_time: Duration::from_secs(5),
        ..Limits::default()
    };
    let report = run(p1("memory"), Grants::default(), Vec::new(), limits);
    assert!(matches!(report.status, Status::MemoryLimited), "{report:?}");
    assert!(!report.stdout.contains("allocated 64"), "{report:?}");
}

#[test]
fn network_is_denied_unless_allowlisted() {
    let report = run(
        p2("net"),
        Grants::default(),
        vec!["127.0.0.1:1".into()],
        Limits::default(),
    );
    assert!(!report.stdout.contains("CONNECTED"), "{report:?}");
    assert!(
        report.events.iter().any(|event| {
            event.kind == EventKind::Network
                && event.target == "127.0.0.1:1"
                && event.decision == Decision::Denied
        }),
        "{report:?}"
    );

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let addr = format!("127.0.0.1:{port}");
    std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let _ = stream.write_all(b"ok");
        }
    });
    let mut grants = Grants::default();
    grants.net.push(boxwren::NetGrant::parse(&addr).unwrap());
    let report = run(p2("net"), grants, vec![addr], Limits::default());
    assert!(
        report.events.iter().any(|event| {
            event.kind == EventKind::Network && event.decision == Decision::Allowed
        }),
        "{report:?}"
    );
    assert!(report.stdout.contains("CONNECTED"), "{report:?}");
}

#[test]
fn invalid_grant_fails_before_the_guest_runs() {
    let err = engine()
        .execute(RunRequest {
            wasm: b"(module)".to_vec(),
            entry: Entry::Export("run".into()),
            limits: Limits::default(),
            grants: Grants {
                dirs: vec![DirGrant {
                    host: PathBuf::from("/no/such/boxwren-dir"),
                    guest: ".".into(),
                    writable: false,
                }],
                net: Vec::new(),
                env: Vec::new(),
            },
            args: Vec::new(),
        })
        .expect_err("missing directory");
    let message = err.to_string();
    assert!(message.contains("no/such"), "{message}");
}

fn tempfile_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "boxwren-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
