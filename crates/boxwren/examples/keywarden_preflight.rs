//! Show how a Keywarden-style policy can refuse a run before the guest starts.

use boxwren::{
    Entry, Error, Grants, Limits, PolicyDeny, Preflight, RunRequest,
};

struct DenyExports;

impl Preflight for DenyExports {
    fn allow(&self, request: &RunRequest) -> Result<(), PolicyDeny> {
        if matches!(request.entry, Entry::Export(_)) {
            return Err(PolicyDeny::new(
                "keywarden: export entry is not allowed for this agent",
            ));
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = boxwren::Engine::new()?;
    let kill = engine.kill_handle();
    let request = RunRequest {
        wasm: br#"(module (func (export "run") (result i32) i32.const 1))"#.to_vec(),
        entry: Entry::Export("run".into()),
        limits: Limits::default(),
        grants: Grants::default(),
        args: Vec::new(),
    };
    match engine.execute_with_preflight(request, &kill, &DenyExports) {
        Err(Error::Policy(message)) => {
            println!("denied before start: {message}");
            Ok(())
        }
        Ok(report) => {
            eprintln!("expected policy deny, got {report:?}");
            Err("policy was not enforced".into())
        }
        Err(err) => Err(err.into()),
    }
}
