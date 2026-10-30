use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    build_guests(
        &repo.join("tests/guests"),
        "wasm32-wasip1",
        &["secret", "memory"],
    );
    build_guests(&repo.join("tests/guests-p2"), "wasm32-wasip2", &["net"]);
}

fn build_guests(dir: &Path, target: &str, bins: &[&str]) {
    println!(
        "cargo:rerun-if-changed={}",
        dir.join("Cargo.toml").display()
    );
    println!("cargo:rerun-if-changed={}", dir.join("src").display());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let target_dir = out_dir.join("guest-target");
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let mut command = Command::new(cargo);
    command
        .args(["build", "--release", "--target", target, "--target-dir"])
        .arg(&target_dir)
        .current_dir(dir)
        .env_remove("CARGO_TARGET_DIR");
    for bin in bins {
        command.args(["--bin", bin]);
    }
    let status = command.status().expect("spawn guest build");
    if !status.success() {
        panic!("guest build for {target} failed");
    }
    for bin in bins {
        let wasm = target_dir
            .join(target)
            .join("release")
            .join(format!("{bin}.wasm"));
        std::fs::copy(&wasm, out_dir.join(format!("{bin}.wasm")))
            .unwrap_or_else(|err| panic!("copy {}: {err}", wasm.display()));
    }
}
