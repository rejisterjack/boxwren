# WASI guest interpreters

Pinned WebAssembly guests used by the optional `interpreters` feature and by
`boxwren run-py` / `boxwren run-js`. They run **inside** Boxwren under the same
deny-by-default limits. They are not host-embedded language runtimes.

| File | Source | License |
| --- | --- | --- |
| `micropython-wasi.wasm` | [simonw/micropython-wasm](https://github.com/simonw/micropython-wasm) artifact (MicroPython WASI unix port) | MIT (MicroPython) |
| `qjs-wasi.wasm` | [quickjs-ng/quickjs](https://github.com/quickjs-ng/quickjs) release `v0.12.1` | MIT |

Refresh:

```bash
# MicroPython — clone and copy artifacts/micropython-wasi.wasm
# QuickJS
curl -L -o qjs-wasi.wasm \
  https://github.com/quickjs-ng/quickjs/releases/download/v0.12.1/qjs-wasi.wasm
```
