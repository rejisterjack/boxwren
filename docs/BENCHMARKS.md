# Benchmarks

Measured on 2026-10-29.

| | |
| --- | --- |
| Machine | Apple M5, 10 cores, 24 GiB RAM |
| OS | macOS 27.0.1, Darwin 27.0.0 arm64 |
| Compiler | rustc 1.98.1 |
| Build | `cargo build --release` |

These are three different operations. A warm in-process run reuses a Wasmtime engine. A cold CLI run starts a process. `docker run` starts a container. None of these numbers is a claim that Boxwren replaces Docker.

## Warm engine

`cargo run -p boxwren --release --example startup`

The example creates one engine, runs a two-plus-two module 20 times without timing, then times 200 more runs. The module bytes are cached on the engine after the first compile.

| Run | `Engine::new` | Per subsequent run |
| --- | --- | --- |
| First process | 584 µs | 1057 ns |
| Second process | 100 µs | 1096 ns |

The second process is faster to start because the operating system has the binary cached. The per-run number is the part an agent loop pays when it keeps the engine.

## Cold CLI

Release binary `target/release/boxwren`. Guest is a WAT module whose `_start` is `nop`. One untimed warmup, then 20 samples of `boxwren run guest.wat --report json` with stdout discarded. Each sample includes process creation, engine startup, compilation, execution, and writing the report.

| min | p50 | p90 | max |
| --- | --- | --- | --- |
| 5.0 ms | 6.1 ms | 8.0 ms | 11.7 ms |

About 6 ms per process is too much for a loop that starts a new process for every snippet. That is why the Python and Go bindings keep an engine inside the caller.

## Docker

Docker Desktop was already running. `alpine:3.20` was pulled, then `docker run --rm alpine:3.20 true` was run once untimed and 8 times timed. The container executes `true`. It does not compile or run WebAssembly.

| min | p50 | p90 | max |
| --- | --- | --- | --- |
| 173 ms | 185 ms | 195 ms | 241 ms |

## How to repeat

```bash
cargo build --release -p boxwren --example startup
cargo build --release -p boxwren-cli
./target/release/examples/startup
# time `boxwren run` and `docker run --rm alpine:3.20 true` the same way
```

Record the machine, the build, and the sample count next to any new numbers.
