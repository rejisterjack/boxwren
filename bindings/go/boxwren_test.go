package boxwren

import (
	"encoding/json"
	"strings"
	"testing"
	"time"
)

func TestKillStopsALoop(t *testing.T) {
	engine, err := New()
	if err != nil {
		t.Fatal(err)
	}
	defer engine.Close()
	kill := engine.KillHandle()
	if kill == nil {
		t.Fatal("nil kill handle")
	}
	defer kill.Close()
	wasm := []byte(`
        (module
          (func (export "run")
            (loop br 0)))
    `)
	wall := uint64(3000)
	fuel := uint64(4611686018427387903)
	go func() {
		time.Sleep(30 * time.Millisecond)
		kill.Kill()
	}()
	started := time.Now()
	report, err := engine.ExecuteWith(wasm, Request{
		Entry: "run",
		WallMS: &wall,
		Fuel:   &fuel,
	}, kill)
	if err != nil {
		t.Fatal(err)
	}
	if elapsed := time.Since(started); elapsed > 500*time.Millisecond {
		t.Fatalf("kill took %s", elapsed)
	}
	var parsed struct {
		Status struct {
			Kind string `json:"kind"`
		} `json:"status"`
	}
	if err := json.Unmarshal(report, &parsed); err != nil {
		t.Fatal(err)
	}
	if parsed.Status.Kind != "killed" {
		t.Fatalf("status %s, report %s", parsed.Status.Kind, report)
	}
}

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
