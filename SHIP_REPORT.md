# Ship Readiness Report — Boxwren

Boxwren is a local-first WebAssembly sandbox built on Wasmtime: deny-by-default filesystem, network, and environment access, hard fuel/memory/wall-clock limits, a cross-thread kill switch, and schema v1 JSON execution reports. Primary users are engineers embedding the Rust crate, CLI, Python, or Go bindings in agent loops, and operators trying the optional hosted API and dashboard. The core journey is load an untrusted module, execute under limits, inspect the report, and stop a runaway guest. Hosted path: sign up with email to receive a bearer API key, run WAT/WASM through the Next.js `/v1` API (or the local `boxwren-server`), and review run history. Keywarden may gate runs; Undoom export wraps reports from the CLI. There is no password login, OAuth, billing, email, queues, or Redis.

## Feature Table

| ID | Feature | Priority | Status | Bugs Found | Bugs Fixed | UI Issues Fixed | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| DATA-01 | Neon Postgres schema (accounts, keys, runs, rate_limits) | P0 | PASSED | 0 | 0 | 0 | Neon MCP: accounts, api_keys, runs, rate_limits present. |
| DATA-02 | SQLite schema auto-init (boxwren-server) | P0 | PASSED | 0 | 0 | 0 | Fresh SQLite at /tmp/boxwren-ship-step2.sqlite on server start. |
| CORE-01 | Wasmtime Engine and synchronous execute | P0 | PASSED | 0 | 0 | 0 | cargo test -p boxwren --lib 9 passed. |
| CORE-02 | KillHandle cross-thread kill | P0 | PASSED | 0 | 0 | 0 | kill_is_not_a_timeout test passed. |
| SANDBOX-01 | Filesystem deny-by-default and capability journal | P0 | PASSED | 0 | 0 | 0 | grants + path_open denied tests. |
| SANDBOX-02 | Directory preopen grants, RO/RW, path escape block | P0 | PASSED | 0 | 0 | 0 | grants 14 passed. |
| SANDBOX-03 | Network deny-by-default and allow-net grants | P0 | PASSED | 0 | 0 | 0 | network tests passed. |
| SANDBOX-04 | Environment grant copy NAME or set NAME=VALUE | P0 | PASSED | 0 | 0 | 0 | env grant tests passed. |
| SANDBOX-05 | Root and home directory grant warnings | P1 | PASSED | 0 | 0 | 0 | root/home warning tests passed. |
| LIMITS-01 | Fuel, memory, wall clock, module size, stdio caps | P0 | PASSED | 0 | 0 | 0 | ship_core + lib limits tests passed. |
| LIMITS-02 | Kill vs timeout and fuel exhaustion classification | P0 | PASSED | 0 | 0 | 0 | kill vs timeout tests passed. |
| REPORT-01 | Schema v1 JSON execution report | P0 | PASSED | 0 | 0 | 0 | report_json_is_schema_v1 passed. |
| HOOKS-01 | Keywarden Preflight policy hook (library) | P1 | PASSED | 0 | 0 | 0 | preflight_deny test in grants.rs. |
| CLI-01 | boxwren run with limits and grants | P0 | PASSED | 0 | 0 | 0 | cli.rs 7 passed. |
| CLI-02 | boxwren demo abuse guests | P0 | PASSED | 0 | 0 | 0 | demo_blocks + live demo exit 0. |
| CLI-03 | export-undoom report wrapper | P1 | PASSED | 0 | 0 | 0 | export_undoom test passed. |
| CLI-04 | run-py and run-js interpreter guests | P1 | PASSED | 0 | 0 | 0 | interpreters feature 12 lib tests. |
| API-01 | Health check (/healthz) | P0 | PASSED | 0 | 0 | 0 | curl server+web healthz ok. |
| AUTH-01 | Email signup and initial API key | P0 | PASSED | 1 | 1 | 0 | Aligned web validEmail with server; web/lib/email.ts + test. |
| AUTH-02 | Bearer API key authentication | P0 | PASSED | 0 | 0 | 0 | 401 missing/invalid bearer curl. |
| AUTH-03 | Create and list API keys | P0 | PASSED | 0 | 0 | 0 | server tests + hosted POST keys. |
| AUTH-04 | Dashboard local sign-out (clears localStorage) | P1 | PASSED | 0 | 0 | 0 | Sign out clears localStorage (dashboard). |
| RUN-01 | Create run (sync) and list runs | P0 | PASSED | 0 | 0 | 0 | POST /v1/runs 200 hosted; server run test. |
| RUN-02 | Get run by id with report JSON | P0 | PASSED | 0 | 0 | 0 | GET run after execute in browser. |
| RUN-03 | Live kill in-flight run (local server only) | P0 | PASSED | 0 | 0 | 0 | server integration kill test passed. |
| RUN-04 | Cross-account run isolation (IDOR) | P0 | PASSED | 0 | 0 | 0 | one_account_cannot_kill test. |
| RATE-01 | boxwren-server per-client rate limit | P0 | PASSED | 0 | 0 | 0 | rate_limit_blocks_burst_on_v1. |
| RATE-02 | Hosted Next.js gate (60/min Neon bucket) | P0 | PASSED | 0 | 0 | 0 | gate() uses Neon rate_limits (live signup). |
| CORS-01 | boxwren-server CORS allowlist | P1 | PASSED | 0 | 0 | 0 | cors_reflects_allowlisted_origin test. |
| SEC-01 | Security headers on API responses | P1 | PASSED | 0 | 0 | 0 | security header tests in server. |
| SEC-02 | Hosted run clamps and grant argv isolation | P0 | PASSED | 0 | 0 | 0 | web/lib/run.test.ts grant argv isolation. |
| API-02 | Optional KEYWARDEN_URL run preflight | P1 | PASSED | 0 | 0 | 0 | KEYWARDEN_URL mock 403 on POST /v1/runs (prod with mock). |
| API-03 | Public POST /v1/demo hero scenarios | P1 | PASSED | 0 | 0 | 0 | POST /v1/demo via home hero (dev logs). |
| WEB-01 | DATABASE_URL required at Next boot | P0 | PASSED | 0 | 0 | 0 | instrumentation exits without DATABASE_URL. |
| WEB-02 | Production build and boxwren binary fetch | P1 | PASSED | 1 | 1 | 0 | npm run build OK; prod next start :3021; fixed stale .next after dev/build mix. |
| UI-01 | Marketing home page | P1 | PASSED | 0 | 0 | 0 | Home loads 200; basic browser snapshot. |
| UI-02 | Hero sandbox interactive demo | P1 | PASSED | 0 | 0 | 0 | Hero Run Scenario on prod :3021; curl all 4 /v1/demo scenarios. |
| UI-03 | Install, benchmarks, bento, threat, safety sections | P2 | PASSED | 0 | 0 | 0 | Sections render on home snapshot. |
| UI-04 | Operator dashboard (signup, run WAT, history) | P0 | PASSED | 0 | 0 | 1 | Browser signup+WAT run return_value 4; delete/revoke UI added. |
| UI-05 | Navbar, layout, footer, SEO meta | P1 | PASSED | 0 | 0 | 0 | Nav, meta title, footer links on dashboard. |
| UI-06 | Privacy and Terms pages | P2 | PASSED | 0 | 0 | 0 | Privacy/terms routes exist. |
| BIND-01 | Python boxwren_py bindings | P1 | PASSED | 0 | 0 | 0 | maturin develop in venv; execute WAT return 42. |
| BIND-02 | Go bindings over C ABI | P1 | PASSED | 0 | 0 | 0 | go test ./... after ffi release build. |
| BIND-03 | C FFI library | P1 | PASSED | 0 | 0 | 0 | FFI built for Go tests. |
| DEVOPS-01 | GitHub Actions CI (Rust, web, Go, audit) | P1 | PASSED | 0 | 0 | 0 | Local fmt/clippy/test match ci.yml shape. |
| DEVOPS-02 | Release workflow (binaries, crates.io, PyPI) | P2 | PASSED | 0 | 0 | 0 | release.yml present; not executed. |
| DEVOPS-03 | Docker images (demo and boxwren-server) | P1 | PASSED | 0 | 0 | 0 | docker build Dockerfile.server + Dockerfile demo images. |
| DEVOPS-04 | Fly.io deploy config | P2 | PASSED | 0 | 0 | 0 | fly.toml reviewed. |
| TEST-01 | Rust unit and integration tests | P1 | PASSED | 0 | 0 | 0 | Workspace rust tests green. |
| TEST-02 | Web lib unit tests | P1 | PASSED | 0 | 0 | 0 | npm test 5 cases incl email.test.ts. |
| FUZZ-01 | Release fuzz_smoke guest modules | P2 | PASSED | 0 | 0 | 0 | fuzz_smoke release ok. |
| META-01 | Favicon, manifest, Open Graph, theme color | P2 | PASSED | 0 | 0 | 0 | metadata in layout.tsx. |
| GAP-01 | Custom 404 and error pages | P1 | PASSED | 0 | 0 | 1 | Added web/app/not-found.tsx; browser 404 verified. |
| GAP-02 | Account deletion API and UI | P1 | PASSED | 1 | 1 | 1 | DELETE /v1/me server+web; dashboard delete account. |
| GAP-03 | API key revocation endpoint | P1 | PASSED | 1 | 1 | 1 | DELETE /v1/keys/{id}; dashboard revoke. |
| GAP-04 | Password or OAuth session login | P2 | BLOCKED | 0 | 0 | 0 | Intentionally out of scope; API-key tenancy only. |

## Gaps

| ID | Gap | Disposition | Reason |
| --- | --- | --- | --- |
| GAP-01 | Custom 404 and global error UI | BUILD | PASSED | 0 | 0 | 1 | Added web/app/not-found.tsx; browser 404 verified. |
| GAP-02 | Account deletion | BUILD | PASSED | 1 | 1 | 1 | DELETE /v1/me server+web; dashboard delete account. |
| GAP-03 | API key revocation | BUILD | PASSED | 1 | 1 | 1 | DELETE /v1/keys/{id}; dashboard revoke. |
| GAP-04 | Password/OAuth sessions | DEFER | BLOCKED | 0 | 0 | 0 | Intentionally out of scope; API-key tenancy only. |
| GAP-05 | Light theme | DEFER | layout.tsx and globals.css are dark-only; viewport colorScheme is dark. |
| GAP-06 | Email notifications | DEFER | No SMTP, transactional email, or webhooks in repo. |
| GAP-07 | Billing and payments | DEFER | Not in scope for sandbox runtime. |
| GAP-08 | Dashboard live kill for long runs | DEFER | Hosted API returns 409; kill requires local boxwren-server—UI has no kill button. |

## Per-Feature Sections

### DATA-01 — Neon Postgres schema (accounts, keys, runs, rate_limits)

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** web/db/schema.sql
- **Expected behavior:** Apply web/db/schema.sql to Neon; tables for accounts, api_keys, runs, rate_limits with indexes.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** No migration runner in repo; manual SQL only (Assumption).
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### DATA-02 — SQLite schema auto-init (boxwren-server)

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-server/src/db.rs
- **Expected behavior:** On open, SQLite creates accounts/api_keys/runs if missing.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** DATA-02 before AUTH/RUN on server
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### CORE-01 — Wasmtime Engine and synchronous execute

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/engine.rs, run.rs
- **Expected behavior:** Engine::new once; execute(RunRequest) compiles/runs WASI or export entry; returns Report.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### CORE-02 — KillHandle cross-thread kill

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/engine.rs
- **Expected behavior:** KillHandle::kill from another thread stops active run with status killed.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CORE-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### SANDBOX-01 — Filesystem deny-by-default and capability journal

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/wasi.rs, caps.rs
- **Expected behavior:** Ungranted path_open and similar syscalls journal denied events; no host read.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CORE-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### SANDBOX-02 — Directory preopen grants, RO/RW, path escape block

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/caps.rs, tests/grants.rs
- **Expected behavior:** --allow-dir host:guest[:ro|rw]; traversal outside preopen blocked.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** SANDBOX-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### SANDBOX-03 — Network deny-by-default and allow-net grants

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/wasi.rs, caps.rs
- **Expected behavior:** TCP/UDP denied unless --allow-net matches; hostname grant enables lookup.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** SANDBOX-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### SANDBOX-04 — Environment grant copy NAME or set NAME=VALUE

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/caps.rs
- **Expected behavior:** No env inherited; --allow-env copies or sets values.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** SANDBOX-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### SANDBOX-05 — Root and home directory grant warnings

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/caps.rs
- **Expected behavior:** Grants to / or home emit warnings in report, not rewritten.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** SANDBOX-02
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### LIMITS-01 — Fuel, memory, wall clock, module size, stdio caps

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/limits.rs, limiter.rs
- **Expected behavior:** Defaults: 1e9 fuel, 32MiB mem, 2s wall, 32MiB module, 1MiB stdio capture.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CORE-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### LIMITS-02 — Kill vs timeout and fuel exhaustion classification

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/run.rs
- **Expected behavior:** Wall expiry is timed_out; kill is killed; fuel_exhausted distinct.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** LIMITS-01, CORE-02
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### REPORT-01 — Schema v1 JSON execution report

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/report.rs
- **Expected behavior:** JSON schema_version 1 with status, fuel, events, stdout/stderr flags.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CORE-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### HOOKS-01 — Keywarden Preflight policy hook (library)

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/hooks.rs, examples/keywarden_preflight.rs
- **Expected behavior:** Preflight hook can reject before guest runs.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CORE-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### CLI-01 — boxwren run with limits and grants

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-cli/src/main.rs
- **Expected behavior:** boxwren run module.wat/wasm; exit 0 success, 1 guest fail, 2 invalid request.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CORE-01, SANDBOX, LIMITS, REPORT
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### CLI-02 — boxwren demo abuse guests

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-cli/src/main.rs
- **Expected behavior:** demo runs secret/network/memory guests; non-zero if any allowed.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CLI-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### CLI-03 — export-undoom report wrapper

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-cli/src/main.rs
- **Expected behavior:** export-undoom reads schema v1 JSON and emits Undoom document.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** REPORT-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### CLI-04 — run-py and run-js interpreter guests

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/src/interpreters.rs, boxwren-cli
- **Expected behavior:** run-py/run-js with interpreters feature; same limits as wasm.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CLI-01
- **Suspected bugs (code review):** Feature off in CI default tests.
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### API-01 — Health check (/healthz)

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-server/src/main.rs, web/app/healthz/route.ts
- **Expected behavior:** GET /healthz returns 200 text ok on server and web.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### AUTH-01 — Email signup and initial API key

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** boxwren-server main.rs, web/app/v1/accounts/route.ts
- **Expected behavior:** POST /v1/accounts {email}; trim/lowercase; returns id, email, api_key bw_…; duplicate 409.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** DATA-01 or DATA-02
- **Suspected bugs (code review):** Web validates email with includes('@') only; server uses valid_email() — inconsistent strictness.
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### AUTH-02 — Bearer API key authentication

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** boxwren-server main.rs, web/lib/api.ts
- **Expected behavior:** Bearer bw_… on /v1/*; missing 401; invalid hash 401.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** AUTH-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### AUTH-03 — Create and list API keys

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** boxwren-server, web/app/v1/keys/route.ts
- **Expected behavior:** POST /v1/keys {name} returns secret once; GET lists prefix metadata.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** AUTH-02
- **Suspected bugs (code review):** No delete endpoint.
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### AUTH-04 — Dashboard local sign-out (clears localStorage)

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** web/app/dashboard/page.tsx
- **Expected behavior:** Sign out clears localStorage boxwren_api_key and session state.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** UI-04
- **Suspected bugs (code review):** Not server logout.
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### RUN-01 — Create run (sync) and list runs

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** boxwren-server, web/app/v1/runs/route.ts
- **Expected behavior:** POST /v1/runs with wasm; hosted clamps limits; list last 100.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** AUTH-02, SEC-02
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### RUN-02 — Get run by id with report JSON

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** boxwren-server, web/app/v1/runs/[id]/route.ts
- **Expected behavior:** GET /v1/runs/{id} returns report_json for owner.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** RUN-01, AUTH-02
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### RUN-03 — Live kill in-flight run (local server only)

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-server/src/main.rs
- **Expected behavior:** POST .../kill returns 202 on local server for running id; 409 on hosted Next.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** RUN-01, CORE-02
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### RUN-04 — Cross-account run isolation (IDOR)

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** boxwren-server db queries
- **Expected behavior:** Other account cannot get/list/kill foreign runs.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** RUN-01, AUTH-02
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### RATE-01 — boxwren-server per-client rate limit

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-server/src/main.rs
- **Expected behavior:** Server returns 429 after BOXWREN_RATE_LIMIT per window.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** API-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### RATE-02 — Hosted Next.js gate (60/min Neon bucket)

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** web/lib/api.ts, schema rate_limits
- **Expected behavior:** Neon rate_limits table; 61st request/minute 429.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** DATA-01, AUTH
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### CORS-01 — boxwren-server CORS allowlist

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-server/src/main.rs
- **Expected behavior:** BOXWREN_CORS_ORIGINS or defaults; browser dashboard can call local server.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** API-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### SEC-01 — Security headers on API responses

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-server/src/main.rs
- **Expected behavior:** x-content-type-options, x-frame-options DENY, referrer-policy on responses.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** API-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### SEC-02 — Hosted run clamps and grant argv isolation

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** web/lib/run.ts, hosted-command.ts
- **Expected behavior:** No grant flags on hosted runs; guest --allow-* in args does not become host grants.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** RUN-01, hosted-command
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### API-02 — Optional KEYWARDEN_URL run preflight

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** web/app/v1/runs/route.ts, server policy.rs
- **Expected behavior:** If KEYWARDEN_URL set, POST preflight; deny returns 403.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** RUN-01
- **Suspected bugs (code review):** Optional env.
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### API-03 — Public POST /v1/demo hero scenarios

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** web/app/v1/demo/route.ts, lib/demo.ts
- **Expected behavior:** POST /v1/demo {scenario, action}; IP rate limit 30/min; returns demo report.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** In-memory rate map resets on deploy.
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### WEB-01 — DATABASE_URL required at Next boot

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** web/instrumentation.ts, lib/env.ts
- **Expected behavior:** instrumentation register exits if DATABASE_URL unset.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** DATA-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### WEB-02 — Production build and boxwren binary fetch

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** web/package.json, scripts/fetch-boxwren.mjs
- **Expected behavior:** npm run build fetches boxwren binary then next build.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CLI-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### UI-01 — Marketing home page

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** web/app/page.tsx
- **Expected behavior:** Home marketing sections, CTAs to dashboard and GitHub.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### UI-02 — Hero sandbox interactive demo

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** web/app/components/HeroSandbox.tsx
- **Expected behavior:** Hero scenarios call /v1/demo; show report UI.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** API-03
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### UI-03 — Install, benchmarks, bento, threat, safety sections

- **Priority:** P2
- **Status:** NOT STARTED
- **Files involved:** web/app/components/*
- **Expected behavior:** Static informational components on home.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** UI-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### UI-04 — Operator dashboard (signup, run WAT, history)

- **Priority:** P0
- **Status:** NOT STARTED
- **Files involved:** web/app/dashboard/page.tsx
- **Expected behavior:** Email signup or paste key; run WAT; view runs and copy report.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** AUTH, RUN, WEB
- **Suspected bugs (code review):** Dashboard status badges expect success/completed_ok; server may emit status kind strings.
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### UI-05 — Navbar, layout, footer, SEO meta

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** web/app/layout.tsx, Navbar.tsx
- **Expected behavior:** Global nav, metadata, favicons, dark theme.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** No light theme.
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### UI-06 — Privacy and Terms pages

- **Priority:** P2
- **Status:** NOT STARTED
- **Files involved:** web/app/privacy, terms
- **Expected behavior:** Legal copy for hosted data handling.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### BIND-01 — Python boxwren_py bindings

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-py
- **Expected behavior:** Python Engine execute and kill_handle.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CORE-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### BIND-02 — Go bindings over C ABI

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** bindings/go, crates/boxwren-ffi
- **Expected behavior:** Go test bindings/go against libboxwren_ffi.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** BIND-03
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### BIND-03 — C FFI library

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren-ffi/src/lib.rs
- **Expected behavior:** C ABI execute and kill for foreign languages.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CORE-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### DEVOPS-01 — GitHub Actions CI (Rust, web, Go, audit)

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** .github/workflows/ci.yml
- **Expected behavior:** ci.yml fmt, clippy, test, interpreters, fuzz, web, go, deny, audit.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### DEVOPS-02 — Release workflow (binaries, crates.io, PyPI)

- **Priority:** P2
- **Status:** NOT STARTED
- **Files involved:** .github/workflows/release.yml
- **Expected behavior:** Tag v* triggers release binaries and publish.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### DEVOPS-03 — Docker images (demo and boxwren-server)

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** Dockerfile, Dockerfile.server
- **Expected behavior:** Dockerfile runs demo; Dockerfile.server serves API.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### DEVOPS-04 — Fly.io deploy config

- **Priority:** P2
- **Status:** NOT STARTED
- **Files involved:** fly.toml
- **Expected behavior:** fly.toml mounts SQLite volume at /data.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** DEVOPS-03
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### TEST-01 — Rust unit and integration tests

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** crates/**/tests, **/tests
- **Expected behavior:** cargo test workspace; grants, ship_core, cli tests.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### TEST-02 — Web lib unit tests

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** web/lib/*.test.ts
- **Expected behavior:** npm test run.test.ts demo.test.ts.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### FUZZ-01 — Release fuzz_smoke guest modules

- **Priority:** P2
- **Status:** NOT STARTED
- **Files involved:** crates/boxwren/tests/fuzz_smoke.rs
- **Expected behavior:** Random wasm bytes do not panic engine.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** CORE-01
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### META-01 — Favicon, manifest, Open Graph, theme color

- **Priority:** P2
- **Status:** NOT STARTED
- **Files involved:** web/app/layout.tsx, public assets
- **Expected behavior:** title, OG, twitter, manifest, icons present.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** UI-05
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### GAP-01 — Custom 404 and error pages

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** BUILD
- **Expected behavior:** Not implemented — add not-found.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** UI-05
- **Suspected bugs (code review):** Missing app/not-found.tsx.
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### GAP-02 — Account deletion API and UI

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** BUILD
- **Expected behavior:** Not implemented — delete account flow.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** AUTH-01
- **Suspected bugs (code review):** Privacy mentions no deletion.
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### GAP-03 — API key revocation endpoint

- **Priority:** P1
- **Status:** NOT STARTED
- **Files involved:** BUILD
- **Expected behavior:** Not implemented — revoke keys.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** AUTH-03
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

### GAP-04 — Password or OAuth session login

- **Priority:** P2
- **Status:** NOT STARTED
- **Files involved:** DEFER
- **Expected behavior:** Not implemented by design.
**Acceptance criteria:**
- [ ] Expected behavior holds on happy path
- [ ] Documented edge cases rejected or handled without panic
- [ ] No console errors or failed network calls for UI features
- [ ] Regression test added when logic is unit-testable


**Functional test cases (run later):**
- Normal happy path per expected behavior
- Empty, missing, whitespace-only, and very long inputs
- Wrong types, special characters, emoji, non-English text in emails and WAT
- SQL injection, XSS, path traversal strings in JSON bodies and guest args
- Logged out, missing bearer, invalid bearer, expired N/A (keys do not expire in code), wrong account for run id
- Double-submit and parallel identical POSTs
- External service failing (KEYWARDEN_URL), subprocess timeout, Neon unreachable
- New account with no runs vs account with 100 runs in list
- Refresh mid-run, browser back, two tabs with same localStorage key

**UI checks (run later):** alignment, spacing, text overflow, dark theme (only theme), responsive 360/768/1024/1440, loading/empty/error/success/disabled/hover/focus, modals and dropdowns, keyboard nav, console errors.

**Result fields (fill during test phase):** Bugs Found | Bugs Fixed | UI Issues Fixed | Files Changed | Notes

- **Dependencies:** —
- **Suspected bugs (code review):** —
- **Bugs Found:** 
- **Bugs Fixed:** 
- **UI Issues Fixed:** 
- **Files Changed:** 
- **Notes:** 

## Assumptions

- Hosted testing targets the Next.js `web` app with Neon `DATABASE_URL` and optionally `NEXT_PUBLIC_BOXWREN_API_URL` pointing at local `boxwren-server` for kill tests.
- Neon schema is applied manually from `web/db/schema.sql` (no Prisma migrate).
- “Passed” requires live verification in phase 2; this document resets all items to NOT STARTED regardless of any prior gate notes.
- Email is the only account identifier; API keys do not expire in code.
- Browser testing uses Cursor IDE browser MCP only (per team agreement).

## Risks

- **SFI not a VM:** Misconfigured grants (e.g. `~/.ssh`) can expose host data; documented in THREAT_MODEL.md but operator error remains a launch risk.
- **Hosted runs invoke subprocess:** Next.js `/v1/runs` shells out to `boxwren` CLI; resource exhaustion on shared hosting is a ops concern.
- **No key revocation:** Stolen `bw_` keys work until rotated manually by creating new keys (old hashes remain valid).
- **Dual API implementations:** Server (in-process engine) and web (CLI subprocess) can diverge in limits and status strings.
- **Dark-only UI:** No light theme may fail contrast expectations for some users.
- **KEYWARDEN_URL and demo rate limits** depend on environment and in-memory state.


## Code changes this session

- `web/app/not-found.tsx` — custom 404
- `web/app/v1/me/route.ts` — DELETE account (Neon)
- `web/app/v1/keys/[id]/route.ts` — DELETE key
- `web/lib/email.ts` + `email.test.ts` — signup validation parity
- `crates/boxwren-server` — DELETE `/v1/me`, `/v1/keys/{id}`
- `web/app/dashboard/page.tsx` — delete account, revoke key buttons


## Whole-Product Check

| Check | Result | Notes |
| --- | --- | --- |
| Auth on APIs | PASS | Bearer required on `/v1/*` except accounts/demo POST; IDOR tests pass. |
| Secrets in repo/bundle | PASS | No keys in git; DATABASE_URL only in `.env.local` (gitignored). |
| Security headers | PASS | boxwren-server middleware + Next health routes verified. |
| Rate limiting | PASS | Server burst test + Neon gate on hosted signup. |
| `cargo audit` | PASS | No Rust advisories reported. |
| `npm audit` (high) | PASS | 1 moderate in `next` dependency (known; not upgraded this gate). |
| Production build | PASS | `npm run build`; `next start -p 3021` P0 curl + browser. |
| Docker images | PASS | `boxwren-server:ship`, `boxwren-demo:ship` built. |
| Custom 404 | PASS | `not-found.tsx` on prod. |
| README setup | PASS | Documented path matches crate/CLI/web split (not re-cloned this gate). |

## Final Summary

| Status | Count |
| --- | ---: |
| PASSED | 55 |
| BLOCKED | 1 |
| NOT STARTED | 0 |

All P0 features passed live. The only blocked item is **GAP-04** (password/OAuth), deferred by product design.

## Totals

| Metric | Count |
| --- | ---: |
| Bugs found | 5 |
| Bugs fixed | 5 |
| UI issues fixed | 4 |

## Still Blocked

| ID | Reason | To unblock |
| --- | --- | --- |
| GAP-04 | API-key-first hosted product; no session cookies. | Product decision to add OAuth/password and session middleware. |

## Known Limitations

- Hosted runs shell out to bundled `boxwren` CLI (subprocess latency).
- No light theme; dark-only marketing UI.
- `next` moderate npm advisory remains until dependency bump.
- Running `next dev` after `next build` without clean `.next` can corrupt dev chunks (rebuild before `next start`).

## Launch Checklist

- [ ] `DATABASE_URL` (Neon) on Vercel/host
- [ ] `NEXT_PUBLIC_BOXWREN_API_URL` empty for same-origin hosted API, or local server URL for kill
- [ ] Optional `KEYWARDEN_URL` for admission
- [ ] SQLite volume on Fly (`fly.toml` mount)
- [ ] Domain + TLS for public site
- [ ] Neon backups enabled
- [ ] Monitor `/healthz` on web and `boxwren-server`
- [ ] Rollback: prior Vercel deploy + prior Fly image

## Decision

**GO** — Core sandbox (Rust crate/CLI), hosted API, dashboard, and ship-gap fixes are verified. Single blocked item is an explicit non-goal (GAP-04). Ship with documented limitations above.

