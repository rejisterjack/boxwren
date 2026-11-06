# Boxwren — Sandboxed Execution Runtime for AI Agents

> Let agents run code without trusting it—local-first, deny-by-default, with hard limits and a kill switch.

**Home:** [boxwren.rejisterjack.com](https://boxwren.rejisterjack.com)

## One-line idea

A small, fast runtime that executes agent-generated code and tool payloads inside WebAssembly isolation, with explicit caps on CPU, memory, network, and filesystem—and a kill switch that always stops runaway work.

## The problem

Agents increasingly write and execute code on developer machines and servers. One bad generation or prompt injection can read secrets, exfiltrate data, or damage the filesystem. Full container platforms are heavy; hosted sandboxes add cost and latency. Teams need something embeddable in minutes: tight defaults, clear reports, honest security boundaries.

## What Boxwren does

- Runs untrusted code in a WebAssembly sandbox (Wasmtime)
- Per-run limits: CPU time, memory, network, filesystem—deny by default
- Capability-based grants for only what a run needs
- Kill switch for immediate termination
- Execution report: attempted actions, blocks, resource usage
- Library + CLI; Python (`boxwren-py`) and Go (C ABI) bindings for agent stacks

## Who it's for

- Developers building code-executing agents who refuse to run generated code bare on the host
- Teams wanting lightweight isolation without operating a container fleet
- Framework authors embedding a safe execution step

## How you use it

1. Add the Boxwren crate to your project or install the CLI
2. Configure limits and capabilities for each run (defaults deny)
3. Submit generated code or a tool invocation to the runtime
4. On violation, Boxwren stops the run and records the reason
5. Inspect the report; use the kill switch for runaway executions

## How it runs

- **Local-first:** embed the library or CLI; no account required; execution stays where the agent runs
- **Optional hosted:** `boxwren-server` exposes accounts, API keys, and runs over HTTP; `web/app/dashboard` is a thin operator UI. Same Wasmtime sandbox — not a VM. See [docs/HOSTED.md](docs/HOSTED.md)
- Published as a Rust crate, release binaries, and an optional self-hosted API

## Tech stack

- Rust (runtime core)
- Wasmtime (WebAssembly engine with resource limiting)
- CLI in Rust; language bindings for common agent ecosystems
- Optional Axum HTTP API (`boxwren-server`) and Next.js dashboard (`web/`)
- Docker optional for demo, CI, and hosted deploy
- Terminal or dashboard UI for execution reports

## Success criteria

- Demo: generated code attempts secret read, network call, and memory abuse—each blocked with a clear report
- Published benchmarks vs. container startup (document methodology honestly)
- README and architecture note stating threat model and non-goals
- Open source at [github.com/rejisterjack/boxwren](https://github.com/rejisterjack/boxwren)

## Limitations & non-goals

- Sandboxing is hard; vendors like E2B exist—Boxwren stays **lightweight and embeddable**; hosted mode is the same SFI, not Firecracker
- **No perfect security claim**—document what WASM + capabilities protect against and what they do not
- Not a replacement for Keywarden policy on which tools an agent may call

## Distribution

| Artifact | Channel |
|----------|---------|
| Library | [crates.io](https://crates.io) (`boxwren`) |
| Binaries | GitHub releases |
| Hosted API | Self-host `boxwren-server` ([docs/HOSTED.md](docs/HOSTED.md)) |
| Dashboard | `web/app/dashboard` against your API |

Hosted execution is optional. Local embed remains the primary path.

## Related projects

Boxwren is the **containment** layer in the agent safety stack:

| Project | Role |
|---------|------|
| [Keywarden](https://github.com/rejisterjack/keywarden) | Credentials and policy on tool/MCP calls |
| [Undoom](https://github.com/rejisterjack/undoom) | Capture failures and regression-test fixes |
| **Boxwren** | Isolate and limit untrusted code execution |

Govern access with Keywarden, prove fixes with Undoom, execute untrusted code inside Boxwren when you must run it at all.
