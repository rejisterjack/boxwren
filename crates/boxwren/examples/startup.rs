//! Warm-engine timing. Cold process and container numbers are measured outside
//! this process and written up in docs/BENCHMARKS.md.

use std::time::Instant;

use boxwren::{Engine, Entry, Grants, Limits, RunRequest};

const WAT: &str = r#"
(module
  (func (export "run") (result i32)
    i32.const 2
    i32.const 2
    i32.add))
"#;

fn main() {
    let created = Instant::now();
    let engine = Engine::new().expect("engine");
    println!("engine_new_us {}", created.elapsed().as_micros());
    let request = || RunRequest {
        wasm: WAT.as_bytes().to_vec(),
        entry: Entry::Export("run".into()),
        limits: Limits::default(),
        grants: Grants::default(),
        args: Vec::new(),
    };
    for _ in 0..20 {
        let report = engine.execute(request()).expect("warmup");
        assert!(report.success());
    }
    let rounds: u32 = 200;
    let started = Instant::now();
    for _ in 0..rounds {
        let report = engine.execute(request()).expect("run");
        assert!(report.success());
    }
    let elapsed = started.elapsed();
    println!(
        "warm_engine_reuse_ns {} total_us {}",
        elapsed.as_nanos() / u128::from(rounds),
        elapsed.as_micros()
    );
}
