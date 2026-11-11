"use client";

import { useState } from "react";

export default function BenchmarkChart() {
  const [activeMode, setActiveMode] = useState<"latency" | "multiplier">("latency");

  const benchmarks = [
    {
      name: "Boxwren Warm In-Process",
      latencyText: "0.001 ms (1,057 ns)",
      latencyNumeric: 0.001057,
      multiplierText: "175,000× faster",
      multiplierNumeric: 175000,
      badge: "In-Process WASM",
      badgeType: "accent",
      barWidthLatency: "8%",
      barWidthMultiplier: "100%",
      notes: "Engine kept in host memory. Microsecond invocation via PyO3 or Rust crate.",
      highlight: true,
    },
    {
      name: "Boxwren Cold CLI Binary",
      latencyText: "6.10 ms",
      latencyNumeric: 6.10,
      multiplierText: "30× faster",
      multiplierNumeric: 30,
      badge: "Single Binary",
      badgeType: "neutral",
      barWidthLatency: "38%",
      barWidthMultiplier: "45%",
      notes: "Full OS process launch, WASM compile, cap-std sandboxing, & JSON report.",
      highlight: false,
    },
    {
      name: "Docker Container (alpine:3.20)",
      latencyText: "185.00 ms",
      latencyNumeric: 185.0,
      multiplierText: "1× Baseline",
      multiplierNumeric: 1,
      badge: "Daemon Fleet",
      badgeType: "dim",
      barWidthLatency: "78%",
      barWidthMultiplier: "12%",
      notes: "Host cgroups, namespace isolation, veth bridge allocation, & daemon handshake.",
      highlight: false,
    },
    {
      name: "Cloud MicroVM Sandbox",
      latencyText: "350.00 ms+",
      latencyNumeric: 350.0,
      multiplierText: "0.5× (Latency Bottleneck)",
      multiplierNumeric: 0.5,
      badge: "Remote Cloud",
      badgeType: "faint",
      barWidthLatency: "100%",
      barWidthMultiplier: "6%",
      notes: "TLS handshake, network hop, hypervisor boot, and API pricing per execution.",
      highlight: false,
    },
  ];

  return (
    <div className="benchmark-card" aria-label="Empirical Benchmark Telemetry">
      <div className="benchmark-header">
        <div className="benchmark-header-copy">
          <span className="badge-kicker">Empirical Performance Telemetry</span>
          <h2 className="section-title">Startup &amp; Execution Latency</h2>
          <p className="section-lead">
            Autonomous agent loops invoke tools dozens of times per conversation. Container overhead turns a 2-second agent response into a 30-second delay. Boxwren eliminates the daemon.
          </p>
        </div>

        {/* Toggle Mode Control */}
        <div className="benchmark-mode-toggle" role="tablist" aria-label="Benchmark Visualization Metric">
          <button
            type="button"
            role="tab"
            aria-selected={activeMode === "latency"}
            className={`mode-btn ${activeMode === "latency" ? "active" : ""}`}
            onClick={() => setActiveMode("latency")}
          >
            Execution Latency (Log Scale)
          </button>
          <button
            type="button"
            role="tab"
            aria-selected={activeMode === "multiplier"}
            className={`mode-btn ${activeMode === "multiplier" ? "active" : ""}`}
            onClick={() => setActiveMode("multiplier")}
          >
            Throughput Multiplier
          </button>
        </div>
      </div>

      {/* Benchmark Graph Visualizer */}
      <div className="benchmark-bars-container" role="list" aria-label="Latency Comparison Bars">
        {benchmarks.map((item, idx) => (
          <div
            key={idx}
            className={`benchmark-row ${item.highlight ? "row-highlight" : ""}`}
            role="listitem"
          >
            <div className="row-meta">
              <div className="row-name-group">
                <span className="row-name">{item.name}</span>
                <span className={`row-badge badge-${item.badgeType}`}>{item.badge}</span>
              </div>
              <span className="row-stat">
                {activeMode === "latency" ? item.latencyText : item.multiplierText}
              </span>
            </div>

            {/* Visual Bar Track */}
            <div className="bar-track">
              <div
                className={`bar-fill ${item.highlight ? "fill-accent" : "fill-neutral"}`}
                style={{
                  width: activeMode === "latency" ? item.barWidthLatency : item.barWidthMultiplier,
                }}
              />
            </div>

            <p className="row-note">{item.notes}</p>
          </div>
        ))}
      </div>

      {/* Reproducible Benchmark Shell Command Box */}
      <div className="benchmark-reproduce-box">
        <div className="reproduce-header">
          <span className="reproduce-title">Reproduce Locally via Criterion &amp; Hyperfine</span>
          <span className="reproduce-badge">100% Deterministic</span>
        </div>
        <pre className="reproduce-code" tabIndex={0} aria-label="Benchmark reproduction shell command">
          <code>{`# Warm in-process Wasmtime execution benchmark
cargo bench --bench latency

# CLI binary comparison against container baseline
hyperfine 'boxwren run tests/guests/clean.wat' 'docker run --rm alpine:3.20 true'`}</code>
        </pre>
      </div>
    </div>
  );
}
