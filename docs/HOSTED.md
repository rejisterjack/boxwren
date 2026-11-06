# Hosted Boxwren

`boxwren-server` is an optional multi-tenant HTTP API around the same local
sandbox. It is **not** a virtual machine. Software fault isolation still applies;
an outer OS boundary (container, Firecracker, separate host) is defense in depth.

The Next.js operator UI lives at `web/app/dashboard`. Point
`NEXT_PUBLIC_BOXWREN_API_URL` at the API. OpenAPI stub: [`openapi.yaml`](openapi.yaml).

## Run locally

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
`POST /v1/runs/{id}/kill` while status is `running`.

## CORS and rate limits

| Env / flag | Default | Meaning |
| --- | --- | --- |
| `BOXWREN_CORS_ORIGINS` / `--cors-origins` | `http://127.0.0.1:3000`, `http://localhost:3000`, `https://boxwren.rejisterjack.com` | Comma-separated browser origin allowlist |
| `BOXWREN_RATE_LIMIT` / `--rate-limit` | `60` | Max `/v1` requests per client key per window |
| `BOXWREN_RATE_WINDOW_SECS` / `--rate-window-secs` | `60` | Rate-limit window length |

`/healthz` is not rate-limited. `/v1` keys on the `Authorization` header when
present, otherwise a shared `anon` bucket.

## Deploy

The hosted API is optional. The public site does not run it. Developers use the library, the CLI, and the language bindings.

Build `Dockerfile.server` and run it on one machine that can execute native Wasmtime. SQLite is the file at `BOXWREN_DB` (default `boxwren-server.sqlite`). Put that file on a persistent disk. The kill map is in memory, so the process does not scale out.

The process binds `$PORT` when that variable is a non-zero port, otherwise `BOXWREN_LISTEN` (default `127.0.0.1:8787`). Do not declare a Docker `VOLUME`; the host mounts the data directory. The default CORS allowlist includes localhost and `https://boxwren.rejisterjack.com`. Set `BOXWREN_CORS_ORIGINS` when another browser origin should be allowed.

`fly.toml` is one such host: one machine, autostop off, volume at `/data`, listen `0.0.0.0:8787`.

Point a dashboard at that API with `NEXT_PUBLIC_BOXWREN_API_URL`, then rebuild the dashboard so the value is baked into the client bundle.

Optional: set `KEYWARDEN_URL` so each run is admitted by Keywarden before the guest starts. Boxwren still does not enforce Keywarden policy inside WASI imports.

## Residual risks

Same as [`THREAT_MODEL.md`](THREAT_MODEL.md), plus multi-tenant concerns:
Wasmtime or Boxwren bugs, Spectre-class leaks, and broad grants issued by an
account. Hosted mode does not claim cloud-hypervisor isolation.
