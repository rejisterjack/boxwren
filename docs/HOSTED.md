# Hosted Boxwren

`boxwren-server` is an optional multi-tenant HTTP API around the same local
sandbox. It is **not** a virtual machine. Software fault isolation still applies;
an outer OS boundary (container, Firecracker, separate host) is defense in depth.

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

## Deploy

Build a container (`Dockerfile.server`) and run on Fly.io or any host that can
execute native Wasmtime. Point the Next.js dashboard at the public API with
`NEXT_PUBLIC_BOXWREN_API_URL`.

Optional: set `KEYWARDEN_URL` so each run is admitted by Keywarden before the
guest starts. Boxwren still does not enforce Keywarden policy inside WASI
imports.

## Residual risks

Same as [`THREAT_MODEL.md`](THREAT_MODEL.md), plus multi-tenant concerns:
Wasmtime or Boxwren bugs, Spectre-class leaks, and broad grants issued by an
account. Hosted mode does not claim cloud-hypervisor isolation.
