# Boxwren ship report

Boxwren is a local-first sandbox that runs untrusted WebAssembly inside Wasmtime. Developers building code-executing agents use it so a generated module cannot read the host, open the network, or inherit environment secrets unless a grant says so. The core journey is: embed the library or CLI, submit a module with deny-by-default limits, read the schema v1 report, and kill a runaway run from another thread. An optional HTTP API and a two-page dashboard store accounts, API keys, and reports. Keywarden may admit a run. Undoom can snapshot the report. There is no password login, OAuth, Redis, job queue, email, or billing.

Boot on this machine, before any feature was marked passed:

- `./target/debug/boxwren-server --listen 127.0.0.1:8787 --db /tmp/boxwren-ship-gate.sqlite` logged `listening` and `GET /healthz` returned `ok`.
- `web` `npx tsc --noEmit` passed. `next dev` on port 3010 (3000 was already taken) returned `ok` from `GET /healthz` and 200 from `/`. `NEXT_PUBLIC_BOXWREN_API_URL=http://127.0.0.1:8787`.
- SQLite schema is created by the server. Neon schema is [web/db/schema.sql](web/db/schema.sql). No migration CLI.

## Inventory

Every row starts Not tested. Passed is written only after that feature's matrix is re-run.

| ID | Name | Priority | Status |
| --- | --- | --- | --- |
| P0-BOOT | Server and web health | P0 | Passed |
| P0-SANDBOX | Deny-by-default filesystem, network, environment | P0 | Not tested |
| P0-LIMITS | Fuel, memory, wall clock, module size, stdio, kill, sibling | P0 | Not tested |
| P0-REPORT | Schema v1 report and CLI exit codes | P0 | Not tested |
| P0-CLI | run, grants, demo, Ctrl-C, export-undoom | P0 | Not tested |
| P0-ACCOUNTS | Accounts, bearer auth, API keys | P0 | Not tested |
| P0-RUNS | Sync run, list, get, live kill | P0 | Not tested |
| P0-IDOR | Cross-account run isolation | P0 | Not tested |
| P0-HOSTED | Next.js /v1 clamps, argv isolation, kill 409 | P0 | Not tested |
| P0-DASHBOARD | Dashboard signup, reload, WAT run | P0 | Not tested |
| P0-RATELIMIT | Server 429 and Neon 60 per minute | P0 | Not tested |
| P1-INTERPRETERS | run-py and run-js WASI guests | P1 | Not tested |
| P1-PYTHON | boxwren_py execute, kill, grant strings | P1 | Not tested |
| P1-GO | Go C ABI execute and kill | P1 | Not tested |
| P1-KEYWARDEN | Admission preflight | P1 | Not tested |
| P1-CORS | Browser origin allowlist | P1 | Not tested |
| P1-SQLITE | Run row survives process restart | P1 | Not tested |
| P1-DEMO | Home demo scenarios | P1 | Not tested |
| P1-HEADERS | Security headers | P1 | Not tested |
| P1-AUDIT | Dependency audit | P1 | Not tested |
| P1-DOCKER | boxwren-server image | P1 | Not tested |
| P1-CI | Local CI command shape | P1 | Not tested |
| P1-ENV | Missing DATABASE_URL fails clearly | P1 | Not tested |
| P1-A11Y | Keyboard, labels, four widths | P1 | Not tested |
| P1-LEGAL | Privacy and terms | P1 | Not tested |
| P2-FUZZ | Release fuzz smoke | P2 | Not tested |
| P2-LIGHTHOUSE | Lighthouse on / and /dashboard | P2 | Not tested |
| P2-LOAD | Short load test | P2 | Not tested |
| P2-META | Favicon and Open Graph | P2 | Not tested |

Not in the product, and not implemented by this gate: password login, logout, reset, OAuth, roles, Redis, queues, cron, webhooks, email, payments, i18n.

## Feature log

### P0-BOOT Server and web health

Done means both processes answer liveness without a stack trace, unknown paths 404, and a fresh SQLite file is created.

Matrix: happy `GET /healthz` on server and web; unknown `/healthz/nope`; `/` and `/dashboard` return 200; two serial health checks. N/A: SQL injection, IDOR, mobile, Redis. This feature has no user input and no session.

Evidence: server `200` body `ok`; `/healthz/nope` `404`; web health `ok`; home `200`; dashboard `200`; SQLite file 36864 bytes at `/tmp/boxwren-ship-gate.sqlite`; second health `ok`. No bug. Verdict PASS.

---

