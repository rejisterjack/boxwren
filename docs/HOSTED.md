# Hosted Boxwren

The public site at [boxwren.rejisterjack.com](https://boxwren.rejisterjack.com) is a Next.js app. Its `/v1` routes are the hosted API, and the dashboard at `/dashboard` calls those routes on the same origin. Neon Postgres stores accounts, API-key hashes, rate-limit counters, and run reports. Each run spawns the published `boxwren` CLI inside the request.

That host is still software fault isolation, not a virtual machine. A Vercel function cannot share a live kill handle with another request. `wait: false` returns 400, and `POST /v1/runs/{id}/kill` returns 409. Both tell the caller to use the local `boxwren-server` for a live kill.

Filesystem, network, and environment grants are ignored on the public API. Fuel, memory, and wall time are clamped to the library defaults (2 seconds). Optional `KEYWARDEN_URL` admits a run before the guest starts. `DATABASE_URL` is the pooled Neon connection string and stays server-only.

OpenAPI: [`openapi.yaml`](openapi.yaml).

## Run locally with a live kill

`boxwren-server` is the long-lived process. Its kill map is in memory, so it stays on one machine.

```bash
cargo run -p boxwren-server -- --listen 127.0.0.1:8787 --db /tmp/boxwren.sqlite
```

Create an account and run a module:

```bash
curl -s http://127.0.0.1:8787/v1/accounts -H 'content-type: application/json' \
  -d '{"email":"you@example.com"}'
# -> { "id", "email", "api_key" }

curl -s http://127.0.0.1:8787/v1/runs \
  -H "authorization: Bearer $API_KEY" \
  -H 'content-type: application/json' \
  -d '{"wasm_text":"(module (func (export \"run\") (result i32) i32.const 4))","entry":"run"}'
```

To kill a live run from another request, start with `"wait": false`, then
`POST /v1/runs/{id}/kill` while status is `running`. Point a separate dashboard
at this process with `NEXT_PUBLIC_BOXWREN_API_URL`. Leave that variable empty
on the public site so the browser calls same-origin `/v1`.

## CORS and rate limits

The public Next.js routes are same-origin, so they do not use CORS. They allow 60 `/v1` requests per minute, keyed by API key or client IP, and store the counter in Neon. `/healthz` is not rate-limited.

`boxwren-server` has its own limits:

| Env / flag | Default | Meaning |
| --- | --- | --- |
| `BOXWREN_CORS_ORIGINS` / `--cors-origins` | `http://127.0.0.1:3000`, `http://localhost:3000`, `https://boxwren.rejisterjack.com` | Comma-separated browser origin allowlist |
| `BOXWREN_RATE_LIMIT` / `--rate-limit` | `60` | Max `/v1` requests per client key per window |
| `BOXWREN_RATE_WINDOW_SECS` / `--rate-window-secs` | `60` | Rate-limit window length |

On `boxwren-server`, `/v1` keys on the `Authorization` header when present, otherwise a shared `anon` bucket.

## Self-host the long-lived process

Build `Dockerfile.server` and run it on one machine that can execute native Wasmtime. SQLite is the file at `BOXWREN_DB` (default `boxwren-server.sqlite`). Put that file on a persistent disk. The kill map is in memory, so the process does not scale out.

The process binds `$PORT` when that variable is a non-zero port, otherwise `BOXWREN_LISTEN` (default `127.0.0.1:8787`). Do not declare a Docker `VOLUME`; the host mounts the data directory. The default CORS allowlist includes localhost and `https://boxwren.rejisterjack.com`. Set `BOXWREN_CORS_ORIGINS` when another browser origin should be allowed.

`fly.toml` is one such host: one machine, autostop off, volume at `/data`, listen `0.0.0.0:8787`. The public site does not use it.

Optional: set `KEYWARDEN_URL` so each run is admitted by Keywarden before the guest starts. Boxwren still does not enforce Keywarden policy inside WASI imports.

## Residual risks

Same as [`THREAT_MODEL.md`](THREAT_MODEL.md), plus multi-tenant concerns:
Wasmtime or Boxwren bugs, Spectre-class leaks, and broad grants issued by an
account on the local server. The public API does not accept those grants.
Hosted mode does not claim cloud-hypervisor isolation.
