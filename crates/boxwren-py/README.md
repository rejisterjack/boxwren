# boxwren-py

In-process Python bindings for the [Boxwren](https://github.com/rejisterjack/boxwren) WebAssembly sandbox. The sandbox is the Rust `boxwren` crate; this package wraps it with PyO3 and adds no capability kinds.

## Install

```bash
pip install boxwren-py
```

From a checkout:

```bash
pip install maturin && maturin develop
```

## Use

```python
import boxwren_py

engine = boxwren_py.Engine()
kill = engine.kill_handle()
report = engine.execute(open("guest.wasm", "rb").read(), entry="_start", kill=kill)
print(report["status"])
```

`dirs`, `net`, and `env` use the same grant strings as the CLI. Limits are `fuel`, `memory_bytes`, and `wall_ms`. The report is the schema-1 JSON document the CLI prints, parsed to dicts.

License: MIT OR Apache-2.0.
