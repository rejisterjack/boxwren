# Architecture

Boxwren is one library and one CLI. The library owns the sandbox. The CLI parses grants, installs a Ctrl-C handler, and prints the same report the library returns.

## Process shape

One `wasmtime::Engine` per process. A background thread ticks `Engine::increment_epoch` about once a millisecond and stops when the engine is dropped. Each run gets its own `Store`, fuel budget, memory limiter, epoch deadline, and journal.

```
caller --> Sandbox --> Engine
                  \--> Store --> WASI host --> Journal
                         \--------------------> Journal --> Report
```

Modules are cached on the engine by a hash of their bytes. Components (preview 2) are not cached; core modules are.

## Limits

| Limit | Mechanism | Report |
| --- | --- | --- |
| Instructions | `Config::consume_fuel`, `Store::set_fuel` | `fuel_exhausted`, `fuel_used` |
| Memory | `StoreLimits` via `Store::limiter` | `memory_limited`, `peak_memory` |
| Instances, memories, tables | The same limiter (8, 4, 8) | `memory_limited` when instantiation is refused |
| Wall clock | Epoch deadline of one tick; the callback compares `Instant` | `timed_out` |
| Kill switch | The same callback, after a per-run flag is set | `killed` |
| Module size | Rejected before `Module::from_binary` / `Component::new` | `rejected` |
| Stdio | Piped writers that drop bytes past the cap | truncated flags |

Fuel is an instruction count. It is not milliseconds.

The epoch deadline lives in the store and is not written from another thread. Every store is armed with a deadline of one tick. The callback returns `Continue(1)` until the kill flag is set or the wall clock has elapsed, then it traps. `KillHandle::kill` sets the flag and increments the epoch once so that store notices. Other stores on the same engine see the tick, observe that their own flag is clear, and continue.

A denied `memory.grow` returns -1 to the guest and sets a flag. The report is `memory_limited` when that flag is set, including when the guest then exits.

## WASI

Nothing is inherited. `inherit_network` is never called. The host environment is not copied. There are no preopens unless a grant says so.

Preview 1 core modules (`wasm32-wasip1`) are linked with `wasmtime_wasi::p1`. `path_open` and the other `path_*` imports are wrapped so the report records the guest path, then a tiny Wasm shim calls the real import. The shim re-exports the guest memory. Calling the real host function directly from the wrapper hides that memory, and preview 1 fails closed with "missing required memory export". Success is errno 0. `..` is left to cap-std, which refuses to leave the preopen; the attempt is still journaled.

Preview 2 components (`wasm32-wasip2`) are linked with `wasmtime_wasi::p2` and invoked as `wasi:cli/run`. TCP and UDP are enabled only so the socket address check can see the destination. The check runs before the connect system call. An implicit bind of `0.0.0.0:0` or `[::]:0` is allowed and not journaled, because the guest stack does that before `TcpConnect`. Every other address is denied unless a `NetGrant` matches, and the attempt is journaled. Name lookup stays off. Enabling it later would leave DNS rebinding as a residual risk.

Environment grants are journaled when they are injected. A guest lookup of a variable that was not granted is not a host call; tests check that the secret string is absent from stdout.

## Report

Schema version 1, serde JSON. Undoom can snapshot a report later without a code dependency on this crate. The status tag is snake_case. Events are `{ kind, target, decision }`.

## What is trusted

The embedder is trusted. The guest is not. A broad grant such as `/` or `$HOME` is passed through and named in `warnings`. Boxwren does not decide whether the agent may run code.
