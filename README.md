# Boxwren

Local-first sandbox for untrusted WebAssembly. Boxwren runs a WASI module inside Wasmtime with deny-by-default fuel, memory, wall-clock, filesystem, network, and environment limits, a per-run kill switch, and a stable execution report.

Boxwren contains execution. [Keywarden](https://github.com/rejisterjack/keywarden) decides whether a tool may be called. [Undoom](https://github.com/rejisterjack/undoom) records and replays failures. There is no Boxwren cloud, account system, or dashboard.

The guest is WebAssembly, not source. Agent frameworks compile or supply a module. A bundled Python or JavaScript interpreter is not part of this runtime.

## Install

The library is the crates.io package. The `boxwren` binary is published as a GitHub Release.

```bash
cargo add boxwren
```

From a checkout:

```bash
cargo build -p boxwren-cli --release
./target/release/boxwren demo
```

Guests need the `wasm32-wasip1` and `wasm32-wasip2` targets (`rustup target add`).

## Library

Reuse one engine. Each run gets its own store, fuel budget, memory cap, and deadline.

```rust
use std::time::Duration;

use boxwren::{Engine, Entry, Grants, Limits, RunRequest};

let engine = Engine::new()?;
let mut limits = Limits::default();
limits.wall_time = Duration::from_secs(2);
let report = engine.execute(RunRequest {
    wasm: std::fs::read("guest.wasm")?,
    entry: Entry::Command { args: vec!["guest".into()] },
    limits,
    grants: Grants::default(),
    args: Vec::new(),
})?;
println!("{}", report.to_json());
```

`KillHandle::kill`, from another thread, stops that run. A wall-clock deadline that expires on its own is `timed_out`, not `killed`.

Defaults: 1_000_000_000 fuel (an instruction count, not milliseconds), 32 MiB of memory, 2 seconds of wall time, 1 MiB of captured stdout and of stderr, 32 MiB maximum module size, no directories, no network, and no environment variables.

## CLI

```bash
boxwren run guest.wasm --report json
boxwren run guest.wasm --allow-dir ./data:/data:ro --allow-net 127.0.0.1:9 --allow-env API_TOKEN
boxwren run guest.wasm --fuel 1000000 --memory 16MiB --timeout 500ms
boxwren demo
```

`--allow-dir` is `host:guest`, `host:guest:ro`, or `host:guest:rw`. The split is the last colon. The default mode is read-only. `--allow-net` is an IP address, or `ip:port`. `--allow-env NAME` copies a host variable, and `NAME=VALUE` sets one explicitly. A missing directory or an unknown variable fails before the guest starts.

Ctrl-C fires the kill handle. The process prints the report and exits 0 when the status is `success`, 1 when the guest was stopped or trapped, and 2 when the request itself was invalid.

`boxwren demo` runs three embedded guests: a read of `/etc/passwd`, a connect to `127.0.0.1:1`, and a memory grow past the cap. It exits non-zero if any of those is allowed. The secret guest calls `path_open` directly so the report names the path. Preview 1 `std::fs` often returns before the host call when no preopen matches.

## Report

Schema version 1, JSON. Fields: `status`, `exit_code`, `return_value`, `duration_ms`, `fuel_used`, `peak_memory`, `stdout`, `stderr`, truncation flags, `events`, and `warnings`. An event is `filesystem`, `network`, or `env`, plus the target and `allowed` or `denied`.

Statuses: `success`, `fuel_exhausted`, `memory_limited`, `timed_out`, `killed`, `trapped`, `rejected`.

A grant of `/` or of the home directory is allowed and named in `warnings`. Rewriting that grant is a policy decision and belongs to Keywarden.

## Limits of the sandbox

Wasm plus these capabilities stops the guest from reading host memory, walking out of a preopen, opening the network, inheriting environment secrets, or running past the fuel, memory, and wall-clock limits when those limits and the kill switch are left on.

It does not stop a bug in Wasmtime or Boxwren, a Spectre-class side channel, a grant pointed at `~/.ssh`, or anything the host process can already do outside the sandbox. Compilation of a module is not interruptible; the module size cap is the mitigation. Software fault isolation is not a virtual machine. See [docs/THREAT_MODEL.md](docs/THREAT_MODEL.md) and [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Python and Go

A cold `boxwren` process is about 6 ms on the machine in [docs/BENCHMARKS.md](docs/BENCHMARKS.md). An agent loop that starts a process per snippet should call the library in-process instead.

Python (`pip install boxwren-py`, then `import boxwren_py`):

```python
import boxwren_py

engine = boxwren_py.Engine()
kill = engine.kill_handle()
report = engine.execute(open("guest.wasm", "rb").read(), entry="_start")
```

From a checkout: `pip install maturin && maturin develop` in `crates/boxwren-py`, or:

```bash
cargo build -p boxwren-py --release
cp target/release/libboxwren_py.dylib boxwren_py.so   # .so on Linux
PYTHONPATH=. python3 -c 'import boxwren_py; print(boxwren_py.Engine())'
```

Go (`go get github.com/rejisterjack/boxwren/bindings/go`): the package wraps the C ABI in `crates/boxwren-ffi`, not a second sandbox. `cargo build -p boxwren-ffi --release`, then `go test` in `bindings/go`. Grants use the same strings as the CLI. `Engine.KillHandle()` and `ExecuteWith(wasm, request, kill)` stop one run from any goroutine.

## License

MIT OR Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.
