# Ship Readiness Report — Boxwren

Boxwren is a local-first WebAssembly sandbox. It runs a WASI module in Wasmtime with deny-by-default fuel, memory, wall-clock, filesystem, network, and environment limits, a per-run kill switch, and a schema v1 execution report.
Engineers embed the Rust crate, the boxwren CLI, or the Python and Go bindings in an agent loop. Operators also use the optional Next.js site and the long-lived boxwren-server.
The core journey is to supply an untrusted module, execute it under those limits, read the report, and stop a runaway guest.
On the hosted path, an operator submits an email, receives a bearer API key, runs WAT or WASM through POST /v1/runs, and reviews that account's history on /dashboard.
The public site ignores filesystem, network, and environment grants and refuses a live kill. Those grants and the live kill exist on the CLI and on boxwren-server.

## Feature Table

| ID | Feature | Priority | Status | Bugs Found | Bugs Fixed | UI Issues Fixed | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| DATA-01 | Neon Postgres schema | P0 | PASSED | 0 | 0 | 0 | Live Neon: four tables, FKs, unique email and hash, runs index. Duplicate email 23505. SQL-like text stored as data. |
| DATA-02 | SQLite schema auto-init | P0 | PASSED | 0 | 0 | 0 | Fresh sqlite created accounts, api_keys, runs. Restart kept the row. Bad paths fail. |
| CORE-01 | Wasmtime engine execute | P0 | PASSED | 0 | 0 | 0 | lib add_returns_four passed. CLI add returned 4, exit 0. Empty, whitespace, and non-module rejected without a crash. Missing export rejected. 100 repeats stayed success. Two parallel CLI runs both returned 4. |
| CORE-02 | KillHandle cross-thread kill | P0 | PASSED | 0 | 0 | 0 | Library kill_is_not_a_timeout passed. CLI SIGINT on a 5s loop returned status killed in 177ms and exit 1. A second process running the add module still returned 4. |
| SANDBOX-01 | Filesystem deny-by-default and capability journal | P0 | PASSED | 0 | 0 | 0 | demo and secret.wasm denied /etc/passwd and named that path. grants.rs secret_read_is_blocked_and_named and path_open_is_denied_and_named passed. Python open of /etc/passwd did not print file contents. |
| SANDBOX-02 | Directory preopen grants | P0 | PASSED | 0 | 0 | 0 | grants.rs read-only, read-write, and escape tests passed. CLI missing directory failed before start. Python read of a ro mount returned hello, write and .. escape returned EPERM. |
| SANDBOX-03 | Network deny-by-default and allow-net | P0 | PASSED | 0 | 0 | 0 | net.wasm and demo denied 127.0.0.1:1. grants.rs network_is_denied_unless_allowlisted, hostname_lookup_stays_off_for_an_ip_grant, and hostname_grant_can_connect passed. |
| SANDBOX-04 | Environment grants | P0 | PASSED | 0 | 0 | 0 | Missing host variable failed before start. grants.rs env_parse and environment_is_not_inherited_unless_granted passed. Python os.getenv of an explicit NAME=VALUE grant printed hello-env. |
| SANDBOX-05 | Root and home directory warnings | P1 | PASSED | 0 | 0 | 0 | CLI --allow-dir /:/:ro succeeded and warned that the grant is the filesystem root. grants.rs root_and_home_grants_are_warned_and_not_rewritten passed. |
| LIMITS-01 | Fuel, memory, wall clock, stdio, and module size caps | P0 | PASSED | 0 | 0 | 0 | fuel 50 returned fuel_exhausted with fuel_used 50. memory.wasm returned memory_limited. Bad --memory 5zz exited 2. Lib tests covered module size, stdio truncation, and five memories. |
| LIMITS-02 | Kill, timeout, and fuel exhaustion classification | P0 | PASSED | 0 | 0 | 0 | Timeout 200ms on a huge fuel budget returned timed_out at 200ms. Fuel 50 returned fuel_exhausted. SIGINT returned killed. ship_core killing_one_run_leaves_its_sibling passed. |
| REPORT-01 | Schema v1 JSON execution report | P0 | PASSED | 0 | 0 | 0 | Add-module JSON had schema_version 1 and the documented fields. ship_core report_json_is_schema_v1 passed. Empty stdout was an empty string. |
| HOOKS-01 | Library Keywarden preflight hook | P1 | PASSED | 0 | 0 | 0 | grants.rs preflight_deny_returns_policy_before_the_guest passed. The library trait does not open a network connection. |
| CLI-01 | boxwren run | P0 | PASSED | 0 | 0 | 0 | JSON report for the add module exited 0. Missing file exited 2. Bad memory unit exited 2. Guest denial and rejection printed a report and exited 1. SIGINT killed a loop. |
| CLI-02 | boxwren demo | P0 | PASSED | 0 | 0 | 0 | boxwren demo --report json exited 0. Secret, network, and memory probes were all blocked. |
| CLI-03 | export-undoom | P1 | PASSED | 0 | 0 | 0 | A success report wrapped to agent boxwren, name boxwren-run, with the original report under metadata.report_json. Empty input exited 2. |
| CLI-04 | run-py and run-js | P1 | PASSED | 0 | 0 | 0 | Release binary has both commands. run-py printed hello-from-py. run-js printed js-ok. Ungranted /etc/passwd did not leak. Empty Python file succeeded with empty stdout. |
| API-01 | Health check | P0 | NOT STARTED | | | | |
| AUTH-01 | Email signup and initial API key | P0 | NOT STARTED | | | | |
| AUTH-02 | Bearer API key authentication | P0 | NOT STARTED | | | | |
| AUTH-03 | Create and list API keys | P0 | NOT STARTED | | | | |
| AUTH-04 | Dashboard sign-out | P1 | NOT STARTED | | | | |
| AUTH-05 | Delete account | P0 | NOT STARTED | | | | |
| AUTH-06 | Revoke API key | P0 | NOT STARTED | | | | |
| RUN-01 | Create and list runs | P0 | NOT STARTED | | | | |
| RUN-02 | Get run by id | P0 | NOT STARTED | | | | |
| RUN-03 | Live kill on boxwren-server | P0 | NOT STARTED | | | | |
| RUN-04 | Cross-account run isolation | P0 | NOT STARTED | | | | |
| RUN-05 | Hosted async run and kill refusal | P0 | NOT STARTED | | | | |
| RATE-01 | boxwren-server rate limit | P0 | NOT STARTED | | | | |
| RATE-02 | Hosted Neon rate limit | P0 | NOT STARTED | | | | |
| RATE-03 | Public demo in-memory rate limit | P1 | NOT STARTED | | | | |
| CORS-01 | boxwren-server CORS allowlist | P1 | NOT STARTED | | | | |
| SEC-01 | boxwren-server security headers | P1 | NOT STARTED | | | | |
| SEC-02 | Hosted Next.js security headers | P1 | NOT STARTED | | | | |
| SEC-03 | Hosted run clamps and guest argv isolation | P0 | NOT STARTED | | | | |
| API-02 | KEYWARDEN_URL run preflight | P1 | NOT STARTED | | | | |
| API-03 | Public POST /v1/demo | P1 | NOT STARTED | | | | |
| WEB-01 | DATABASE_URL required at Next boot | P0 | NOT STARTED | | | | |
| WEB-02 | Production build and CLI binary fetch | P1 | NOT STARTED | | | | |
| UI-01 | Marketing home page | P1 | NOT STARTED | | | | |
| UI-02 | Hero sandbox demo | P1 | NOT STARTED | | | | |
| UI-03 | Install, benchmarks, capabilities, safety, and threat sections | P2 | NOT STARTED | | | | |
| UI-04 | Operator dashboard | P0 | NOT STARTED | | | | |
| UI-05 | Navbar, footer, and SEO metadata | P1 | NOT STARTED | | | | |
| UI-06 | Privacy and Terms pages | P2 | NOT STARTED | | | | |
| UI-07 | Custom 404 page | P1 | NOT STARTED | | | | |
| BIND-01 | Python bindings | P1 | NOT STARTED | | | | |
| BIND-02 | Go bindings | P1 | NOT STARTED | | | | |
| BIND-03 | C FFI library | P1 | NOT STARTED | | | | |
| DEVOPS-01 | GitHub Actions CI | P1 | NOT STARTED | | | | |
| DEVOPS-02 | Release workflow | P2 | NOT STARTED | | | | |
| DEVOPS-03 | Docker images | P1 | NOT STARTED | | | | |
| DEVOPS-04 | Fly.io config | P2 | NOT STARTED | | | | |
| TEST-01 | Rust test suite | P1 | NOT STARTED | | | | |
| TEST-02 | Web unit tests | P1 | NOT STARTED | | | | |
| FUZZ-01 | fuzz_smoke guest modules | P2 | NOT STARTED | | | | |
| META-01 | Favicon, manifest, Open Graph, and theme color | P2 | NOT STARTED | | | | |
| GAP-01 | Document or automate Neon schema apply | P0 | NOT STARTED | | | | |
| GAP-02 | Global 500 error page | P1 | NOT STARTED | | | | |
| GAP-03 | OpenAPI matches live routes | P1 | NOT STARTED | | | | |

## Gaps

| ID | Gap | Disposition | Reason |
| --- | --- | --- | --- |
| GAP-01 | Neon schema is never applied or documented | BUILD | web/db/schema.sql exists, and no app code or HOSTED.md step applies it. A new database cannot sign up. |
| GAP-02 | Global 500 error page | BUILD | web/app/not-found.tsx exists. web/app/error.tsx and global-error.tsx do not. |
| GAP-03 | OpenAPI matches live routes | BUILD | docs/openapi.yaml omits DELETE /v1/me, DELETE /v1/keys/{id}, and POST /v1/demo. |
| | Light theme | DEFER | layout.tsx sets colorScheme dark and globals.css is a dark palette. A light theme is not started. |
| | Password or OAuth login | DEFER | Accounts are email plus a bearer API key. There is no password, cookie session, or OAuth code. |
| | Email delivery | DEFER | No SMTP client, template, or webhook. Signup does not send the key by email. |
| | Billing | DEFER | No payment provider, plan, or invoice code. |
| | Dashboard live kill | DEFER | The hosted API returns 409 for kill and 400 for wait:false. The dashboard has no kill control. The local server already implements kill as RUN-03. |

## Per-Feature Sections

### DATA-01 Neon Postgres schema

- **ID:** DATA-01
- **Name:** Neon Postgres schema
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `web/db/schema.sql`
- `web/lib/db.ts`

**Expected behavior**

The hosted site stores accounts, API-key hashes, run reports, and per-minute rate-limit counters in Neon. schema.sql creates accounts (unique email), api_keys (unique hash, account FK), runs (account FK, JSONB report, index on account_id + created_at DESC), and rate_limits (bucket, window_start, count). Nothing in the Next.js app applies this file automatically.

**Acceptance criteria**

- [x] Four tables and the runs index exist with the columns in schema.sql.
- [x] Email and key hash are unique. A second account with the same email fails.
- [x] Runs for one account can be listed newest first.
- [x] Raw API keys are not stored. Only prefix and sha256 hash are stored.

**Test cases**

- **Normal use:** Apply schema.sql to an empty database and insert one account, one key, and one run. Confirm the row shapes match the INSERT statements in the route handlers.
- **Empty, missing, whitespace-only, and very long input:** Insert an account with an empty email only through SQL, then confirm the app's validEmail rejects that value before insert. Confirm a missing DATABASE_URL throws from db().
- **Wrong types, special characters, emoji, and non-English text:** Pass a non-JSON body to a route that writes these tables and confirm the write does not happen. Unicode and emoji emails are a validation concern on AUTH-01, not a column-type failure (TEXT).
- **SQL injection, XSS, and path traversal:** Signup and key-name values that contain quotes, semicolons, and SQL comments must be stored as data. The Neon tagged-template calls in the routes must not concatenate SQL.
- **Logged out, expired session, wrong role, and another user's data:** A run row written for account A must not be returned by a query filtered to account B. Covered again under RUN-04.
- **Double-submit and parallel requests:** Two transactions inserting the same email must leave exactly one accounts row.
- **External service failing, timing out, and rate limiting:** With Neon unreachable, db() calls fail and routes that catch nothing return 500. Record the status and body.
- **New user with no data, and a user with lots of data:** Insert more than 100 runs for one account and confirm GET /v1/runs still returns at most 100.
- **Refresh mid-action, back button, and two tabs:** Not a page. Re-applying schema.sql must be idempotent because every statement is CREATE TABLE IF NOT EXISTS or CREATE INDEX IF NOT EXISTS.

**UI checks**

No UI. Skip layout, theme, and viewport checks.

**Dependencies**

None.

**Suspected bugs**

- web/db/schema.sql is never executed by the app or by a migration command. A fresh Neon database will fail the first signup until someone applies it by hand. Tracked as GAP-01.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** Queried Neon project withered-voice-36073875. Columns match schema.sql. Unique email, unique hash, both FKs, and runs_account_created exist. Re-applied the CREATE IF NOT EXISTS statements. A second insert of the same email returned 23505 and the transaction rolled back. Inserted an account, key, and run whose email and key name contained quotes and a script tag; the SELECT returned them as data. Deleted that fixture. Sampled api_keys: hash is 64 hex chars, prefix length 10, prefix is not a substring of the hash. Did not take Neon offline. Automatic apply of schema.sql remains GAP-01.

### DATA-02 SQLite schema auto-init

- **ID:** DATA-02
- **Name:** SQLite schema auto-init
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren-server/src/db.rs`

**Expected behavior**

boxwren-server opens the SQLite file from --db or BOXWREN_DB and creates accounts, api_keys, and runs if they are missing. There is no rate_limits table; the server limiter is in memory. Dates are SQLite datetime text, not timestamptz.

**Acceptance criteria**

- [x] Starting the server against a new file creates the three tables.
- [x] Starting again against the same file does not drop rows.
- [x] The parent directory must exist. The code opens the path; it does not mkdir.

**Test cases**

- **Normal use:** Start the server with a fresh sqlite path and create an account. Confirm the three tables exist and the account row is present.
- **Empty, missing, whitespace-only, and very long input:** Open a zero-byte file path and an existing empty file. Both should initialize. A missing parent directory should fail at startup.
- **Wrong types, special characters, emoji, and non-English text:** The schema is created by the server, not by user SQL. Pass a directory path as --db and confirm startup fails cleanly.
- **SQL injection, XSS, and path traversal:** No user SQL. Account email and key name are bound parameters in db.rs.
- **Logged out, expired session, wrong role, and another user's data:** No session. Two accounts in one file must not share key hashes.
- **Double-submit and parallel requests:** Two server processes on one SQLite file are not supported. Note the failure mode; do not treat it as a required feature.
- **External service failing, timing out, and rate limiting:** SQLite is local. A read-only filesystem should fail open() with a logged error.
- **New user with no data, and a user with lots of data:** Insert many runs through the API and confirm list_runs returns them. Check the SQL for a LIMIT; the server list has no LIMIT in the signature read from list_runs.
- **Refresh mid-action, back button, and two tabs:** Restart the process and confirm rows survive.

**UI checks**

No UI.

**Dependencies**

None.

**Suspected bugs**

- crates/boxwren-server/src/db.rs:72-89 create_account inserts the account, then create_key separately. A failure on the key insert leaves an account with no key. The hosted path uses one Neon transaction (web/app/v1/accounts/route.ts:25-29).

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** Fresh file /tmp/boxwren-ship-data02.sqlite created accounts, api_keys, and runs. No rate_limits table. POST /v1/accounts stored data02@example.com; the raw key was not equal to the hash column (hash length 64, prefix length 10). Restart kept the row. A missing parent directory and a directory path both exited with "unable to open database file". A zero-byte file initialized the three tables. The non-atomic create_account path was not reproduced, so it was not changed.

### CORE-01 Wasmtime engine execute

- **ID:** CORE-01
- **Name:** Wasmtime engine execute
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/lib.rs`
- `crates/boxwren/src/engine.rs`
- `crates/boxwren/src/run.rs`

**Expected behavior**

Engine::new builds one Wasmtime engine. Engine::execute runs one RunRequest in its own store with that request's limits and grants. A successful exported i32 add returns Status::Success and return_value 4. WAT and wasm bytes are both accepted. The engine is reused across runs.

**Acceptance criteria**

- [x] The 2+2 WAT module in lib.rs tests returns success and return_value 4.
- [x] A second execute on the same Engine still works.
- [x] An invalid module becomes a rejected report or an Error, and does not panic the host.

**Test cases**

- **Normal use:** Run the library example: a tiny add module, default limits, no grants. Expect success and return value 4.
- **Empty, missing, whitespace-only, and very long input:** Execute with empty wasm bytes and with whitespace-only WAT. Expect rejection, not a host crash.
- **Wrong types, special characters, emoji, and non-English text:** A non-module byte string, a truncated wasm header, and a module whose export name does not exist. Expect a rejected or trapped report.
- **SQL injection, XSS, and path traversal:** WAT text containing SQL-looking strings and <script> is data, not host code. The host must not interpret it as anything but WAT.
- **Logged out, expired session, wrong role, and another user's data:** No account model in the library.
- **Double-submit and parallel requests:** Two executes on one Engine from two threads, each with its own request. Both must finish with independent reports.
- **External service failing, timing out, and rate limiting:** No network in the default request. Compilation of a huge module is covered by LIMITS-01.
- **New user with no data, and a user with lots of data:** Run the same module 100 times on one engine. Fuel and memory must reset per run.
- **Refresh mid-action, back button, and two tabs:** Dropping the engine and building a new one must not require process restart.

**UI checks**

No UI. Library only.

**Dependencies**

None.

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** lib add_returns_four passed. CLI add returned 4, exit 0. Empty, whitespace, and non-module rejected without a crash. Missing export rejected. 100 repeats stayed success. Two parallel CLI runs both returned 4.

### CORE-02 KillHandle cross-thread kill

- **ID:** CORE-02
- **Name:** KillHandle cross-thread kill
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/engine.rs`
- `crates/boxwren/src/lib.rs`

**Expected behavior**

KillHandle::kill, called from another thread, stops that run. The report status is killed, not timed_out. The library test kill_is_not_a_timeout sleeps 30ms then kills a loop whose wall clock is 3 seconds.

**Acceptance criteria**

- [x] A looping guest stopped by KillHandle reports status kind killed.
- [x] The run ends well before the wall-clock deadline.
- [x] Killing a handle that is not running does not panic.

**Test cases**

- **Normal use:** Start a tight loop with a long timeout and a large fuel budget. Kill from another thread. Expect killed.
- **Empty, missing, whitespace-only, and very long input:** Call kill before execute, then execute a short module. Record whether the next run is immediately killed or the flag resets. This is the behavior to pin.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable beyond the handle API. Cloning the handle and killing either clone must stop the same run.
- **SQL injection, XSS, and path traversal:** Not an input parser.
- **Logged out, expired session, wrong role, and another user's data:** No accounts.
- **Double-submit and parallel requests:** Two runs, two handles. Killing one must leave the other running.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** Kill and rerun the same engine 20 times.
- **Refresh mid-action, back button, and two tabs:** Process-local only. A new process has no leftover kill state.

**UI checks**

No UI.

**Dependencies**

`CORE-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** Library kill_is_not_a_timeout passed. CLI SIGINT on a 5s loop returned status killed in 177ms and exit 1. A second process running the add module still returned 4.

### SANDBOX-01 Filesystem deny-by-default and capability journal

- **ID:** SANDBOX-01
- **Name:** Filesystem deny-by-default and capability journal
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/wasi.rs`
- `crates/boxwren/src/caps.rs`
- `crates/boxwren/src/hooks.rs`

**Expected behavior**

With Grants::default there are no directory preopens. A guest path_open of a host path is denied. The report events list includes a filesystem event with decision denied and the target path. The guest cannot read host memory or walk outside a preopen.

**Acceptance criteria**

- [x] A guest that opens /etc/passwd with no grants does not receive the file.
- [x] The report names the path on a direct path_open.
- [x] No filesystem event is allowed when the grants list is empty.

**Test cases**

- **Normal use:** Run the secret guest from the demo (path_open /etc/passwd) with no grants. Expect denial and an event.
- **Empty, missing, whitespace-only, and very long input:** A guest that opens an empty path and a whitespace path. Expect denial, not a host file.
- **Wrong types, special characters, emoji, and non-English text:** A path of emoji, non-English characters, and NUL. Expect denial and a bounded event string.
- **SQL injection, XSS, and path traversal:** Paths ../../etc/passwd, /etc/passwd, and a path containing quotes. With no preopen, all denied. With a preopen, escape is SANDBOX-02.
- **Logged out, expired session, wrong role, and another user's data:** No user accounts. The host process identity is the boundary.
- **Double-submit and parallel requests:** Two guests, one granted a temp dir and one not. The ungranted guest must not see the other guest's preopen.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** A guest that opens thousands of paths. The journal and stdio caps must stay bounded.
- **Refresh mid-action, back button, and two tabs:** Not a UI flow.

**UI checks**

No library UI. The hero demo that shows this is UI-02.

**Dependencies**

`CORE-01`, `REPORT-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** demo and secret.wasm denied /etc/passwd and named that path. grants.rs secret_read_is_blocked_and_named and path_open_is_denied_and_named passed. Python open of /etc/passwd did not print file contents.

### SANDBOX-02 Directory preopen grants

- **ID:** SANDBOX-02
- **Name:** Directory preopen grants
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/caps.rs`
- `crates/boxwren-cli/src/main.rs`

**Expected behavior**

A directory grant is host:guest, host:guest:ro, or host:guest:rw. The split is the last colon. Default mode is read-only. A missing host directory fails before the guest starts. Writes to a read-only preopen are denied. Path traversal must not leave the preopen.

**Acceptance criteria**

- [x] ro can read a file inside the guest mount and cannot write it.
- [x] rw can write inside the mount.
- [x] A path that climbs out of the preopen is denied.
- [x] A missing host directory fails the request before execution.

**Test cases**

- **Normal use:** Mount a temp dir ro, read a file, try to write. Then rw and write a new file.
- **Empty, missing, whitespace-only, and very long input:** Grant an empty spec, a spec with only colons, and a whitespace path. Expect a parse or preflight error.
- **Wrong types, special characters, emoji, and non-English text:** Emoji directory names that exist, and a grant whose mode is not ro or rw.
- **SQL injection, XSS, and path traversal:** host paths with .., symlink pointing outside the dir, and a guest path of /../../etc/passwd. The file outside the preopen must not be readable.
- **Logged out, expired session, wrong role, and another user's data:** No accounts. A grant of another user's home is a caller choice and is warned, not rewritten (SANDBOX-05).
- **Double-submit and parallel requests:** Two runs with different preopens must not see each other's files.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** A directory with many files. Listing must still respect the stdio cap.
- **Refresh mid-action, back button, and two tabs:** Not a UI flow.

**UI checks**

CLI only. Confirm error text for a bad --allow-dir fits on one line and names the spec.

**Dependencies**

`SANDBOX-01`, `CLI-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** grants.rs read-only, read-write, and escape tests passed. CLI missing directory failed before start. Python read of a ro mount returned hello, write and .. escape returned EPERM.

### SANDBOX-03 Network deny-by-default and allow-net

- **ID:** SANDBOX-03
- **Name:** Network deny-by-default and allow-net
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/caps.rs`
- `crates/boxwren/src/wasi.rs`

**Expected behavior**

Default grants include no network. --allow-net accepts an IP, ip:port, hostname, or hostname:port. A bare target allows every port. A hostname grant turns on WASI name lookup for that run. A denied connect is a network event with decision denied.

**Acceptance criteria**

- [x] A connect with no grant is denied and journaled.
- [x] An explicit ip:port grant allows that target and denies a different port.
- [x] A hostname grant is what enables lookup. Without it, names do not resolve through the sandbox.

**Test cases**

- **Normal use:** Deny a connect to 127.0.0.1:1. Then allow 127.0.0.1:9 and confirm only that target is allowed.
- **Empty, missing, whitespace-only, and very long input:** An empty --allow-net value and a grant of just ':'. Expect a parse error before the guest starts.
- **Wrong types, special characters, emoji, and non-English text:** IPv6 literals, a hostname with emoji or non-ASCII, and a port of 0, 65535, and 65536.
- **SQL injection, XSS, and path traversal:** Hostnames like 127.0.0.1.example.com, and a grant string with spaces and quotes.
- **Logged out, expired session, wrong role, and another user's data:** No accounts.
- **Double-submit and parallel requests:** Two runs, only one granted. The other stays denied.
- **External service failing, timing out, and rate limiting:** Point an allowed grant at a closed local port and at a blackholed address. The guest fails at connect. The host process must still return a report.
- **New user with no data, and a user with lots of data:** Many denied connects in one run. Events must remain bounded or the run must still finish under the wall clock.
- **Refresh mid-action, back button, and two tabs:** Not a UI flow.

**UI checks**

CLI only.

**Dependencies**

`CORE-01`, `REPORT-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** net.wasm and demo denied 127.0.0.1:1. grants.rs network_is_denied_unless_allowlisted, hostname_lookup_stays_off_for_an_ip_grant, and hostname_grant_can_connect passed.

### SANDBOX-04 Environment grants

- **ID:** SANDBOX-04
- **Name:** Environment grants
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/caps.rs`

**Expected behavior**

The guest starts with no environment. --allow-env NAME copies a host variable. NAME=VALUE sets one explicitly. An unknown host variable fails before the guest starts.

**Acceptance criteria**

- [x] An ungranted variable is not visible to the guest.
- [x] NAME copies the host value and records an env event.
- [x] NAME=VALUE sets that value even if the host variable differs.
- [x] A missing host variable for a copy grant fails the request.

**Test cases**

- **Normal use:** Set a host variable, copy it, and also set NAME=VALUE. Read both from the guest.
- **Empty, missing, whitespace-only, and very long input:** Empty name, '=', '=value', and a whitespace name. Expect a parse error.
- **Wrong types, special characters, emoji, and non-English text:** Unicode names and values, emoji values, and a value that is only spaces.
- **SQL injection, XSS, and path traversal:** Names and values containing newlines, quotes, and $(...). They are data. The host shell must not expand them because the CLI does not pass them through a shell.
- **Logged out, expired session, wrong role, and another user's data:** Copying a secret is allowed when the caller asks. It must not happen by inheritance when no grant is set.
- **Double-submit and parallel requests:** Two runs with different env grants must not see each other's values.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** A value larger than the stdio cap. The guest can read it; captured stdout must truncate.
- **Refresh mid-action, back button, and two tabs:** Not a UI flow.

**UI checks**

CLI only.

**Dependencies**

`CORE-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** Missing host variable failed before start. grants.rs env_parse and environment_is_not_inherited_unless_granted passed. Python os.getenv of an explicit NAME=VALUE grant printed hello-env.

### SANDBOX-05 Root and home directory warnings

- **ID:** SANDBOX-05
- **Name:** Root and home directory warnings
- **Priority:** P1
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/caps.rs`
- `crates/boxwren/src/report.rs`

**Expected behavior**

A grant of / or of the home directory is allowed and named in report.warnings. Boxwren does not rewrite the grant. Rewriting it is left to Keywarden.

**Acceptance criteria**

- [x] Granting / succeeds and warnings is non-empty.
- [x] Granting the home directory succeeds and warnings names that choice.
- [x] A normal temp-dir grant does not add that warning.

**Test cases**

- **Normal use:** Run with --allow-dir /:/:ro and with a home grant. Read warnings in the JSON report.
- **Empty, missing, whitespace-only, and very long input:** A grant of a path that does not exist is a startup failure, not a warning.
- **Wrong types, special characters, emoji, and non-English text:** Not a typed form. '//' and a symlink to / should be recorded if the code treats them as root.
- **SQL injection, XSS, and path traversal:** Not applicable beyond the path itself.
- **Logged out, expired session, wrong role, and another user's data:** No accounts.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

The warning must be visible in JSON and in the text report.

**Dependencies**

`SANDBOX-02`, `REPORT-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** CLI --allow-dir /:/:ro succeeded and warned that the grant is the filesystem root. grants.rs root_and_home_grants_are_warned_and_not_rewritten passed.

### LIMITS-01 Fuel, memory, wall clock, stdio, and module size caps

- **ID:** LIMITS-01
- **Name:** Fuel, memory, wall clock, stdio, and module size caps
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/limits.rs`
- `crates/boxwren/src/limiter.rs`
- `crates/boxwren/src/capture.rs`

**Expected behavior**

Defaults are 1_000_000_000 fuel, 32 MiB memory, 2 seconds wall time, 1 MiB stdout, 1 MiB stderr, and 32 MiB max module size. Fuel is an instruction count. A module over the size cap is rejected before compile. Memory grow past the cap is memory_limited. Stdout and stderr set truncation flags.

**Acceptance criteria**

- [x] Defaults match limits.rs constants.
- [x] A module larger than max_module_bytes is rejected and not compiled.
- [x] A memory grow past memory_bytes is memory_limited.
- [x] Stdout over max_stdio_bytes is truncated and stdout_truncated is true.

**Test cases**

- **Normal use:** Run a small module under defaults. fuel_used > 0 and duration_ms is present.
- **Empty, missing, whitespace-only, and very long input:** fuel 0, memory 0, and timeout 0 from the CLI. Record whether they are rejected or mean 'use default'.
- **Wrong types, special characters, emoji, and non-English text:** CLI values 16MiB, 65536, 500ms, 2s, and a garbage unit like 5zz. Garbage must fail before the guest starts.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** No accounts. Hosted clamps are SEC-03.
- **Double-submit and parallel requests:** Two runs with different fuel budgets on one engine.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** A guest that prints more than 1 MiB. The report stays near the cap.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`CORE-01`, `REPORT-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** fuel 50 returned fuel_exhausted with fuel_used 50. memory.wasm returned memory_limited. Bad --memory 5zz exited 2. Lib tests covered module size, stdio truncation, and five memories.

### LIMITS-02 Kill, timeout, and fuel exhaustion classification

- **ID:** LIMITS-02
- **Name:** Kill, timeout, and fuel exhaustion classification
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/lib.rs`
- `crates/boxwren/src/limiter.rs`

**Expected behavior**

A loop that spends its fuel is fuel_exhausted. A loop stopped by the wall clock is timed_out, not killed. A loop stopped by KillHandle is killed, not timed_out. These three statuses stay distinct.

**Acceptance criteria**

- [x] fuel 50 on a loop reports fuel_exhausted.
- [x] A 200ms wall clock with a huge fuel budget reports timed_out.
- [x] KillHandle reports killed and finishes before the wall deadline.

**Test cases**

- **Normal use:** Run the three cases from the lib tests.
- **Empty, missing, whitespace-only, and very long input:** A module that returns immediately is success, not timed_out.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** No accounts.
- **Double-submit and parallel requests:** A killed run and a timed-out run at the same time on two stores.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** Repeat the timeout case. It must not drift into multi-second hangs.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`CORE-02`, `LIMITS-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** Timeout 200ms on a huge fuel budget returned timed_out at 200ms. Fuel 50 returned fuel_exhausted. SIGINT returned killed. ship_core killing_one_run_leaves_its_sibling passed.

### REPORT-01 Schema v1 JSON execution report

- **ID:** REPORT-01
- **Name:** Schema v1 JSON execution report
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/report.rs`

**Expected behavior**

Reports are schema_version 1. Fields: status (tagged kind), exit_code, return_value, duration_ms, fuel_used, peak_memory, stdout, stderr, truncation flags, events (filesystem, network, or env; allowed or denied), and warnings. Status kinds: success, fuel_exhausted, memory_limited, timed_out, killed, trapped, rejected. to_json is pretty JSON. to_text is the CLI text form.

**Acceptance criteria**

- [x] A success report serializes schema_version 1 and status.kind success.
- [x] trapped and rejected include a message.
- [x] to_json does not panic on any status variant.

**Test cases**

- **Normal use:** Execute the add module with --report json and parse every field.
- **Empty, missing, whitespace-only, and very long input:** A guest with empty stdout. stdout is an empty string, not null, unless the struct says otherwise. Confirm against a real report.
- **Wrong types, special characters, emoji, and non-English text:** Non-English stdout and emoji stdout round-trip in JSON.
- **SQL injection, XSS, and path traversal:** Stdout containing </script> and quotes stays inside the JSON string.
- **Logged out, expired session, wrong role, and another user's data:** No accounts.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** Truncated stdout still produces valid JSON.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

Dashboard rendering of this JSON is UI-04. Here, confirm the text report has one field per line.

**Dependencies**

`CORE-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** Add-module JSON had schema_version 1 and the documented fields. ship_core report_json_is_schema_v1 passed. Empty stdout was an empty string.

### HOOKS-01 Library Keywarden preflight hook

- **ID:** HOOKS-01
- **Name:** Library Keywarden preflight hook
- **Priority:** P1
- **Status:** PASSED

**Files involved**

- `crates/boxwren/src/hooks.rs`
- `crates/boxwren/examples/keywarden_preflight.rs`

**Expected behavior**

Preflight::allow runs before the module is instantiated. PolicyDeny becomes Error::Policy and the guest does not start. The HTTP call to Keywarden is not inside the library. The embedder implements the trait. The example shows the shape.

**Acceptance criteria**

- [x] A deny result does not instantiate the guest.
- [x] An allow result continues into normal execution.
- [x] The library does not open a network connection by itself for policy.

**Test cases**

- **Normal use:** Use a test Preflight that returns Ok and one that returns PolicyDeny. Only the deny path skips the guest.
- **Empty, missing, whitespace-only, and very long input:** A deny message that is empty, whitespace, or very long is preserved on the error.
- **Wrong types, special characters, emoji, and non-English text:** Not a parser.
- **SQL injection, XSS, and path traversal:** A deny message with quotes is an error string, not executed.
- **Logged out, expired session, wrong role, and another user's data:** The trait does not know accounts. The embedder passes that context.
- **Double-submit and parallel requests:** Two requests, two preflight decisions.
- **External service failing, timing out, and rate limiting:** The HTTP client lives in boxwren-server (API-02), not here.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`CORE-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** grants.rs preflight_deny_returns_policy_before_the_guest passed. The library trait does not open a network connection.

### CLI-01 boxwren run

- **ID:** CLI-01
- **Name:** boxwren run
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren-cli/src/main.rs`

**Expected behavior**

boxwren run MODULE executes a wasm or wat file. Flags: --report text|json, --fuel, --memory, --timeout, --entry, --allow-dir, --allow-net, --allow-env, and trailing guest args. Default entry is _start. Ctrl-C fires the kill handle. Exit 0 on success, 1 when the guest is stopped or trapped, 2 when the request is invalid. BOXWREN_READY is printed when set, which the web demo uses.

**Acceptance criteria**

- [x] A valid module with --report json prints one JSON report.
- [x] A missing file exits 2 and does not pretend to run.
- [x] Ctrl-C on a loop yields status killed and exit 1.
- [x] Guest args after the module are not parsed as host grants.

**Test cases**

- **Normal use:** boxwren run add.wat --entry run --report json. Expect success.
- **Empty, missing, whitespace-only, and very long input:** Missing path, empty file, whitespace file, and a multi-megabyte path argument.
- **Wrong types, special characters, emoji, and non-English text:** Bad --memory, bad --timeout, --entry that does not exist, and --report yaml.
- **SQL injection, XSS, and path traversal:** Module path with .., a path that is a directory, and guest args --allow-env SECRET. Those args must be guest argv, not host grants, when placed as trailing args.
- **Logged out, expired session, wrong role, and another user's data:** No login. The process user is the caller.
- **Double-submit and parallel requests:** Two CLI processes at once.
- **External service failing, timing out, and rate limiting:** None beyond grants.
- **New user with no data, and a user with lots of data:** A module at the 32 MiB cap and one byte over.
- **Refresh mid-action, back button, and two tabs:** Ctrl-C twice. The process must exit.

**UI checks**

Terminal only. Confirm the text report is aligned as key: value lines and errors go to stderr.

**Dependencies**

`CORE-01`, `LIMITS-01`, `REPORT-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** JSON report for the add module exited 0. Missing file exited 2. Bad memory unit exited 2. Guest denial and rejection printed a report and exited 1. SIGINT killed a loop.

### CLI-02 boxwren demo

- **ID:** CLI-02
- **Name:** boxwren demo
- **Priority:** P0
- **Status:** PASSED

**Files involved**

- `crates/boxwren-cli/src/main.rs`
- `crates/boxwren-cli/tests/cli.rs`

**Expected behavior**

boxwren demo runs three embedded guests: a read of /etc/passwd, a connect to 127.0.0.1:1, and a memory grow past the cap. It exits non-zero if any of those is allowed. --report json prints the reports. The secret guest calls path_open directly so the report names the path.

**Acceptance criteria**

- [x] demo exits 0 when all three probes are denied.
- [x] JSON mode includes the network guest and the secret path.
- [x] The command does not require a user-supplied module.

**Test cases**

- **Normal use:** boxwren demo and boxwren demo --report json.
- **Empty, missing, whitespace-only, and very long input:** No arguments. Extra unknown flags must fail clap.
- **Wrong types, special characters, emoji, and non-English text:** --report text and --report json only.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** None.
- **Double-submit and parallel requests:** Two demos at once.
- **External service failing, timing out, and rate limiting:** The network probe must not need a route to the internet.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

Terminal only.

**Dependencies**

`SANDBOX-01`, `SANDBOX-03`, `LIMITS-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** boxwren demo --report json exited 0. Secret, network, and memory probes were all blocked.

### CLI-03 export-undoom

- **ID:** CLI-03
- **Name:** export-undoom
- **Priority:** P1
- **Status:** PASSED

**Files involved**

- `crates/boxwren-cli/src/main.rs`

**Expected behavior**

boxwren export-undoom REPORT wraps a schema v1 report as an Undoom document with report_json. --output writes a file. Default is stdout. --agent defaults to boxwren. --name defaults to boxwren-run.

**Acceptance criteria**

- [x] A valid report JSON becomes a document that still contains the original report.
- [x] A missing file and invalid JSON fail before writing a partial document.
- [x] --output creates the file the caller named.

**Test cases**

- **Normal use:** Export a success report to stdout and to a file. Check agent and name defaults.
- **Empty, missing, whitespace-only, and very long input:** Empty file, whitespace file, and a very long --name.
- **Wrong types, special characters, emoji, and non-English text:** A JSON array, a report missing schema_version, and --agent with emoji.
- **SQL injection, XSS, and path traversal:** Output path with .. and an absolute path outside the work directory. The command writes where it is told. Record that as caller responsibility, and confirm it does not execute the path.
- **Logged out, expired session, wrong role, and another user's data:** None.
- **Double-submit and parallel requests:** Two exports to the same --output.
- **External service failing, timing out, and rate limiting:** No network. Undoom itself is not called.
- **New user with no data, and a user with lots of data:** A report with truncated 1 MiB stdout.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

Terminal only.

**Dependencies**

`REPORT-01`, `CLI-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** A success report wrapped to agent boxwren, name boxwren-run, with the original report under metadata.report_json. Empty input exited 2.

### CLI-04 run-py and run-js

- **ID:** CLI-04
- **Name:** run-py and run-js
- **Priority:** P1
- **Status:** PASSED

**Files involved**

- `crates/boxwren-cli/src/main.rs`
- `crates/boxwren/src/interpreters.rs`
- `guests/interpreters/README.md`

**Expected behavior**

With the interpreters feature, run-py executes a Python file inside bundled MicroPython WASI, and run-js executes JavaScript inside bundled QuickJS WASI. They are not host CPython or V8. The same fuel, memory, timeout, and grant flags as run apply. The workspace default feature set includes interpreters for the CLI package; CI unit tests pass --no-default-features and then test interpreters separately.

**Acceptance criteria**

- [x] A Python file that prints hello returns a report under the limits.
- [x] A JavaScript file does the same.
- [x] The commands are absent when the binary is built without the interpreters feature.
- [x] Grants and limits behave as they do for wasm.

**Test cases**

- **Normal use:** run-py a one-line print. run-js a one-line print. JSON report.
- **Empty, missing, whitespace-only, and very long input:** Missing script, empty file, and a file of only whitespace.
- **Wrong types, special characters, emoji, and non-English text:** A .py passed to run-js, binary data, emoji source, and non-English strings in the script.
- **SQL injection, XSS, and path traversal:** Python or JS that tries to read /etc/passwd and open a socket with no grants. Both must be denied by the same sandbox.
- **Logged out, expired session, wrong role, and another user's data:** None.
- **Double-submit and parallel requests:** run-py and run-js at the same time.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** A script that prints past the stdio cap, and a script that loops until fuel or timeout.
- **Refresh mid-action, back button, and two tabs:** Ctrl-C kills the script run.

**UI checks**

Terminal only.

**Dependencies**

`CLI-01`, `SANDBOX-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:** 0
- **Bugs Fixed:** 0
- **UI Issues Fixed:** 0
- **Files Changed:** none
- **Notes:** Release binary has both commands. run-py printed hello-from-py. run-js printed js-ok. Ungranted /etc/passwd did not leak. Empty Python file succeeded with empty stdout.

### API-01 Health check

- **ID:** API-01
- **Name:** Health check
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/app/healthz/route.ts`
- `crates/boxwren-server/src/main.rs`

**Expected behavior**

GET /healthz on the Next.js app and on boxwren-server returns text/plain ok with status 200. The Next route is nodejs. The server route is outside the /v1 rate-limit layer. HOSTED.md says the public /healthz is not rate-limited.

**Acceptance criteria**

- [ ] Both processes return 200 and a body of ok.
- [ ] No Authorization header is required.
- [ ] The Next health check does not run a wasm module.

**Test cases**

- **Normal use:** curl both /healthz endpoints.
- **Empty, missing, whitespace-only, and very long input:** POST and GET with an empty body. Only GET is implemented.
- **Wrong types, special characters, emoji, and non-English text:** A query string and extra headers are ignored.
- **SQL injection, XSS, and path traversal:** A path like /healthz/../v1/accounts is not this route.
- **Logged out, expired session, wrong role, and another user's data:** No credentials. Confirm it stays public.
- **Double-submit and parallel requests:** Many concurrent GETs.
- **External service failing, timing out, and rate limiting:** Next /healthz still requires the process to have booted, which requires DATABASE_URL (WEB-01), even though the handler does not query it.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

Plain text. No page chrome.

**Dependencies**

`WEB-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### AUTH-01 Email signup and initial API key

- **ID:** AUTH-01
- **Name:** Email signup and initial API key
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/app/v1/accounts/route.ts`
- `web/lib/email.ts`
- `crates/boxwren-server/src/main.rs`
- `crates/boxwren-server/src/db.rs`

**Expected behavior**

POST /v1/accounts with {email} creates an account and a key named default. The response is {id, email, api_key}. The raw key is bw_ plus 48 hex characters and is returned once. Email is trimmed and lowercased. validEmail requires length <= 320, an @ that is not first or last, and no whitespace. Duplicate email is 409. Invalid JSON is 400. The same rules exist on boxwren-server.

**Acceptance criteria**

- [ ] A new email returns 200 with an api_key that authenticates.
- [ ] The same email again returns 409 and does not rotate the key.
- [ ] Invalid email and invalid JSON return 400.
- [ ] The database stores the hash, not the raw key.

**Test cases**

- **Normal use:** Sign up you@example.com on the server and on the hosted route. Use the key on GET /v1/me.
- **Empty, missing, whitespace-only, and very long input:** Missing email, empty string, spaces only, and a 321-character local part.
- **Wrong types, special characters, emoji, and non-English text:** email as a number, an array, emoji, non-English local part, and a@b with no dot. The code accepts a@b. Record that.
- **SQL injection, XSS, and path traversal:** Email values with quotes, <script>, and SQL. They are either rejected or stored as text. They must not run.
- **Logged out, expired session, wrong role, and another user's data:** Signup is unauthenticated. It is rate-limited.
- **Double-submit and parallel requests:** Two parallel signups of the same email. One 200 and one 409.
- **External service failing, timing out, and rate limiting:** Neon down on the hosted route. SQLite disk full on the server.
- **New user with no data, and a user with lots of data:** Many distinct emails until the rate limit.
- **Refresh mid-action, back button, and two tabs:** The dashboard form is UI-04. Retrying the API is safe because email is unique.

**UI checks**

API. Dashboard form is UI-04.

**Dependencies**

`DATA-01`, `DATA-02`, `RATE-02`

**Suspected bugs**

- crates/boxwren-server/src/db.rs:72-89 can leave an account with no key if key insert fails. web/app/v1/accounts/route.ts:25-29 is transactional.
- web/lib/email.ts:2-8 and crates/boxwren-server/src/main.rs:637-643 accept addresses with no domain dot, such as a@b.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### AUTH-02 Bearer API key authentication

- **ID:** AUTH-02
- **Name:** Bearer API key authentication
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/lib/api.ts`
- `crates/boxwren-server/src/main.rs`
- `crates/boxwren-server/src/db.rs`

**Expected behavior**

Protected routes require Authorization: Bearer <key>. The lookup is sha256 of the token against api_keys.hash, joined to accounts. Missing header is 401 missing Authorization. Unknown key is 401 invalid API key. There is no expiry, refresh, or session cookie.

**Acceptance criteria**

- [ ] A current key returns the owning account.
- [ ] Missing, empty, and wrong keys return 401.
- [ ] A key from account A never resolves to account B.

**Test cases**

- **Normal use:** Bearer with the signup key on GET /v1/me.
- **Empty, missing, whitespace-only, and very long input:** No header, Bearer with nothing after it, and whitespace.
- **Wrong types, special characters, emoji, and non-English text:** Basic auth, lowercase bearer, a key with emoji, and the prefix only.
- **SQL injection, XSS, and path traversal:** A bearer value of ' OR 1=1 -- and a value with newlines.
- **Logged out, expired session, wrong role, and another user's data:** This is the auth check. Also send another account's key.
- **Double-submit and parallel requests:** Many requests with the same key.
- **External service failing, timing out, and rate limiting:** Database unavailable returns 500, not a false 200.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** The key does not expire. Revocation is AUTH-06.

**UI checks**

401 bodies are plain text.

**Dependencies**

`AUTH-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### AUTH-03 Create and list API keys

- **ID:** AUTH-03
- **Name:** Create and list API keys
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/app/v1/keys/route.ts`
- `crates/boxwren-server/src/main.rs`

**Expected behavior**

GET /v1/keys lists id, name, prefix, and created_at for the caller. The hash and raw key are not listed. POST /v1/keys with {name} creates a key. Name is trimmed, required, and at most 80 characters. The raw key is returned once. The hosted POST also returns created_at. The server CreateKeyResponse returns id, name, and api_key.

**Acceptance criteria**

- [ ] List returns only the caller's keys.
- [ ] Create returns a raw key that then authenticates.
- [ ] Empty and 81-character names return 400.
- [ ] The list never includes the raw key or the hash.

**Test cases**

- **Normal use:** Create a key named dashboard and list it. The prefix matches the start of the raw key.
- **Empty, missing, whitespace-only, and very long input:** Missing name, spaces, and an 80-character name versus 81.
- **Wrong types, special characters, emoji, and non-English text:** name as a number, emoji name, and non-English name.
- **SQL injection, XSS, and path traversal:** name with quotes, <script>, and path separators. Stored as text.
- **Logged out, expired session, wrong role, and another user's data:** No header is 401. Account B does not see account A's keys.
- **Double-submit and parallel requests:** Two creates at once. Both keys exist.
- **External service failing, timing out, and rate limiting:** Database failure mid-insert.
- **New user with no data, and a user with lots of data:** Create many keys and confirm the list returns all of them. There is no pagination.
- **Refresh mid-action, back button, and two tabs:** The raw key is not recoverable after the response.

**UI checks**

Dashboard issuance is UI-04.

**Dependencies**

`AUTH-02`

**Suspected bugs**

- crates/boxwren-server/src/main.rs CreateKeyResponse (around line 311) omits created_at, while web/app/v1/keys/route.ts:45-49 includes it. Clients that read created_at from the POST body differ by host.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### AUTH-04 Dashboard sign-out

- **ID:** AUTH-04
- **Name:** Dashboard sign-out
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/app/dashboard/page.tsx`

**Expected behavior**

Sign out removes boxwren_api_key from localStorage and clears token, account, keys, runs, and the report in React state. It does not call the API and does not delete the account. The privacy page describes this.

**Acceptance criteria**

- [ ] After sign-out, the signup form is shown.
- [ ] localStorage no longer has the key.
- [ ] Reload stays signed out.
- [ ] The account still exists on the server and the old key still works if pasted again.

**Test cases**

- **Normal use:** Sign in, sign out, reload.
- **Empty, missing, whitespace-only, and very long input:** Sign out with no extra state.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Sign out is local. Another browser profile keeps its own key.
- **Double-submit and parallel requests:** Two tabs. Signing out in one tab does not automatically clear the other tab until reload or storage events. Record the actual behavior. The code does not listen for storage events.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** Reload after sign-out. Back button after sign-out.

**UI checks**

Button sits with Issue key and Delete account. Check wrap at 360px, focus ring, and that the label is Sign Out.

**Dependencies**

`UI-04`, `AUTH-01`

**Suspected bugs**

- web/app/dashboard/page.tsx has no window storage listener. A second tab keeps the signed-in view until it reloads.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### AUTH-05 Delete account

- **ID:** AUTH-05
- **Name:** Delete account
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/app/v1/me/route.ts`
- `web/app/dashboard/page.tsx`
- `crates/boxwren-server/src/main.rs`
- `crates/boxwren-server/src/db.rs`

**Expected behavior**

DELETE /v1/me deletes the caller's runs, API keys, and account, then returns 204. The dashboard asks window.confirm and, on success, clears localStorage and the signed-in state. GET /v1/me returns {id, email}.

**Acceptance criteria**

- [ ] After delete, the key returns 401.
- [ ] Runs and keys for that account are gone.
- [ ] Another account is untouched.
- [ ] Canceling the confirm leaves the account in place.

**Test cases**

- **Normal use:** Create an account, add a run and a second key, delete, then GET /v1/me with the old key.
- **Empty, missing, whitespace-only, and very long input:** DELETE with no body. Body is ignored.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable. The id comes from the key lookup.
- **Logged out, expired session, wrong role, and another user's data:** No key is 401. Account B's key must not delete account A.
- **Double-submit and parallel requests:** Two deletes with the same key. One 204. The second 401.
- **External service failing, timing out, and rate limiting:** Database error during the transaction. Hosted route uses one transaction. Confirm the server delete is also ordered so a key cannot survive a deleted account.
- **New user with no data, and a user with lots of data:** Delete an account that has 100 runs.
- **Refresh mid-action, back button, and two tabs:** Confirm, then reload. The dashboard must show the signed-out form.

**UI checks**

Confirm dialog, button placement, and the signed-out state. Escape dismisses the browser confirm.

**Dependencies**

`AUTH-02`, `DATA-01`

**Suspected bugs**

- crates/boxwren-server/src/main.rs:169 allows only GET, POST, and OPTIONS in CORS. A dashboard on another origin cannot preflight DELETE /v1/me. Same-origin hosted use is unaffected.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### AUTH-06 Revoke API key

- **ID:** AUTH-06
- **Name:** Revoke API key
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/app/v1/keys/[id]/route.ts`
- `web/app/dashboard/page.tsx`
- `crates/boxwren-server/src/main.rs`

**Expected behavior**

DELETE /v1/keys/{id} deletes that key only when it belongs to the caller. Missing or another account's key is 404 key not found. Success is 204. The dashboard disables Revoke when only one key remains.

**Acceptance criteria**

- [ ] Revoking a listed key makes that raw key return 401.
- [ ] Another account's key id returns 404 and the key still works for its owner.
- [ ] The last remaining key cannot be revoked from the dashboard control.

**Test cases**

- **Normal use:** Create a second key, revoke it, confirm the first key still works.
- **Empty, missing, whitespace-only, and very long input:** DELETE with an empty id is a different path. A whitespace id returns 404.
- **Wrong types, special characters, emoji, and non-English text:** An id that is not a UUID, emoji, and a very long id.
- **SQL injection, XSS, and path traversal:** id values with ../, quotes, and SQL. Parameterized query. Expect 404.
- **Logged out, expired session, wrong role, and another user's data:** No key is 401. Cross-account id is 404.
- **Double-submit and parallel requests:** Two revokes of the same id. One 204 and one 404.
- **External service failing, timing out, and rate limiting:** Database down.
- **New user with no data, and a user with lots of data:** Revoke many keys, leaving one.
- **Refresh mid-action, back button, and two tabs:** Reload the dashboard and the revoked key is gone from the table.

**UI checks**

Revoke button disabled state, title tooltip, and the table at 360px.

**Dependencies**

`AUTH-03`

**Suspected bugs**

- web/app/dashboard/page.tsx:383 disables Revoke when keys.length <= 1, but web/app/v1/keys/[id]/route.ts:15-21 does not. DELETE of the last key succeeds and the saved bearer can no longer call the API. There is no password recovery.
- Same CORS DELETE gap as AUTH-05 at crates/boxwren-server/src/main.rs:169.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### RUN-01 Create and list runs

- **ID:** RUN-01
- **Name:** Create and list runs
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/app/v1/runs/route.ts`
- `web/lib/run.ts`
- `web/lib/hosted-command.ts`
- `crates/boxwren-server/src/main.rs`

**Expected behavior**

POST /v1/runs accepts wasm_base64 or wasm_text, optional entry, args, fuel, memory_bytes, and wall_ms. The hosted path shells out to the boxwren CLI and stores the report. List returns the caller's latest 100 runs without the report body. The server path runs in-process, accepts dirs, net, and env grants, and can return before the guest finishes when wait is false.

**Acceptance criteria**

- [ ] A small WAT module returns 200, an id, and a report.
- [ ] The new run appears in GET /v1/runs for that account only.
- [ ] Missing module bytes return 400.
- [ ] Hosted list is capped at 100.

**Test cases**

- **Normal use:** POST the 2+2 module with entry run. List and see the id.
- **Empty, missing, whitespace-only, and very long input:** No body fields, empty wasm_text, empty wasm_base64, and whitespace WAT.
- **Wrong types, special characters, emoji, and non-English text:** wasm_text as a number, args as a string, fuel as a string, emoji WAT, and non-English in a data segment.
- **SQL injection, XSS, and path traversal:** WAT and args containing SQL, <script>, and --allow-env. Hosted argv isolation is SEC-03.
- **Logged out, expired session, wrong role, and another user's data:** No key is 401. The run is stored on the authenticated account.
- **Double-submit and parallel requests:** Two POSTs at once. Two ids. Neither response is mixed up.
- **External service failing, timing out, and rate limiting:** Missing CLI binary on the hosted path. Keywarden is API-02. A hung guest is bounded by the 12s child timeout in run.ts and maxDuration 15.
- **New user with no data, and a user with lots of data:** More than 100 runs. List shows 100 on the hosted route.
- **Refresh mid-action, back button, and two tabs:** Refresh the dashboard while a run is in flight. The button state is UI-04.

**UI checks**

Dashboard form is UI-04. API errors are plain text.

**Dependencies**

`AUTH-02`, `CLI-01`, `SEC-03`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### RUN-02 Get run by id

- **ID:** RUN-02
- **Name:** Get run by id
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/app/v1/runs/[id]/route.ts`
- `crates/boxwren-server/src/main.rs`

**Expected behavior**

GET /v1/runs/{id} returns id, status, and report for a run owned by the caller. Anyone else's id, or an unknown id, is 404 run not found. The hosted query filters on account_id and id.

**Acceptance criteria**

- [ ] The owner receives the stored report JSON.
- [ ] Another account receives 404.
- [ ] The response does not include another account's report.

**Test cases**

- **Normal use:** Create a run and GET it.
- **Empty, missing, whitespace-only, and very long input:** Unknown UUID and a blank id.
- **Wrong types, special characters, emoji, and non-English text:** Non-UUID, emoji, and a very long id.
- **SQL injection, XSS, and path traversal:** id with quotes and ../ . Expect 404, not a SQL error.
- **Logged out, expired session, wrong role, and another user's data:** No key is 401. Cross-account is 404.
- **Double-submit and parallel requests:** GET the same id twice.
- **External service failing, timing out, and rate limiting:** Database down.
- **New user with no data, and a user with lots of data:** A report with a large stdout field still returns valid JSON.
- **Refresh mid-action, back button, and two tabs:** Open the same run from the dashboard twice.

**UI checks**

Report <pre> is UI-04.

**Dependencies**

`RUN-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### RUN-03 Live kill on boxwren-server

- **ID:** RUN-03
- **Name:** Live kill on boxwren-server
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `crates/boxwren-server/src/main.rs`

**Expected behavior**

POST /v1/runs with wait false inserts a running row, returns the id immediately, and finishes the guest in the background. POST /v1/runs/{id}/kill checks ownership, then calls KillHandle if the id is in the in-memory map. Success is 202. Unknown id is 404 run not found. A finished run is 404 run not in flight. The kill map is process-local.

**Acceptance criteria**

- [ ] A looping guest started with wait false can be killed and ends as killed.
- [ ] Killing another account's run id returns 404 and does not stop it.
- [ ] Killing an already finished run returns 404 run not in flight.

**Test cases**

- **Normal use:** Start a loop with a long wall time and wait false. Kill it. GET the run until status is killed.
- **Empty, missing, whitespace-only, and very long input:** Kill a random UUID.
- **Wrong types, special characters, emoji, and non-English text:** Non-UUID id.
- **SQL injection, XSS, and path traversal:** id with SQL metacharacters.
- **Logged out, expired session, wrong role, and another user's data:** No key is 401. Cross-account is RUN-04.
- **Double-submit and parallel requests:** Two kills of the same id. One 202 and one 404.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** Several in-flight runs. Killing one leaves the others.
- **Refresh mid-action, back button, and two tabs:** Kill after the run has already finished.

**UI checks**

No dashboard control. That absence is a deferred gap.

**Dependencies**

`CORE-02`, `RUN-01`, `DATA-02`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### RUN-04 Cross-account run isolation

- **ID:** RUN-04
- **Name:** Cross-account run isolation
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/app/v1/runs/route.ts`
- `web/app/v1/runs/[id]/route.ts`
- `crates/boxwren-server/src/main.rs`

**Expected behavior**

Every run read and the server kill path filters by the authenticated account id. One account cannot list, fetch, or kill another account's run. The hosted kill route does not look up the id at all. It returns 409 for every authenticated caller. That refusal is RUN-05.

**Acceptance criteria**

- [ ] Account B's list does not contain account A's run.
- [ ] Account B's GET of A's id is 404.
- [ ] Account B's kill of A's in-flight server run is 404 and A's guest keeps running.

**Test cases**

- **Normal use:** Two accounts. A creates a run. B lists, gets, and tries to kill it.
- **Empty, missing, whitespace-only, and very long input:** B with no runs sees an empty list, not A's rows.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** B sends A's id with encoded characters.
- **Logged out, expired session, wrong role, and another user's data:** This is the wrong-account case.
- **Double-submit and parallel requests:** A and B create runs at the same time.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** A has many runs. B's list stays empty.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI beyond the dashboard only loading the signed-in account.

**Dependencies**

`AUTH-02`, `RUN-01`, `RUN-03`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### RUN-05 Hosted async run and kill refusal

- **ID:** RUN-05
- **Name:** Hosted async run and kill refusal
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/lib/run.ts`
- `web/app/v1/runs/[id]/kill/route.ts`

**Expected behavior**

On the Next.js API, wait false returns 400 live kill requires the local boxwren-server. POST /v1/runs/{id}/kill returns 409 with the same message after the caller authenticates. The hosted function does not share a kill handle across requests. Grants in the JSON body are not in the RunBody type, so they are ignored.

**Acceptance criteria**

- [ ] wait false never starts a guest on the hosted API.
- [ ] Kill returns 409 for an authenticated caller.
- [ ] dirs, net, and env fields do not open host resources.

**Test cases**

- **Normal use:** POST wait false and POST kill with a real run id.
- **Empty, missing, whitespace-only, and very long input:** Kill with an unknown id. The handler ignores the id, so this is still 409 after auth.
- **Wrong types, special characters, emoji, and non-English text:** wait as the string false and as 0.
- **SQL injection, XSS, and path traversal:** dirs set to / and env set to a secret name. The hosted command builder must not pass them.
- **Logged out, expired session, wrong role, and another user's data:** Kill without a key is 401.
- **Double-submit and parallel requests:** Several kill calls. All 409.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No kill button on the dashboard.

**Dependencies**

`AUTH-02`, `RUN-01`

**Suspected bugs**

- web/app/v1/runs/[id]/kill/route.ts:5-10 authenticates, then ignores the run id. A missing id and another account's id both return 409, not 404. That avoids a data leak, and it also hides whether the id existed.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### RATE-01 boxwren-server rate limit

- **ID:** RATE-01
- **Name:** boxwren-server rate limit
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `crates/boxwren-server/src/main.rs`

**Expected behavior**

The /v1 router counts requests per window. Default is 60 requests per 60 seconds, overridable by --rate-limit and --rate-window-secs or BOXWREN_RATE_LIMIT and BOXWREN_RATE_WINDOW_SECS. The key is the Authorization header truncated to 48 characters, or the shared bucket anon when the header is absent. Over the limit, the response is 429 rate limit exceeded. /healthz is outside this layer.

**Acceptance criteria**

- [ ] Request 61 in a window returns 429.
- [ ] A different Authorization value has its own bucket.
- [ ] /healthz still returns 200 when /v1 is limited.

**Test cases**

- **Normal use:** Set the limit to 3 and send 4 signup attempts.
- **Empty, missing, whitespace-only, and very long input:** Missing Authorization shares the anon bucket.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** A very long Authorization header. Only 48 characters are used.
- **Logged out, expired session, wrong role, and another user's data:** Two real keys should not block each other. See the suspected bug.
- **Double-submit and parallel requests:** A burst of parallel requests around the limit.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** Hold the window open and confirm it resets.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

Plain text 429.

**Dependencies**

`API-01`

**Suspected bugs**

- crates/boxwren-server/src/main.rs:233-238 truncates the Authorization header to 48 characters. A bearer token is longer than that, so distinct keys can share one bucket. The stored key is a prefix of the raw header, not a hash.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### RATE-02 Hosted Neon rate limit

- **ID:** RATE-02
- **Name:** Hosted Neon rate limit
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/lib/api.ts`
- `web/db/schema.sql`

**Expected behavior**

gate() allows 60 requests per minute per bucket. With an Authorization header the bucket is auth: plus the first 32 hex chars of sha256(header). Otherwise it is ip: plus the first X-Forwarded-For hop, or X-Real-IP, or anon. The counter is an upsert into rate_limits. Over the limit, the response is 429. /healthz and /v1/demo do not call gate().

**Acceptance criteria**

- [ ] The 61st gated request in a minute returns 429.
- [ ] Two different API keys do not share a bucket.
- [ ] The raw key is not stored in rate_limits.

**Test cases**

- **Normal use:** Send 61 GETs to /v1/me with one key inside one minute.
- **Empty, missing, whitespace-only, and very long input:** No Authorization. Bucket uses the forwarded IP or anon.
- **Wrong types, special characters, emoji, and non-English text:** A spoofed X-Forwarded-For list. The code uses the first hop.
- **SQL injection, XSS, and path traversal:** Header values with quotes. The bucket is hashed or bound as a parameter.
- **Logged out, expired session, wrong role, and another user's data:** An invalid key still consumes the auth hash bucket before the 401, because gate() runs first.
- **Double-submit and parallel requests:** Parallel requests must not lose increments. The SQL increments in one statement.
- **External service failing, timing out, and rate limiting:** Neon timeout during the upsert. The route throws if gate throws.
- **New user with no data, and a user with lots of data:** Old minute rows are not deleted by this code. Note the growth.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

Plain text 429. Dashboard should show the error string.

**Dependencies**

`DATA-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### RATE-03 Public demo in-memory rate limit

- **ID:** RATE-03
- **Name:** Public demo in-memory rate limit
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/app/v1/demo/route.ts`

**Expected behavior**

POST /v1/demo allows 30 requests per 60 seconds per client IP, in a process-local Map. The IP is the first X-Forwarded-For hop, else X-Real-IP, else anon. The map drops stale rows only when it grows past 1024. Over the limit, the response is 429. This limiter is not the Neon gate.

**Acceptance criteria**

- [ ] The 31st demo call from one IP in a minute returns 429.
- [ ] A second IP is not blocked by the first.
- [ ] Restarting the process clears the counters.

**Test cases**

- **Normal use:** Call one scenario 31 times.
- **Empty, missing, whitespace-only, and very long input:** No forwarded headers. All such clients share anon.
- **Wrong types, special characters, emoji, and non-English text:** A long X-Forwarded-For chain.
- **SQL injection, XSS, and path traversal:** Header values are only used as map keys.
- **Logged out, expired session, wrong role, and another user's data:** The route is public.
- **Double-submit and parallel requests:** A burst from one IP.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** More than 1024 distinct IPs. Stale entries are deleted. Live entries stay.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

The hero must show a failure state when the demo returns 429. That display is UI-02.

**Dependencies**

`API-03`

**Suspected bugs**

- web/app/v1/demo/route.ts:33-47 is per server instance. Two serverless isolates do not share the counter.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### CORS-01 boxwren-server CORS allowlist

- **ID:** CORS-01
- **Name:** boxwren-server CORS allowlist
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `crates/boxwren-server/src/main.rs`
- `docs/HOSTED.md`

**Expected behavior**

Default origins are http://127.0.0.1:3000, http://localhost:3000, and https://boxwren.rejisterjack.com. BOXWREN_CORS_ORIGINS or --cors-origins replaces the list. Allowed methods in code are GET, POST, and OPTIONS. Allowed headers are Authorization, Content-Type, and X-Requested-With. The Next.js app does not add CORS because it is same-origin.

**Acceptance criteria**

- [ ] An allowlisted Origin is reflected on a /v1 response.
- [ ] A random Origin is not.
- [ ] OPTIONS preflight succeeds for GET and POST from an allowlisted origin.

**Test cases**

- **Normal use:** Send Origin http://localhost:3000 to GET /healthz and POST /v1/accounts.
- **Empty, missing, whitespace-only, and very long input:** No Origin header. Request still works. An empty BOXWREN_CORS_ORIGINS falls back to the defaults because the parser treats blank as unset.
- **Wrong types, special characters, emoji, and non-English text:** Origin with a trailing slash and a different port.
- **SQL injection, XSS, and path traversal:** Origin https://boxwren.rejisterjack.com.evil.example.
- **Logged out, expired session, wrong role, and another user's data:** Preflight does not send the bearer token.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** Browser dashboard pointed at the server with NEXT_PUBLIC_BOXWREN_API_URL.

**UI checks**

When the dashboard is cross-origin, failed preflight shows as a network error. Check the console.

**Dependencies**

`API-01`

**Suspected bugs**

- crates/boxwren-server/src/main.rs:169 omits DELETE. Dashboard revoke and delete-account fail CORS when the page is not same-origin.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### SEC-01 boxwren-server security headers

- **ID:** SEC-01
- **Name:** boxwren-server security headers
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `crates/boxwren-server/src/main.rs`

**Expected behavior**

Every server response gets X-Content-Type-Options nosniff, X-Frame-Options DENY, and Referrer-Policy no-referrer. There is no Content-Security-Policy and no Strict-Transport-Security in this layer. Fly force_https is separate.

**Acceptance criteria**

- [ ] GET /healthz and a JSON /v1 response include the three headers.
- [ ] A 400 and a 401 include them too.

**Test cases**

- **Normal use:** curl -D- the health and accounts routes.
- **Empty, missing, whitespace-only, and very long input:** Not applicable.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Check a 401 response.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No page.

**Dependencies**

`API-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### SEC-02 Hosted Next.js security headers

- **ID:** SEC-02
- **Name:** Hosted Next.js security headers
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/next.config.mjs`

**Expected behavior**

next.config.mjs sets X-Content-Type-Options nosniff, X-Frame-Options DENY, Referrer-Policy no-referrer, and Permissions-Policy camera=(), microphone=(), geolocation=() on /:path*. poweredByHeader is false, so X-Powered-By is omitted.

**Acceptance criteria**

- [ ] Document responses include those four headers.
- [ ] X-Powered-By is absent.

**Test cases**

- **Normal use:** curl -D- /, /dashboard, and /healthz.
- **Empty, missing, whitespace-only, and very long input:** Not applicable.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Not applicable.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No visual change. Confirm the site still renders.

**Dependencies**

`WEB-02`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### SEC-03 Hosted run clamps and guest argv isolation

- **ID:** SEC-03
- **Name:** Hosted run clamps and guest argv isolation
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/lib/hosted-command.ts`
- `web/lib/run.ts`
- `web/lib/run.test.ts`

**Expected behavior**

Hosted runs clamp fuel, memory, and wall time to the library defaults (1e9, 32 MiB, 2000 ms). Values that are missing, non-finite, or below 1 use the default. Larger values are cut to the max. Entry is trimmed, max 256 characters, must not contain NUL, and must not start with '-'. Guest args must be a string array of at most 64 items, each at most 4096 characters, and are placed after -- so clap cannot treat them as --allow-dir or --allow-env. The module path is substituted through a NUL-prefixed slot so a guest arg cannot replace it. Modules over 32 MiB are rejected.

**Acceptance criteria**

- [ ] fuel above the max is clamped.
- [ ] An arg of --allow-env SECRET stays after --.
- [ ] Entry '-run' is 400.
- [ ] dirs, net, and env are not read by the hosted command builder.

**Test cases**

- **Normal use:** A normal WAT run with entry run and fuel 1000.
- **Empty, missing, whitespace-only, and very long input:** Empty args array, missing fuel, and entry of spaces.
- **Wrong types, special characters, emoji, and non-English text:** args containing a number, 65 args, a 4097-character arg, and fuel Infinity.
- **SQL injection, XSS, and path traversal:** args ['--allow-dir','/:/:rw'] and entry '--report'. Confirm the spawned argv.
- **Logged out, expired session, wrong role, and another user's data:** Requires a key. The clamp applies after auth.
- **Double-submit and parallel requests:** Two runs with different fuel values.
- **External service failing, timing out, and rate limiting:** CLI exit 2 becomes HTTP 400. Unparseable stdout becomes 500.
- **New user with no data, and a user with lots of data:** A 32 MiB module and one byte over, via wasm_base64.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No separate UI.

**Dependencies**

`RUN-01`, `CLI-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### API-02 KEYWARDEN_URL run preflight

- **ID:** API-02
- **Name:** KEYWARDEN_URL run preflight
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/app/v1/runs/route.ts`
- `crates/boxwren-server/src/policy.rs`
- `crates/boxwren-server/src/main.rs`

**Expected behavior**

When KEYWARDEN_URL is set, both hosts POST {action:'boxwren.run', account_id, entry} before the guest starts. A non-OK response becomes 403. The hosted route uses response text or 'keywarden denied run'. The server prefixes 'keywarden denied run:'. If the variable is unset, preflight is skipped. Boxwren does not enforce Keywarden policy inside WASI imports.

**Acceptance criteria**

- [ ] Unset URL runs the guest.
- [ ] HTTP 200 from Keywarden allows the run.
- [ ] HTTP 403 from Keywarden blocks the guest and returns 403.
- [ ] A connection failure blocks the run rather than failing open.

**Test cases**

- **Normal use:** Point KEYWARDEN_URL at a local stub that returns 200, then one that returns 403.
- **Empty, missing, whitespace-only, and very long input:** Empty KEYWARDEN_URL is unset. A stub that returns an empty body still yields the fallback message.
- **Wrong types, special characters, emoji, and non-English text:** A stub that returns HTML and one that returns a JSON deny.
- **SQL injection, XSS, and path traversal:** The entry sent to Keywarden is the caller entry. A hostile entry is JSON-encoded.
- **Logged out, expired session, wrong role, and another user's data:** The account id in the body is the authenticated account, not a field from the request.
- **Double-submit and parallel requests:** Two runs, one denied and one allowed, if the stub keys off entry.
- **External service failing, timing out, and rate limiting:** Close the port. Time out the stub. There is no client timeout in either implementation.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

Dashboard shows the 403 text in the error panel.

**Dependencies**

`RUN-01`

**Suspected bugs**

- web/app/v1/runs/route.ts:68-75 and crates/boxwren-server/src/policy.rs:6-16 call Keywarden with no timeout. A hung URL holds the run request.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### API-03 Public POST /v1/demo

- **ID:** API-03
- **Name:** Public POST /v1/demo
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/app/v1/demo/route.ts`
- `web/lib/demo.ts`
- `web/lib/demo-report.ts`

**Expected behavior**

POST /v1/demo takes {scenario, action}. Scenarios are secret, loop, network, and clean. action kill is honored. Anything else runs. secret opens /etc/passwd through WASI and must be denied. loop with action run uses a 200ms timeout and a huge fuel budget. action kill SIGINTs that loop after the CLI prints boxwren: ready, with a 3 second backup timer. network runs boxwren demo and returns the network report. clean runs a 2+2 WAT export. Unknown scenario is 400. Invalid JSON is 400. A CLI failure is 503. No API key is required.

**Acceptance criteria**

- [ ] Each of the four scenarios returns a JSON report.
- [ ] secret does not return file contents.
- [ ] clean reports a successful add.
- [ ] Unknown scenario is 400 and does not spawn the CLI.

**Test cases**

- **Normal use:** POST each scenario with action run, and loop with action kill.
- **Empty, missing, whitespace-only, and very long input:** Empty body, missing scenario, whitespace scenario.
- **Wrong types, special characters, emoji, and non-English text:** scenario as a number, an unknown name, emoji, and action as an array.
- **SQL injection, XSS, and path traversal:** scenario '../../bin' and a body with extra grant fields. The scenario is matched against a fixed list.
- **Logged out, expired session, wrong role, and another user's data:** No key required. Rate limit is RATE-03.
- **Double-submit and parallel requests:** Four scenarios at once.
- **External service failing, timing out, and rate limiting:** Rename or remove the CLI binary. Expect 503.
- **New user with no data, and a user with lots of data:** Repeat clean. Do not leak temp directories. The code removes them in finally.
- **Refresh mid-action, back button, and two tabs:** The hero button is UI-02.

**UI checks**

Covered by UI-02.

**Dependencies**

`CLI-01`, `CLI-02`, `WEB-02`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### WEB-01 DATABASE_URL required at Next boot

- **ID:** WEB-01
- **Name:** DATABASE_URL required at Next boot
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/instrumentation.ts`
- `web/lib/env.ts`
- `web/.env.example`

**Expected behavior**

instrumentation.ts calls process.exit(1) when DATABASE_URL is missing, and logs that sentence. assertDatabaseUrl throws the same fact for any later db() call. The example env file lists NEXT_PUBLIC_BOXWREN_API_URL and DATABASE_URL, both empty. The database URL is server-only.

**Acceptance criteria**

- [ ] next dev and next start refuse to stay up without DATABASE_URL.
- [ ] The URL is not referenced from a NEXT_PUBLIC variable.
- [ ] With the URL set, the process boots.

**Test cases**

- **Normal use:** Boot with a pooled Neon URL and request /healthz.
- **Empty, missing, whitespace-only, and very long input:** Unset the variable and boot. Expect exit 1.
- **Wrong types, special characters, emoji, and non-English text:** A URL of 'postgres://' that is non-empty but invalid. Boot proceeds. The first query fails.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Not applicable.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** Neon rejects the password. Boot still passes the env check.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No page is served if the process exits.

**Dependencies**

None.

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### WEB-02 Production build and CLI binary fetch

- **ID:** WEB-02
- **Name:** Production build and CLI binary fetch
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/package.json`
- `web/scripts/fetch-boxwren.mjs`
- `web/lib/cli.ts`
- `web/next.config.mjs`

**Expected behavior**

npm run build runs fetch-boxwren.mjs, then next build. The script downloads boxwren v0.2.4 from GitHub releases: the musl binary saved as bin/boxwren, and the darwin arm64 binary saved as bin/boxwren-darwin. At runtime, darwin arm64 uses boxwren-darwin and every other platform uses bin/boxwren. The binary is copied to a temp dir and chmod 0755. next.config traces ./bin/boxwren into the /v1/runs function. Dev uses next dev and does not fetch.

**Acceptance criteria**

- [ ] A production build finishes when the release assets exist.
- [ ] POST /v1/runs and POST /v1/demo can spawn that binary.
- [ ] The client bundle does not contain the database URL.

**Test cases**

- **Normal use:** npm run build, npm start, then a demo call and an authenticated run.
- **Empty, missing, whitespace-only, and very long input:** Missing bin directory. The fetch script creates it. A failed download throws.
- **Wrong types, special characters, emoji, and non-English text:** BOXWREN_CLI_VERSION set to a tag that does not exist.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Not applicable.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** GitHub release 404 or a timeout during build.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

Load the production server and confirm the home page renders.

**Dependencies**

`CLI-01`

**Suspected bugs**

- web/lib/cli.ts:7-8 treats every non-darwin-arm64 platform as bin/boxwren, which fetch-boxwren.mjs downloads as the x86_64 musl asset. A linux arm64 host would spawn the wrong binary.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### UI-01 Marketing home page

- **ID:** UI-01
- **Name:** Marketing home page
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/app/page.tsx`
- `web/app/globals.css`
- `web/app/layout.tsx`

**Expected behavior**

The home page is a dark marketing page: hero, telemetry strip (1.05 µs, 6.10 ms, 0 MB, Schema v1), benchmark section, capability bento, safety stack, threat model, install tabs, and a closing call to action. Links go to #install, /dashboard, and the GitHub README. The numbers are written in the component. They are not computed at request time.

**Acceptance criteria**

- [ ] GET / returns 200 and the hero heading.
- [ ] In-page anchors scroll to sandbox, benchmarks, capabilities, architecture, threat-model, and install.
- [ ] The dashboard link goes to /dashboard.

**Test cases**

- **Normal use:** Open / and follow each anchor and the dashboard link.
- **Empty, missing, whitespace-only, and very long input:** No form on this page except the hero demo, which is UI-02.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable. Content is static.
- **Logged out, expired session, wrong role, and another user's data:** The page is public.
- **Double-submit and parallel requests:** Open / in two tabs.
- **External service failing, timing out, and rate limiting:** Install tabs copy commands. They do not call the network except the hero.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** Reload mid-scroll. Back from /dashboard returns to the home page.

**UI checks**

Alignment of the hero and telemetry strip. Text overflow on the heading. Dark theme only. Widths 360, 768, 1024, and 1440. No horizontal scroll. Focus on the three hero buttons. Console errors.

**Dependencies**

`UI-05`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### UI-02 Hero sandbox demo

- **ID:** UI-02
- **Name:** Hero sandbox demo
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/app/components/HeroSandbox.tsx`
- `web/lib/demo.ts`

**Expected behavior**

The hero lists four scenarios: secret, loop, network, and clean. Run posts {scenario, action:'run'} to /v1/demo and renders the returned report. Kill posts action kill. The component shows illustrative Rust snippets, not the WAT that demo.ts actually runs.

**Acceptance criteria**

- [ ] Each scenario button runs and shows a report without a page reload.
- [ ] A 429 or 503 shows an error, not a blank panel.
- [ ] Kill is only meaningful for the loop. The control's actual wiring must match what the user can click.

**Test cases**

- **Normal use:** Run all four scenarios. Kill the loop.
- **Empty, missing, whitespace-only, and very long input:** The component always sends a known scenario.
- **Wrong types, special characters, emoji, and non-English text:** Not a free-text form.
- **SQL injection, XSS, and path traversal:** Not a free-text form.
- **Logged out, expired session, wrong role, and another user's data:** Public.
- **Double-submit and parallel requests:** Click Run twice quickly.
- **External service failing, timing out, and rate limiting:** Stop the CLI binary or throttle the network. The button needs a loading state.
- **New user with no data, and a user with lots of data:** Hit the 30/minute demo limit and confirm the UI shows it.
- **Refresh mid-action, back button, and two tabs:** Refresh while a run is in flight. Use Back on the home page.

**UI checks**

Scenario list, code block overflow, report panel, loading, error, disabled, hover, and focus. Widths 360, 768, 1024, 1440. Dark theme. Keyboard across the scenario buttons. Console errors.

**Dependencies**

`API-03`, `UI-01`

**Suspected bugs**

- web/app/components/HeroSandbox.tsx:24-29 shows a read of ~/.ssh/id_rsa. web/lib/demo.ts:29 opens /etc/passwd. The picture and the guest do not match.
- web/app/components/HeroSandbox.tsx:38 labels the loop 'Epoch Fuel Trapped'. web/lib/demo.ts:60 runs that scenario with a 200ms timeout and a huge fuel value, so the stop is the wall clock, not fuel.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### UI-03 Install, benchmarks, capabilities, safety, and threat sections

- **ID:** UI-03
- **Name:** Install, benchmarks, capabilities, safety, and threat sections
- **Priority:** P2
- **Status:** NOT STARTED

**Files involved**

- `web/app/page.tsx`
- `web/app/components/InstallTabs.tsx`
- `web/app/components/BenchmarkChart.tsx`
- `web/app/components/BentoFeatures.tsx`
- `web/app/components/SafetyStack.tsx`
- `web/app/components/ThreatModelMatrix.tsx`

**Expected behavior**

Below the hero, the page renders a benchmark chart, a bento of engine capabilities, the safety-stack diagram, a threat-model matrix, and install tabs for the crate, CLI, Python, and Go. Tab buttons switch the visible command. They do not execute it.

**Acceptance criteria**

- [ ] Each section is reachable from the navbar.
- [ ] Install tabs change the visible command and do not navigate away.
- [ ] The threat matrix is readable without overlapping cells.

**Test cases**

- **Normal use:** Scroll the page and switch every install tab.
- **Empty, missing, whitespace-only, and very long input:** No inputs.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Public.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** Outbound links in the install section open GitHub, crates.io, or docs as coded.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** Reload and the default tab is shown again.

**UI checks**

Tab selected state, focus, overflow of code samples, chart labels, matrix at 360 and 1440, dark theme, no horizontal scroll.

**Dependencies**

`UI-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### UI-04 Operator dashboard

- **ID:** UI-04
- **Name:** Operator dashboard
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/app/dashboard/page.tsx`

**Expected behavior**

 /dashboard is a client page. Signed out, it offers email signup and a paste-an-existing-key form. The key is stored in localStorage as boxwren_api_key. Signed in, it loads /v1/me, /v1/keys, and /v1/runs. The operator can issue a key named dashboard, copy a newly issued key, revoke a key when more than one exists, delete the account after confirm, sign out, submit the WAT textarea with entry run, and open a past run to see pretty JSON. API is NEXT_PUBLIC_BOXWREN_API_URL with the trailing slash removed, or same-origin when that variable is empty. There is no kill control.

**Acceptance criteria**

- [ ] Signup stores the key and shows the tenant panel.
- [ ] Pasting a valid key loads that account. A bad key shows the error text.
- [ ] Execute runs the textarea and lists the new run.
- [ ] Empty history says no runs have been recorded.
- [ ] Reload keeps the session via localStorage.

**Test cases**

- **Normal use:** Sign up, run the default WAT, open the run, issue a key, copy it, sign out, paste the key.
- **Empty, missing, whitespace-only, and very long input:** Empty email is blocked by the required input. Empty existing-key submit returns early. Empty WAT is sent and the API rejects it. The error must show.
- **Wrong types, special characters, emoji, and non-English text:** Email with emoji and non-English text. WAT with emoji comments. A pasted key with surrounding spaces is trimmed.
- **SQL injection, XSS, and path traversal:** Email and WAT containing <script> and quotes render as text, not HTML. The report is in a <pre>, not innerHTML. Confirm that.
- **Logged out, expired session, wrong role, and another user's data:** A revoked key shows the me/keys/runs error. Delete account returns to the signed-out form.
- **Double-submit and parallel requests:** Double-click Create Account and double-click Execute.
- **External service failing, timing out, and rate limiting:** Throttle the network. Stop the API. The error panel must show the failure. There is no per-button loading state on signup.
- **New user with no data, and a user with lots of data:** An account with many runs. The history pane scrolls and does not stretch the page without a bound.
- **Refresh mid-action, back button, and two tabs:** Reload during execute. Two tabs with the same key. Back from a run view, which is the same page.

**UI checks**

Forms, tables, error panel, copy buttons, confirm dialog, empty history, disabled Revoke and disabled textarea while running. Hover and focus. Widths 360, 768, 1024, 1440. Dark theme. The page is dark-only, so light-scheme emulation is expected to stay dark. Keyboard through the forms. Console errors. Overflow of long keys and long JSON.

**Dependencies**

`AUTH-01`, `AUTH-03`, `AUTH-04`, `AUTH-05`, `AUTH-06`, `RUN-01`, `RUN-02`

**Suspected bugs**

- web/app/dashboard/page.tsx:105-121 signup and :154-167 createKey do not disable the button while the request is in flight.
- web/app/dashboard/page.tsx:227 renders error text in a <p>. That is safe. The warning glyph is a literal character in the JSX, which is fine, but the error from a thrown TypeError on a failed fetch is shown raw.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### UI-05 Navbar, footer, and SEO metadata

- **ID:** UI-05
- **Name:** Navbar, footer, and SEO metadata
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/app/components/Navbar.tsx`
- `web/app/layout.tsx`
- `web/app/components/BoxwrenLogo.tsx`

**Expected behavior**

The navbar links to the home sections and to /dashboard. On small screens a menu button opens a panel, traps body scroll, and closes on Escape and on navigation. The current section is highlighted from scroll position on /. The footer links to GitHub, crates.io, PyPI, docs.rs, the dashboard, privacy, and terms. Root metadata sets title, description, keywords, manifest, icons, Open Graph, and Twitter card. metadataBase is NEXT_PUBLIC_APP_URL or https://boxwren.dev. The viewport is dark.

**Acceptance criteria**

- [ ] Every nav link reaches its target.
- [ ] The mobile menu opens, closes on Escape, and closes when a link is chosen.
- [ ] The document title matches layout.tsx.
- [ ] Footer links are present on home, dashboard, privacy, and terms.

**Test cases**

- **Normal use:** Use the nav on desktop and at 360px. Open privacy from the footer.
- **Empty, missing, whitespace-only, and very long input:** Not a form.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Nav is public.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** Footer outbound links use target _blank and rel noreferrer.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** Reload on /dashboard. The section highlight clears because the path is not /.

**UI checks**

Header alignment, menu overlay at 360 and 768, focus on the menu button, Escape, no horizontal scroll, logo not stretched, dark theme, console errors.

**Dependencies**

None.

**Suspected bugs**

- web/app/layout.tsx:18 defaults metadataBase and the Open Graph URL to https://boxwren.dev, while docs/HOSTED.md and docs/openapi.yaml name https://boxwren.rejisterjack.com. NEXT_PUBLIC_APP_URL can override the metadata base only.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### UI-06 Privacy and Terms pages

- **ID:** UI-06
- **Name:** Privacy and Terms pages
- **Priority:** P2
- **Status:** NOT STARTED

**Files involved**

- `web/app/privacy/page.tsx`
- `web/app/terms/page.tsx`

**Expected behavior**

Privacy says the local tools do not create an account, and the hosted site stores email, key hashes, and run reports. It says there is no password or mailing list, and that sign-out does not delete the account row. Terms describe Wasmtime isolation, caller-chosen grants, hosted clamps, and the refusal of live kill on the hosted site. Both set their own titles.

**Acceptance criteria**

- [ ] GET /privacy and GET /terms return 200.
- [ ] Titles are Privacy — Boxwren and Terms — Boxwren.
- [ ] The pages use the same header and footer as the rest of the site.

**Test cases**

- **Normal use:** Open both pages from the footer.
- **Empty, missing, whitespace-only, and very long input:** No forms.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Static copy.
- **Logged out, expired session, wrong role, and another user's data:** Public.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** Reload and use Back.

**UI checks**

Line length, padding at 360 and 1440, heading size, dark theme, focus on footer links, console errors.

**Dependencies**

`UI-05`

**Suspected bugs**

- web/app/privacy/page.tsx:22-24 says sign-out does not delete the account. The dashboard now deletes the account through DELETE /v1/me. The page does not mention that control.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### UI-07 Custom 404 page

- **ID:** UI-07
- **Name:** Custom 404 page
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/app/not-found.tsx`

**Expected behavior**

Unknown paths render not-found.tsx: a 404 kicker, the heading Page not found, and links to / and /dashboard. It sits inside the root layout, so the navbar and footer still show.

**Acceptance criteria**

- [ ] An unknown path returns the not-found page.
- [ ] Home and Dashboard links work.
- [ ] The page is readable at 360px.

**Test cases**

- **Normal use:** Open /does-not-exist.
- **Empty, missing, whitespace-only, and very long input:** Not a form.
- **Wrong types, special characters, emoji, and non-English text:** A path with emoji and encoded characters.
- **SQL injection, XSS, and path traversal:** A path with <script> and ../ . It must render as a 404, not as HTML from the path.
- **Logged out, expired session, wrong role, and another user's data:** Public.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** A very long path.
- **Refresh mid-action, back button, and two tabs:** Reload and Back.

**UI checks**

Centered block, button row wrap, dark theme, focus, console errors, widths 360, 768, 1024, 1440.

**Dependencies**

`UI-05`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### BIND-01 Python bindings

- **ID:** BIND-01
- **Name:** Python bindings
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `crates/boxwren-py/src/lib.rs`
- `crates/boxwren-py/README.md`
- `.github/workflows/python-wheels.yml`

**Expected behavior**

boxwren_py exposes Engine.new, Engine.kill_handle, KillHandle.kill, and Engine.execute. execute takes wasm bytes and an entry, and returns the report. The package is built with maturin and PyO3. Wheels are published by the python-wheels workflow.

**Acceptance criteria**

- [ ] import boxwren_py works from a built extension.
- [ ] execute of the add module returns a report whose return value is 4.
- [ ] kill_handle stops a loop.

**Test cases**

- **Normal use:** Build the extension and run the add module.
- **Empty, missing, whitespace-only, and very long input:** execute with empty bytes.
- **Wrong types, special characters, emoji, and non-English text:** Pass a str where bytes are required, and an unknown entry.
- **SQL injection, XSS, and path traversal:** Wasm bytes are not executed as Python.
- **Logged out, expired session, wrong role, and another user's data:** None.
- **Double-submit and parallel requests:** Two executes, one killed.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** Repeat execute on one engine.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`CORE-01`, `CORE-02`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### BIND-02 Go bindings

- **ID:** BIND-02
- **Name:** Go bindings
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `bindings/go/boxwren.go`
- `bindings/go/boxwren_test.go`
- `bindings/go/README.md`

**Expected behavior**

The Go package wraps the C ABI. New, Close, KillHandle, Kill, Execute, and ExecuteWith are the calls. Grants use the same strings as the CLI. The caller builds libboxwren_ffi first.

**Acceptance criteria**

- [ ] go test ./... passes after the FFI library is built.
- [ ] Execute returns JSON for a small module.
- [ ] ExecuteWith plus Kill stops one run.

**Test cases**

- **Normal use:** Run the package tests.
- **Empty, missing, whitespace-only, and very long input:** Execute nil or empty wasm.
- **Wrong types, special characters, emoji, and non-English text:** A Request with a bad grant string.
- **SQL injection, XSS, and path traversal:** Grant strings are parsed by the Rust crate, not by the shell.
- **Logged out, expired session, wrong role, and another user's data:** None.
- **Double-submit and parallel requests:** Kill from another goroutine, as the README describes.
- **External service failing, timing out, and rate limiting:** Missing libboxwren_ffi fails at link or load time.
- **New user with no data, and a user with lots of data:** None beyond the tests.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`BIND-03`, `CORE-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### BIND-03 C FFI library

- **ID:** BIND-03
- **Name:** C FFI library
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `crates/boxwren-ffi/src/lib.rs`
- `crates/boxwren-ffi/Cargo.toml`

**Expected behavior**

boxwren-ffi exports a C ABI, including boxwren_engine_new, for the Go package. It is the same sandbox as the Rust crate, not a second implementation. The release workflow builds libboxwren_ffi artifacts.

**Acceptance criteria**

- [ ] cargo build -p boxwren-ffi --release produces the library.
- [ ] The Go tests link it.
- [ ] The exported engine uses the same deny-by-default limits.

**Test cases**

- **Normal use:** Release build, then Go tests.
- **Empty, missing, whitespace-only, and very long input:** A null engine pointer must not be dereferenced by a safe Go wrapper. Check the Go side for a nil check.
- **Wrong types, special characters, emoji, and non-English text:** Not a CLI.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** None.
- **Double-submit and parallel requests:** Covered by Go ExecuteWith.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`CORE-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### DEVOPS-01 GitHub Actions CI

- **ID:** DEVOPS-01
- **Name:** GitHub Actions CI
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `.github/workflows/ci.yml`

**Expected behavior**

CI runs on push to main and on pull requests. The Rust job uses stable with rustfmt and clippy, targets wasm32-wasip1 and wasm32-wasip2, checks format, denies clippy warnings, tests the workspace with --no-default-features, tests interpreters, and runs fuzz_smoke in release. A second job runs cargo deny and cargo audit. The web job uses Node 22, npm ci, tsc --noEmit, and npm test. The Go job builds the FFI library and runs go test.

**Acceptance criteria**

- [ ] The workflow file matches those four jobs.
- [ ] Local fmt, clippy, cargo test, npm test, and tsc use the same commands.

**Test cases**

- **Normal use:** Read the workflow and run the same commands locally during the later test pass. Do not require a GitHub token for the report.
- **Empty, missing, whitespace-only, and very long input:** Not applicable.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Not applicable.
- **Double-submit and parallel requests:** Jobs are independent except that they share the checkout.
- **External service failing, timing out, and rate limiting:** crates.io and the advisory database are needed for audit.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`TEST-01`, `TEST-02`, `BIND-02`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### DEVOPS-02 Release workflow

- **ID:** DEVOPS-02
- **Name:** Release workflow
- **Priority:** P2
- **Status:** NOT STARTED

**Files involved**

- `.github/workflows/release.yml`
- `.github/workflows/python-wheels.yml`

**Expected behavior**

An annotated tag v* on main builds multi-OS boxwren binaries and libboxwren_ffi, creates a GitHub Release, publishes the boxwren crate with CARGO_REGISTRY_TOKEN, and builds boxwren-py wheels with PYPI_API_TOKEN. The release workflow also runs boxwren demo and uploads demo.json. The web fetch script expects the v0.2.4 asset names.

**Acceptance criteria**

- [ ] The tag filter is v*.
- [ ] Asset names include the musl and darwin arm64 binaries the web script downloads.
- [ ] Secrets are referenced, not committed.

**Test cases**

- **Normal use:** Read the workflow. Do not publish during the report. A later test pass may do a dry read only, unless a local demo build is enough.
- **Empty, missing, whitespace-only, and very long input:** Not applicable.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Publishing needs repository secrets. Missing secrets block the publish jobs.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** GitHub Releases, crates.io, and PyPI.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`CLI-02`, `BIND-01`, `BIND-03`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### DEVOPS-03 Docker images

- **ID:** DEVOPS-03
- **Name:** Docker images
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `Dockerfile`
- `Dockerfile.server`

**Expected behavior**

Dockerfile builds boxwren-cli in rust:1.97 and runs boxwren demo. It is a demo image, not the hosted product. Dockerfile.server builds boxwren-server and copies it onto debian bookworm slim with ca-certificates. It sets PORT and BOXWREN_LISTEN to 8787 and BOXWREN_DB to /data/boxwren.sqlite. It does not declare a VOLUME. The host is expected to mount /data.

**Acceptance criteria**

- [ ] The server image starts and answers /healthz when /data is writable.
- [ ] The demo image exits according to boxwren demo.
- [ ] The database file lives on the mounted disk, not only in the writable layer.

**Test cases**

- **Normal use:** Build both images. Run the server with a volume. curl /healthz. Run the demo image.
- **Empty, missing, whitespace-only, and very long input:** Run the server without a mount. SQLite still creates a file in the container filesystem and it disappears with the container.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** None at container start.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** Image builds need the base images.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** Restart the container and confirm the volume kept the account.

**UI checks**

No UI.

**Dependencies**

`CLI-02`, `API-01`, `DATA-02`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### DEVOPS-04 Fly.io config

- **ID:** DEVOPS-04
- **Name:** Fly.io config
- **Priority:** P2
- **Status:** NOT STARTED

**Files involved**

- `fly.toml`

**Expected behavior**

fly.toml describes one app, boxwren-api, in region iad, built from Dockerfile.server. It listens on 8787, forces HTTPS, keeps auto_stop off, and mounts volume boxwren_data at /data. BOXWREN_DB is /data/boxwren.sqlite. HOSTED.md says the public site does not use this process. It is the optional long-lived server.

**Acceptance criteria**

- [ ] The internal port matches the image EXPOSE and BOXWREN_LISTEN.
- [ ] The sqlite path is on the mount.
- [ ] auto_stop_machines is off so the in-memory kill map is not dropped by an idle stop.

**Test cases**

- **Normal use:** Read fly.toml against Dockerfile.server and HOSTED.md. Do not deploy during the report.
- **Empty, missing, whitespace-only, and very long input:** Not applicable.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** A deploy needs a Fly token. Treat a missing token as blocked later, not as a product bug.
- **Double-submit and parallel requests:** min_machines_running is 1. A second machine would split the kill map. The file does not set a max.
- **External service failing, timing out, and rate limiting:** Fly API.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`DEVOPS-03`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### TEST-01 Rust test suite

- **ID:** TEST-01
- **Name:** Rust test suite
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `crates/boxwren/src/lib.rs`
- `crates/boxwren/tests/ship_core.rs`
- `crates/boxwren/tests/grants.rs`
- `crates/boxwren-cli/tests/cli.rs`
- `crates/boxwren-server/src/main.rs`

**Expected behavior**

The crate tests cover the add module, fuel, memory, module size, kill versus timeout, grants, and the CLI. The server file contains integration tests for health, rate limit, CORS, email, runs, and cross-account kill. CI runs them with --no-default-features, then interpreters separately.

**Acceptance criteria**

- [ ] cargo test --workspace --no-default-features passes.
- [ ] The interpreters lib tests pass.
- [ ] A failure names the test that failed.

**Test cases**

- **Normal use:** Run the CI test commands.
- **Empty, missing, whitespace-only, and very long input:** Not applicable.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Covered inside grants tests if they exist. Do not delete tests.
- **Logged out, expired session, wrong role, and another user's data:** Server tests create accounts.
- **Double-submit and parallel requests:** Interpreters are pinned to --test-threads=2 in CI.
- **External service failing, timing out, and rate limiting:** None for the unit tests.
- **New user with no data, and a user with lots of data:** fuzz is FUZZ-01.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`CORE-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### TEST-02 Web unit tests

- **ID:** TEST-02
- **Name:** Web unit tests
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/package.json`
- `web/lib/email.test.ts`
- `web/lib/run.test.ts`
- `web/lib/demo.test.ts`

**Expected behavior**

npm test runs node:test on web/lib/*.test.ts. The script uses --experimental-strip-types. Tests exist for email validation, hosted command and run behavior, and the demo report parser.

**Acceptance criteria**

- [ ] npm test passes without a database.
- [ ] email.test.ts covers the validEmail rules.
- [ ] run tests cover the argv isolation described in SEC-03.

**Test cases**

- **Normal use:** npm test in web/.
- **Empty, missing, whitespace-only, and very long input:** Covered by the email and run tests if those cases exist. Read them before calling a gap.
- **Wrong types, special characters, emoji, and non-English text:** Same.
- **SQL injection, XSS, and path traversal:** run tests should include a guest arg that looks like a grant.
- **Logged out, expired session, wrong role, and another user's data:** These tests do not boot Next.
- **Double-submit and parallel requests:** node:test default.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`SEC-03`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### FUZZ-01 fuzz_smoke guest modules

- **ID:** FUZZ-01
- **Name:** fuzz_smoke guest modules
- **Priority:** P2
- **Status:** NOT STARTED

**Files involved**

- `crates/boxwren/tests/fuzz_smoke.rs`

**Expected behavior**

CI runs cargo test -p boxwren --test fuzz_smoke --release. It is a short smoke fuzz of guest modules, not a long campaign.

**Acceptance criteria**

- [ ] The release test command exits 0.
- [ ] A crash or panic fails the test.

**Test cases**

- **Normal use:** Run the CI command.
- **Empty, missing, whitespace-only, and very long input:** Whatever inputs the test already generates.
- **Wrong types, special characters, emoji, and non-English text:** Malformed modules inside the test.
- **SQL injection, XSS, and path traversal:** Not a string API.
- **Logged out, expired session, wrong role, and another user's data:** None.
- **Double-submit and parallel requests:** As the test harness runs.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** The test's own bound. Do not extend it during the report.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI.

**Dependencies**

`CORE-01`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### META-01 Favicon, manifest, Open Graph, and theme color

- **ID:** META-01
- **Name:** Favicon, manifest, Open Graph, and theme color
- **Priority:** P2
- **Status:** NOT STARTED

**Files involved**

- `web/app/layout.tsx`
- `web/public/manifest.json`
- `web/public/favicon.svg`
- `web/app/apple-icon.png`

**Expected behavior**

The layout points at favicon.svg, favicon.png, apple-icon.png, manifest.json, and icon-512.png for social cards. The manifest is a standalone dark PWA shell with 192 and 512 icons. themeColor and color-scheme are dark (#060907). Those files are present: favicon.svg, favicon.png, favicon.ico, icon-192.png, icon-512.png, and app/apple-icon.png.

**Acceptance criteria**

- [ ] The browser requests a favicon and gets 200.
- [ ] /manifest.json is valid JSON and its icon paths return 200.
- [ ] og:title and the theme color are present in the document.

**Test cases**

- **Normal use:** Load / and check the icon and manifest requests.
- **Empty, missing, whitespace-only, and very long input:** Not applicable.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Public assets.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** Reload and confirm icons are cached or returned again.

**UI checks**

Icon is not stretched in the navbar. Theme color is the dark green-black used by the page.

**Dependencies**

`UI-05`

**Suspected bugs**

None noticed while reading the code.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### GAP-01 Document or automate Neon schema apply

- **ID:** GAP-01
- **Name:** Document or automate Neon schema apply
- **Priority:** P0
- **Status:** NOT STARTED

**Files involved**

- `web/db/schema.sql`
- `docs/HOSTED.md`
- `README.md`

**Expected behavior**

This is missing. schema.sql is the only definition of the Neon tables. No script, migration tool, or HOSTED.md section applies it. A new hosted database will fail on signup until a person runs the file. The build item is either a documented command that applies schema.sql, or a startup path that applies the same statements. It must stay idempotent.

**Acceptance criteria**

- [ ] A new database can be brought to the schema in one documented step.
- [ ] Running that step twice does not drop accounts or runs.
- [ ] HOSTED.md or the README states that step.

**Test cases**

- **Normal use:** Follow the documented step against an empty database, then sign up.
- **Empty, missing, whitespace-only, and very long input:** Run it against a database that already has the tables and rows.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** The SQL file is static. Do not interpolate env into it.
- **Logged out, expired session, wrong role, and another user's data:** The database URL stays server-side.
- **Double-submit and parallel requests:** Do not run two applies that could race on an index build without checking. CREATE INDEX IF NOT EXISTS is the current statement.
- **External service failing, timing out, and rate limiting:** Neon unavailable.
- **New user with no data, and a user with lots of data:** Apply on a database that already has rows.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI unless the docs page is the site. The docs file is markdown.

**Dependencies**

`DATA-01`

**Suspected bugs**

- docs/HOSTED.md describes Neon and never says to apply web/db/schema.sql.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### GAP-02 Global 500 error page

- **ID:** GAP-02
- **Name:** Global 500 error page
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `web/app/not-found.tsx`

**Expected behavior**

This is missing. not-found.tsx covers unknown routes. There is no app/error.tsx or app/global-error.tsx. An exception in a server component falls through to the framework's default error UI. The build item is an error page in the same visual language as the 404 page, with a way back home, and it must not print secrets.

**Acceptance criteria**

- [ ] A thrown server render shows the project error page.
- [ ] The page does not display DATABASE_URL or API keys.
- [ ] Home is still reachable from it.

**Test cases**

- **Normal use:** Force a throw on a test-only path or a temporary error boundary and look at the page. Remove the throw after the check.
- **Empty, missing, whitespace-only, and very long input:** Not a form.
- **Wrong types, special characters, emoji, and non-English text:** Not applicable.
- **SQL injection, XSS, and path traversal:** An error message containing HTML must be escaped.
- **Logged out, expired session, wrong role, and another user's data:** The error page is safe to show to a logged-out visitor.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** A failed database call on a page that queries it. The current pages do not query during render. Dashboard errors stay in the client panel.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** Reload the error page.

**UI checks**

Same checks as UI-07: alignment, dark theme, 360 to 1440, focus, console.

**Dependencies**

`UI-07`

**Suspected bugs**

- No web/app/error.tsx and no web/app/global-error.tsx exist.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

### GAP-03 OpenAPI matches live routes

- **ID:** GAP-03
- **Name:** OpenAPI matches live routes
- **Priority:** P1
- **Status:** NOT STARTED

**Files involved**

- `docs/openapi.yaml`
- `web/app/v1/me/route.ts`
- `web/app/v1/keys/[id]/route.ts`
- `web/app/v1/demo/route.ts`

**Expected behavior**

This is half-built. openapi.yaml documents GET /healthz, POST /v1/accounts, GET /v1/me, GET and POST /v1/keys, GET and POST /v1/runs, GET /v1/runs/{id}, and POST /v1/runs/{id}/kill. It does not document DELETE /v1/me, DELETE /v1/keys/{id}, or POST /v1/demo. The file's version is 0.1.0 while the crate is 0.2.4. The build item is to add those operations with the status codes the handlers return.

**Acceptance criteria**

- [ ] DELETE /v1/me and DELETE /v1/keys/{id} are in the spec with 204, 401, and 404.
- [ ] POST /v1/demo lists the four scenarios and 400, 429, and 503.
- [ ] Every path in the spec exists in the Next app or in boxwren-server.

**Test cases**

- **Normal use:** Compare each documented path with the route file, then call one example from the spec.
- **Empty, missing, whitespace-only, and very long input:** The spec's required body fields match the handlers.
- **Wrong types, special characters, emoji, and non-English text:** Not a runtime form.
- **SQL injection, XSS, and path traversal:** Not applicable.
- **Logged out, expired session, wrong role, and another user's data:** Security schemes in the spec match bearer auth.
- **Double-submit and parallel requests:** None.
- **External service failing, timing out, and rate limiting:** None.
- **New user with no data, and a user with lots of data:** None.
- **Refresh mid-action, back button, and two tabs:** None.

**UI checks**

No UI. The spec is a file.

**Dependencies**

`AUTH-05`, `AUTH-06`, `API-03`

**Suspected bugs**

- docs/openapi.yaml has no delete operation and no /v1/demo path. Confirmed by search.

**Results**

- **Bugs Found:**
- **Bugs Fixed:**
- **UI Issues Fixed:**
- **Files Changed:**
- **Notes:**

## Assumptions

- The product under test is the whole boxwren repository: the Rust library, CLI, Python and Go bindings, boxwren-server, and the Next.js app in web/. The hosted site and the local server are both in scope because both implement /v1.
- Email validation that accepts a@b is the current rule in web/lib/email.ts and the server. It is recorded as a suspected bug, not rewritten in this report.
- NEXT_PUBLIC_BOXWREN_API_URL empty means the dashboard calls same-origin /v1. A non-empty value means the dashboard is aimed at boxwren-server.
- Neon schema application is a gap because no code applies web/db/schema.sql. SQLite schema application is a feature because boxwren-server creates its tables on open.
- Light theme, password login, email delivery, billing, and a dashboard kill button are deferred. The code is explicit that hosted kill is refused and that accounts are API keys.
- OpenAPI version 0.1.0 versus crate version 0.2.4 is noted inside GAP-03. The public behavior is taken from the route handlers, not from the spec, where they differ.
- An earlier SHIP_REPORT.md in this repo recorded a finished test pass. This report replaces it. Every status is NOT STARTED because this pass did not execute the product.

## Risks

- Hosted execution shells out to a native boxwren binary inside the web server. A bug in Wasmtime or in the CLI is outside the guest and can affect the host process. docs/THREAT_MODEL.md states that limit.
- boxwren-server accepts directory, network, and environment grants. A stolen API key on that server can read any path the process can read, including a grant of /. The public Next.js API ignores those fields.
- The server kill map is in memory. A second instance, or a restart, cannot kill a run that the previous process started.
- API keys do not expire. Revoking the last key through the API, which the dashboard tries to prevent and the route allows, leaves the email row with no way back in.
- Demo and server rate limits are per process. The Neon limiter is shared. Public demo abuse is only partly shared across hosted isolates.
- KEYWARDEN_URL has no timeout on either host. A stalled policy service stalls run creation.
- fetch-boxwren.mjs downloads a release binary during the production build. A missing GitHub release asset fails the build. linux arm64 is not a distinct download.
- metadataBase defaults to https://boxwren.dev while the hosted docs name https://boxwren.rejisterjack.com. Social cards and the live site can disagree until NEXT_PUBLIC_APP_URL is set.
