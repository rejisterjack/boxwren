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

### P0-HOSTED Next.js /v1 clamps, argv isolation, kill 409

Done means the public site runs a module through the CLI with clamped limits, ignores grant-shaped guest argv, refuses `wait: false`, and refuses live kill.

Matrix: bad email, malformed JSON, logged out, oversized fuel and memory and wall, guest `--allow-dir` plus a `dirs` field, spinning module, cross-account get, kill, flag-shaped entry. N/A: Redis, mobile. The browser cases belong to the dashboard.

Evidence: bad email 400. Authenticated `{` is 400 `invalid JSON`. Logged-out run 401. A 2+2 module with fuel `1e12`, memory `1e12`, and wall `1e9` returned 4. `wait: false` 400 `live kill requires the local boxwren-server`. A `/etc/passwd` guest with `--allow-env DATABASE_URL`, `--allow-dir /:/:rw`, and `dirs: ["/:/:rw"]` was denied on `/etc/passwd` and stdout had no `root:`. A spin with fuel `1e15` and wall `60000` finished `fuel_exhausted` in 277ms, so the 1e9 fuel clamp held. Account B `GET` 404. Owner kill 409 with the same live-kill message. Entry `--allow-env` 400 `invalid entry`. `npm test` 4 passed, including the grant-argv regression in `web/lib/run.test.ts`. No bug. Verdict PASS.

---

### P0-DASHBOARD Dashboard signup, reload, WAT run

Done means a person can create an account in the browser, keep the key after reload and in a second tab, and run the sample module.

Matrix: create account, empty email, duplicate email, reload, second tab, sample WAT, whitespace WAT, keyboard labels, 360px width. N/A: password login, Redis. The dashboard stores a bearer key in `localStorage`.

Evidence: creating `dashboard-gate@example.com` showed the tenant and stored a `bw_` key. Reload and a second tab both restored that tenant and the run list. The sample module reported schema v1, status success, `return_value` 4, fuel 4. Whitespace WAT was stored as `rejected`. An empty email was blocked by the required field (`Please fill in this field.`). Submitting the same email showed `account already exists`. The email and WAT fields expose accessible names. At 360px the signed-in page overflowed: the version tag and GitHub control stayed in the header, and the report `<pre>` widened the panels. Fix: hide the version tag below 680px, force the GitHub control off below 900px, let panels shrink, and wrap the report text. After the fix, 360px `innerWidth` had `scrollWidth` 360 with the tenant still visible, and executing the sample again returned 4 with no horizontal overflow. `npx tsc --noEmit` passed. Verdict FIXED.

---

### P0-RATELIMIT Server 429 and Neon 60 per minute

Done means the local API starts refusing after its configured limit, health stays up, and the hosted API refuses the 61st authenticated call in a minute.

Matrix: four anonymous calls against a limit of 3, health during the limit, 65 authenticated hosted calls inside one minute, error text has no stack. N/A: Redis, browser. The counters are in-process for the server and a Postgres row for the site.

Evidence: server on port 8790 with `--rate-limit 3` returned 401, 401, 401, then 429 `rate limit exceeded`. `GET /healthz` stayed 200. Hosted `GET /v1/me` inside one minute returned 200 sixty times and 429 `rate limit exceeded` on request 61, five times in all. The body had no stack trace. No bug. Verdict PASS.

---

### P1-INTERPRETERS run-py and run-js WASI guests

Done means `run-py` and `run-js` execute source inside the bundled WASI interpreters, a host file read does not return host bytes, and a spin stops.

Matrix: hello Python, hello JavaScript, Python read of `/etc/passwd`, empty file, syntax error, missing file, JavaScript spin. N/A: browser, Neon, IDOR. These commands are local.

Evidence: `run-py hi.py` exit 0, stdout `hello-py`. `run-js hi.js` exit 0, stdout `hello-js`. `open("/etc/passwd")` exit 0 with empty stdout and stderr `OSError: [Errno 44] ENOENT`, no `root:`. Empty file exit 0. A syntax error stayed inside the guest stderr and did not leak a host path. Missing file exit 2. `while(true){}` with `--timeout 500ms` exit 1, status `fuel_exhausted` at 63ms. No bug. Verdict PASS.

---

### P1-PYTHON boxwren_py execute, kill, grant strings

Done means Python can run a module and a second Python thread can kill it before the wall clock fires.

Matrix: 2+2, empty module, bad grant string, kill from another Python thread. N/A: browser back button, SQL. This is an in-process library.

Evidence: `Engine.execute` on the add module returned 4. Empty bytes returned status `rejected` and did not raise. `dirs=["not-a-grant"]` raised `RuntimeError` naming the grant syntax. A loop with fuel `2**62` and wall 5000ms was killed by another Python thread in 55ms, status `killed`. `cargo test -p boxwren-py --lib` 2 passed. `cargo clippy -p boxwren-py --all-targets -- -D warnings` passed. The binding releases the GIL around Wasmtime in `crates/boxwren-py/src/lib.rs`. Verdict FIXED.

---

### P1-GO Go C ABI execute and kill

Done means the Go package runs a module through the release C ABI, a goroutine can kill a loop, and a bad grant string fails before the guest.

Matrix: export returns 4, kill, missing directory, unset environment variable. N/A: browser, SQL. The package is a cgo wrapper.

Evidence: `cargo build -p boxwren-ffi --release` then `go test ./...` in `bindings/go` passed in 0.418s, including `TestExportReturnsFour`, `TestKillStopsALoop`, and `TestGrantStringsMatchTheCLI`. No bug. Verdict PASS.

---

### P1-KEYWARDEN Admission preflight

Done means a run does not start when Keywarden returns 500 or is unreachable, and a 200 response lets the module return.

Matrix: stub 500, stub 200, connection refused. N/A: browser, Redis. Preflight is an HTTP call before Wasmtime.

Evidence: a stub returning 500 produced 403 `keywarden denied run: no` and no report. The stub received `{"action":"boxwren.run","entry":"run"}`. A stub returning 200 let the add module return 4. A closed port produced 403 `keywarden request failed: error sending request...` with no stack trace. No bug. Verdict PASS.

---

### P1-CORS Browser origin allowlist

Done means a listed origin is reflected and any other origin is not.

Matrix: `http://localhost:3010` on the server started with that origin, `http://evil.example`, preflight for both, default server with `http://localhost:3000` and `http://localhost:3010`. N/A: SQL, module fuel. CORS is a response header.

Evidence: the configured server reflected `access-control-allow-origin: http://localhost:3010` on `GET /healthz` and on `OPTIONS /v1/runs`, and allowed `GET,POST,OPTIONS` plus `authorization,content-type`. `http://evil.example` got 200 with no allow-origin header. The default server reflected `http://localhost:3000` and did not reflect `http://localhost:3010`. No bug. Verdict PASS.

---

### P1-SQLITE Run row survives process restart

Done means a finished run can be read with the same API key after the server process is replaced.

Matrix: create a run, stop the process, start it on the same file, `GET` the id, health. N/A: browser, Redis. The row is a SQLite table.

Evidence: run `02057606-27d1-49f3-970c-9603b2a8515d` returned 7 before the stop. After `boxwren-server` was started again on `/tmp/boxwren-ship-gate.sqlite`, `GET` returned 200, the same id, status success, `return_value` 7, schema 1. `GET /healthz` returned `ok`. No bug. Verdict PASS.

---

### P1-DEMO Home demo scenarios

Done means the four home scenarios run through `/v1/demo` and the page shows a denied host read.

Matrix: secret, loop, network, clean, kill, unknown scenario, empty body, malformed JSON, trimmed scenario name, the Host File button in the browser. N/A: account login, Redis. The demo is anonymous and rate-limited in the page process.

Evidence: secret denied `/etc/passwd` with empty stdout. Loop `timed_out`. Network stdout `blocked Permission denied` and event `127.0.0.1:1` denied. Clean returned 4. Kill returned `killed`. Unknown and empty bodies 400 `unknown scenario`. `{` 400 `invalid JSON`. A trimmed `secret` still denied `/etc/passwd`. In the Cursor browser the Host File button finished with `/etc/passwd` denied and no `root:` on the page. No bug. Verdict PASS.

---

### P1-HEADERS Security headers

Done means the API and the site tell browsers not to sniff content, not to frame the page, and not to send a referrer.

Matrix: `GET /healthz` and a 404 on the server, `/`, `/dashboard`, and `/healthz` on the site. N/A: module fuel, IDOR. This is response metadata.

Evidence before the fix: neither process sent `nosniff`, `DENY`, or `no-referrer`, and the site sent `X-Powered-By: Next.js`. Fix: the server sets the three headers on every response, including 404, in `security_headers`. The site sets those plus `Permissions-Policy` and turns `poweredByHeader` off. After restart, server health and 404 carried the three headers. Home, dashboard, and web health carried all four and no longer sent `X-Powered-By`. `healthz_ok` asserts the server headers. `cargo clippy -p boxwren-server --all-targets -- -D warnings` passed. Verdict FIXED.

---

### P1-AUDIT Dependency audit

Done means `cargo audit` and `npm audit` report no known advisory in the locked trees, or a remaining advisory is recorded with the reason it stays.

Matrix: `cargo audit` on `Cargo.lock`, `npm audit` on `web/package-lock.json`. N/A: browser, guest fuel. This is the dependency lock.

Evidence: `cargo audit` first reported eight Wasmtime 50.0.1 advisories, all fixed in 50.0.2 or newer. The workspace now requires `wasmtime` and `wasmtime-wasi` 50.0.2, and the lock resolved to 50.0.3. `cargo test -p boxwren --lib` 9 passed on that build. A second `cargo audit` printed no advisories. `npm audit` reported PostCSS `<=8.5.22` (high, XSS in CSS stringify) through Next 15.5.28's pinned `postcss@8.4.31`. The registry's Next fix is 16.4.6, a major upgrade. An override to `postcss@8.5.33` clears the advisory without that upgrade. After `npm install`, `npm audit` reported zero vulnerabilities and the site still returned 200 for `/` and `/dashboard`. Verdict FIXED.

---

### P1-ENV Missing DATABASE_URL fails clearly

Done means a Next process that serves `/v1` refuses to start, with a plain message, when `DATABASE_URL` is absent. `BOXWREN_LISTEN` and `BOXWREN_DB` stay optional.

Matrix: unset variable, empty variable. N/A: guest fuel, browser width. This is process startup.

Evidence: `assertDatabaseUrl()` throws `DATABASE_URL is not set` when the variable is unset. `register()` in `web/instrumentation.ts` prints that message and exits 1 when the variable is empty. The local server still defaults `BOXWREN_LISTEN` to `127.0.0.1:8787` and `BOXWREN_DB` to `boxwren-server.sqlite` when those are omitted. `npx tsc --noEmit` passed. Verdict FIXED.

---

### P1-LEGAL Privacy and terms

Done means the site has a page that says what it stores and a page that says what the sandbox does not claim, both linked from the footer.

Matrix: `/privacy` and `/terms` return 200, the footer links are on `/`. N/A: password reset, billing. There is no account password.

Evidence: `/privacy` 200 and names the stored email, the API-key hash, localStorage, and the absence of a password. `/terms` 200 and says the boundary is software fault isolation, not a virtual machine, and that the hosted site clamps limits. The home page footer links to both. Verdict FIXED.

---

### P1-A11Y Keyboard, labels, four widths

Done means `/` and `/dashboard` can be used from the keyboard and do not scroll sideways at 360, 768, 1024, and 1440.

Matrix: accessible names, the mobile menu button, page `scrollWidth` at the four widths on both routes. N/A: Redis, guest fuel. This is the Cursor browser.

Evidence: the operator email field is named `Operator Email`, the sample module field is named `WASM Text Format (WAT)`, and the scenario button is named `Execute scenario in sandbox`. At 1024 the home page overflowed to 1119px because the desktop nav was 1120px wide and the install tabs reported 1302px. The nav now collapses at 1200px, and the install tab row is constrained so it scrolls inside the card. After that, in the Cursor browser: home 360 `scrollWidth` 360 with the menu button and Privacy link, home 768 `scrollWidth` 768, home 1024 `scrollWidth` 1024, home 1440 `scrollWidth` 1440 with the desktop nav visible. Dashboard 360, 768, 1024, and 1440 each had `scrollWidth` equal to the viewport. Verdict FIXED.

---

### P0-RUNS Sync run, list, get, live kill

Done means an authenticated caller can run a module, read it back, and kill a background run. Guest argv that looks like a grant does not open the host.

Matrix: logged out, empty body, malformed JSON, whitespace module, bad WAT, happy 2+2, negative fuel, unicode argv, `--allow-dir` in guest argv, `../..` directory grant, bad grant syntax, list, get, missing id, double submit, `wait: false` plus kill. N/A: browser back button, Redis. The API is bearer-authenticated JSON.

Evidence: logged out 401. Empty body 400 `provide wasm_base64 or wasm_text`. `not-json` 400. Whitespace and `not wasm` stored as schema v1 `rejected`. Happy run status success, `return_value` 4. `fuel: -1` 422. Unicode argv still returns 4. Guest argv `--allow-dir /:/:rw` and dir grant `../..:/:rw` both denied `/etc/passwd` with no `root:` in stdout. Bad grant 400 before the guest. `GET` of the run id returned the same success report. Missing id 404 `run not found`. A second submit created a new id. `wait: false` returned `running` in 1ms, `POST .../kill` returned 202, and the stored report was `killed` at 58ms. A second kill was 404 `run not in flight`. No bug. Verdict PASS.

---


