# Boxwren Go bindings

Package `github.com/rejisterjack/boxwren/bindings/go` embeds the Boxwren sandbox through its C ABI in [`crates/boxwren-ffi`](../../crates/boxwren-ffi). It does not reimplement the sandbox.

## Use

```go
import boxwren "github.com/rejisterjack/boxwren/bindings/go"

engine, err := boxwren.New()
if err != nil { /* ... */ }
defer engine.Close()

kill := engine.KillHandle()
defer kill.Close()
go func() {
    time.Sleep(5 * time.Second)
    kill.Kill() // stops only this run
}()

report, err := engine.ExecuteWith(wasm, boxwren.Request{
    Entry: "run",
    Dirs:  []string{"./data:/data:ro"},
    Net:   []string{"127.0.0.1:9"},
    Env:   []string{"API_TOKEN"},
}, kill)
```

`Dirs`, `Net`, and `Env` use the same grant strings as the CLI. `Fuel`, `MemoryBytes`, and `WallMS` override the defaults. The report is the schema-1 JSON the CLI prints.

## Build

The package links `libboxwren_ffi` from `target/release` of this repository (see the `#cgo LDFLAGS` in `boxwren.go`):

```bash
cargo build -p boxwren-ffi --release
go test ./...
```

To link a released FFI library instead of a local build, download the `libboxwren_ffi-<target>` artifact from a [GitHub Release](https://github.com/rejisterjack/boxwren/releases), place it on the library path, and build with:

```bash
CGO_LDFLAGS="-lboxwren_ffi" go build ./...
```

Version tags on this module follow the repository tags (`go get github.com/rejisterjack/boxwren/bindings/go@v0.1.0`).

## License

MIT OR Apache-2.0, same as the repository.
