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

### P0-SANDBOX Deny-by-default filesystem, network, environment

Done means a guest with empty grants cannot read `/etc/passwd`, open a socket, or see host environment, and a directory grant cannot escape its root. A path that looks like a grant flag in guest argv stays data.

Matrix: secret read, granted dir plus `../` escape, network connect, env absent, hostname grant versus IP grant, root grant warned and not rewritten. N/A: browser back button, Redis, XSS. The guest is Wasm, not HTML.

Evidence: `cargo test -p boxwren --test grants` 14 passed in 2.48s, including secret read, directory escape, read-only write rejection, network deny, env not inherited, IP grant without DNS, hostname grant connect, and root grant warned. CLI `boxwren run secret.wasm` printed `filesystem /etc/passwd denied` and stdout `blocked errno 8`, with no `root:`. `cargo fmt --all --check` and `cargo clippy -p boxwren --all-targets -- -D warnings` passed. No bug. Verdict PASS. Regression tests are the grant cases in `crates/boxwren/tests/grants.rs`.

---

### P0-LIMITS Fuel, memory, wall clock, module size, stdio, kill, sibling

Done means a guest stops on fuel, memory, wall clock, module size, and stdio cap, a kill is not recorded as a timeout, and a sibling run still returns.

Matrix: fuel starvation, memory limit, wall clock, oversized module, stdio truncated to 4 bytes, kill versus timeout, sibling survives, five memories. N/A: form double-submit, XSS, logged-out. These are library limits, not a page.

Evidence: `cargo test -p boxwren --lib` 9 passed, including fuel starvation, oversized memory, module size cap, kill is not a timeout, and wall clock is not a kill. `cargo test -p boxwren --test ship_core` 7 passed, including stdio truncated to 4 bytes with `uvwxyz` absent, five memories `MemoryLimited`, and a killed loop whose sibling still returned 4. No bug. Verdict PASS. Regression file: `crates/boxwren/tests/ship_core.rs`.

---

### P0-REPORT Schema v1 report and CLI exit codes

Done means a successful run prints schema version 1, a stopped guest exits 1, and a missing module or missing argument exits 2.

Matrix: add module exit 0 and `return_value` 4; fuel loop exit 1 with `fuel_exhausted`; missing path exit 2; empty argv exit 2; text `not wasm` is a `rejected` report and exit 1 because a report was produced. N/A: IDOR, mobile, SQL. The report is a local CLI document.

Evidence: `boxwren run add.wat --entry run --report json` exit 0, `schema_version` 1, status success, return 4. Loop with `--fuel 50` exit 1, status `fuel_exhausted`. Missing file exit 2, message `could not read ... No such file`. No module argument exit 2. `not wasm` exit 1, status rejected, schema 1. `ship_core` `report_json_is_schema_v1` passed in the limits run. No bug. Verdict PASS.

---

### P0-CLI run, grants, demo, Ctrl-C, export-undoom

Done means `run` honors a directory grant, `demo` blocks the three abuse guests, Ctrl-C kills a loop, and `export-undoom` wraps schema v1.

Matrix: happy run, grant read, bad grant flag, demo exit 0, SIGINT on a loop, export-undoom document. N/A: browser refresh, Neon, Redis. This is the local binary.

Evidence: `cargo test -p boxwren-cli --test cli` 7 passed in 1.77s: export return, Ctrl-C status killed exit 1, invalid grant before the module, JSON report, export-undoom wrap, guest `--allow-env` after `--` does not become a grant, demo blocks secret, network, and memory. No bug. Verdict PASS.

---

### P0-ACCOUNTS Accounts, bearer auth, API keys

Done means a trimmed email creates an account and a bearer key, a bad or duplicate email is refused, and a second key can be created and listed.

Matrix: empty object, `not-an-email`, whitespace, unicode local-part, malformed `{`, duplicate after trim and lowercase, `GET /v1/me` with bearer, missing and bad bearer, empty and 81-character key name, create and list, double submit of the same key name. N/A: password login, OAuth, browser. The product issues a bearer key and does not have a session cookie.

Evidence: bad and blank email 400 `invalid email`. `üser@example.com` 200. `{` 400 JSON parse error. `{}` 422 missing `email`. `Ship.Gate+Accounts@Example.com` stored as `ship.gate+accounts@example.com`, duplicate 409 `account already exists`. `GET /v1/me` 200 with the email. No header 401 `missing Authorization`. Bad key 401 `invalid API key`. Empty and 81-character names 400 `invalid key name`. Named key listed beside `default`. A second key named `ci` is a new id, so double submit creates two keys rather than corrupting the first. No bug. Verdict PASS.

---

### P0-IDOR Cross-account run isolation

Done means account B cannot read, list, or kill account A's run, including a run that is still executing.

Matrix: B `GET` A's finished run, B list, anonymous `GET`, B kill of a finished run, B kill of a live run, owner kill still works. N/A: password session, Redis. Isolation is the bearer key and the SQLite `account_id` column.

Evidence: B `GET` 404 `run not found` and B's list did not contain A's id. Anonymous `GET` 401. B kill of the finished run 404. B kill of a `running` background loop 404, and A's `GET` still said `running`. A's own kill returned 202 and the report was `killed`. B's `/v1/me` returned only B's email. No bug. Verdict PASS.

---

### P0-RUNS Sync run, list, get, live kill

Done means an authenticated caller can run a module, read it back, and kill a background run. Guest argv that looks like a grant does not open the host.

Matrix: logged out, empty body, malformed JSON, whitespace module, bad WAT, happy 2+2, negative fuel, unicode argv, `--allow-dir` in guest argv, `../..` directory grant, bad grant syntax, list, get, missing id, double submit, `wait: false` plus kill. N/A: browser back button, Redis. The API is bearer-authenticated JSON.

Evidence: logged out 401. Empty body 400 `provide wasm_base64 or wasm_text`. `not-json` 400. Whitespace and `not wasm` stored as schema v1 `rejected`. Happy run status success, `return_value` 4. `fuel: -1` 422. Unicode argv still returns 4. Guest argv `--allow-dir /:/:rw` and dir grant `../..:/:rw` both denied `/etc/passwd` with no `root:` in stdout. Bad grant 400 before the guest. `GET` of the run id returned the same success report. Missing id 404 `run not found`. A second submit created a new id. `wait: false` returned `running` in 1ms, `POST .../kill` returned 202, and the stored report was `killed` at 58ms. A second kill was 404 `run not in flight`. No bug. Verdict PASS.

---


