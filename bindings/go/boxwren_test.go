package boxwren

import (
	"encoding/json"
	"strings"
	"testing"
)

func TestExportReturnsFour(t *testing.T) {
	engine, err := New()
	if err != nil {
		t.Fatal(err)
	}
	defer engine.Close()
	wasm := []byte(`
        (module
          (func (export "run") (result i32)
            i32.const 2
            i32.const 2
            i32.add))
    `)
	report, err := engine.Execute(wasm, Request{Entry: "run"})
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(string(report), "success") {
		t.Fatalf("report %s", report)
	}
	var parsed struct {
		ReturnValue *int64 `json:"return_value"`
	}
	if err := json.Unmarshal(report, &parsed); err != nil {
		t.Fatal(err)
	}
	if parsed.ReturnValue == nil || *parsed.ReturnValue != 4 {
		t.Fatalf("return value %+v", parsed.ReturnValue)
	}
}
