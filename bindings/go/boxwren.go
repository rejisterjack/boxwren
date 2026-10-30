// Package boxwren embeds the Boxwren sandbox through its C ABI.
// Build the library first: cargo build -p boxwren-ffi --release
package boxwren

/*
#cgo darwin LDFLAGS: -L${SRCDIR}/../../target/release -Wl,-rpath,${SRCDIR}/../../target/release -lboxwren_ffi
#cgo linux LDFLAGS: -L${SRCDIR}/../../target/release -Wl,-rpath,${SRCDIR}/../../target/release -lboxwren_ffi -ldl -lm -lpthread
#include <stdlib.h>
#include <stdint.h>

void *boxwren_engine_new(void);
void boxwren_engine_free(void *engine);
char *boxwren_execute(void *engine, const uint8_t *wasm, size_t wasm_len, const char *request_json, char **err_out);
char *boxwren_execute_with(void *engine, const uint8_t *wasm, size_t wasm_len, const char *request_json, void *kill, char **err_out);
void *boxwren_kill_new(void *engine);
void boxwren_kill(void *kill);
void boxwren_kill_free(void *kill);
void boxwren_string_free(char *s);
*/
import "C"

import (
	"encoding/json"
	"errors"
	"unsafe"
)

// Request is the JSON body passed to the C ABI. Empty fields use Boxwren defaults.
// Dir, net, and env strings use the CLI grant syntax.
type Request struct {
	Entry       string   `json:"entry,omitempty"`
	Args        []string `json:"args,omitempty"`
	Fuel        *uint64  `json:"fuel,omitempty"`
	MemoryBytes *uint64  `json:"memory_bytes,omitempty"`
	WallMS      *uint64  `json:"wall_ms,omitempty"`
	Dirs        []string `json:"dirs,omitempty"`
	Net         []string `json:"net,omitempty"`
	Env         []string `json:"env,omitempty"`
}

// Engine is one process-wide Wasmtime engine. Reuse it across runs.
type Engine struct {
	ptr unsafe.Pointer
}

// KillHandle stops one run. Other runs on the same engine keep running.
// Create one per run, call Kill from any goroutine, then Close.
type KillHandle struct {
	ptr unsafe.Pointer
}

func New() (*Engine, error) {
	ptr := C.boxwren_engine_new()
	if ptr == nil {
		return nil, errors.New("boxwren: engine failed to start")
	}
	return &Engine{ptr: unsafe.Pointer(ptr)}, nil
}

func (e *Engine) Close() {
	if e.ptr != nil {
		C.boxwren_engine_free(e.ptr)
		e.ptr = nil
	}
}

// KillHandle returns a fresh kill switch bound to this engine.
func (e *Engine) KillHandle() *KillHandle {
	ptr := C.boxwren_kill_new(e.ptr)
	if ptr == nil {
		return nil
	}
	return &KillHandle{ptr: unsafe.Pointer(ptr)}
}

// Kill stops the run this handle was passed to. Idempotent.
func (k *KillHandle) Kill() {
	if k != nil && k.ptr != nil {
		C.boxwren_kill(k.ptr)
	}
}

// Close frees the handle. Kill after Close is a no-op.
func (k *KillHandle) Close() {
	if k != nil && k.ptr != nil {
		C.boxwren_kill_free(k.ptr)
		k.ptr = nil
	}
}

// Execute runs wasm and returns the schema-1 report JSON.
func (e *Engine) Execute(wasm []byte, request Request) (json.RawMessage, error) {
	return e.ExecuteWith(wasm, request, nil)
}

// ExecuteWith runs wasm with an external kill handle. Pass nil to run
// without one.
func (e *Engine) ExecuteWith(wasm []byte, request Request, kill *KillHandle) (json.RawMessage, error) {
	body, err := json.Marshal(request)
	if err != nil {
		return nil, err
	}
	req := C.CString(string(body))
	defer C.free(unsafe.Pointer(req))
	var wasmPtr *C.uint8_t
	if len(wasm) > 0 {
		wasmPtr = (*C.uint8_t)(unsafe.Pointer(&wasm[0]))
	}
	var killPtr unsafe.Pointer
	if kill != nil && kill.ptr != nil {
		killPtr = kill.ptr
	}
	var errPtr *C.char
	raw := C.boxwren_execute_with(e.ptr, wasmPtr, C.size_t(len(wasm)), req, killPtr, &errPtr)
	if raw == nil {
		message := "boxwren: run failed"
		if errPtr != nil {
			message = C.GoString(errPtr)
			C.boxwren_string_free(errPtr)
		}
		return nil, errors.New(message)
	}
	text := C.GoString(raw)
	C.boxwren_string_free(raw)
	return json.RawMessage(text), nil
}
