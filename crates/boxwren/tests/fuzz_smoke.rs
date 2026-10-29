//! Short fuzz of the execute edge. Random bytes must not panic.
//!
//! A longer run is `BOXWREN_FUZZ_ITERS=10000 cargo test -p boxwren --test fuzz_smoke -- --ignored --nocapture`.
//! Wasm semantics are Wasmtime's job; this only checks that Boxwren returns.

use boxwren::{Engine, Entry, Grants, Limits, RunRequest};

#[test]
fn random_modules_do_not_panic() {
    let engine = Engine::new().expect("engine");
    let mut rng = 0xB0E5_5EED_u64;
    for _ in 0..48 {
        let len = (rng % 2048) as usize;
        let bytes = next_bytes(&mut rng, len);
        execute(&engine, bytes);
    }
    execute(&engine, Vec::new());
    execute(&engine, b"\0asm".to_vec());
    execute(&engine, b"\0asm\x01\x00\x00\x00".to_vec());
    execute(
        &engine,
        vec![0x00, b'a', b's', b'm', 0x0d, 0x00, 0x01, 0x00, 0xff],
    );
}

#[test]
#[ignore]
fn longer_random_modules() {
    let iters: usize = std::env::var("BOXWREN_FUZZ_ITERS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(2_000);
    let engine = Engine::new().expect("engine");
    let mut rng = 0xF00D_u64;
    for _ in 0..iters {
        let len = (rng % 8192) as usize;
        let bytes = next_bytes(&mut rng, len);
        execute(&engine, bytes);
    }
}

fn execute(engine: &Engine, wasm: Vec<u8>) {
    let limits = Limits {
        wall_time: std::time::Duration::from_millis(200),
        fuel: 50_000,
        max_module_bytes: 64 * 1024,
        ..Limits::default()
    };
    let _ = engine.execute(RunRequest {
        wasm,
        entry: Entry::Command {
            args: vec!["fuzz".into()],
        },
        limits,
        grants: Grants::default(),
        args: Vec::new(),
    });
}

fn next_bytes(rng: &mut u64, len: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(len);
    for _ in 0..len {
        *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        bytes.push(*rng as u8);
    }
    bytes
}
