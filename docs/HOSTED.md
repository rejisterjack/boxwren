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

Build a container (`Dockerfile.server`) and run on Fly.io or any host that can
execute native Wasmtime. Point the Next.js dashboard at the public API with
`NEXT_PUBLIC_BOXWREN_API_URL`. Set `BOXWREN_CORS_ORIGINS` to that dashboard’s
origin (and any other frontends).

Optional: set `KEYWARDEN_URL` so each run is admitted by Keywarden before the
guest starts. Boxwren still does not enforce Keywarden policy inside WASI
imports.

## Residual risks

Same as [`THREAT_MODEL.md`](THREAT_MODEL.md), plus multi-tenant concerns:
Wasmtime or Boxwren bugs, Spectre-class leaks, and broad grants issued by an
account. Hosted mode does not claim cloud-hypervisor isolation.
