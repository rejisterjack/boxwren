"use client";

import { useState } from "react";

export default function BenchmarkChart() {
  const [activeUnit, setActiveUnit] = useState<"time" | "speedup">("time");

  const benchmarks = [
    {
      label: "Boxwren Warm In-Process",
      latency: "0.001 ms (1,057 ns)",
      speedup: "175,000× faster",
      latencyWidth: "5%",
      speedupWidth: "100%",
      colorClass: "bar-accent",
      badge: "In-Process Engine",
      note: "Engine retained in memory across agent loops (Python / Rust)",
      highlight: true,
    },
    {
      label: "Boxwren Cold CLI Process",
      latency: "6.10 ms",
      speedup: "30× faster",
      latencyWidth: "16%",
      speedupWidth: "42%",
      colorClass: "bar-medium",
      badge: "Single Binary",
      note: "Full OS process fork, compile, WASI execution, & JSON report",
      highlight: false,
    },
    {
      label: "Docker Container (alpine:3.20)",
      latency: "185.00 ms",
      speedup: "1× Baseline",
      latencyWidth: "68%",
      speedupWidth: "12%",
      colorClass: "bar-dim",
      badge: "Container Daemon",
      note: "Host Linux cgroup, namespace, and bridge network setup",
      highlight: false,
    },
    {
      label: "Hosted MicroVM (Cloud Sandbox)",
      latency: "350.00 ms+",
      speedup: "0.5× (High Latency)",
      latencyWidth: "100%",
      speedupWidth: "6%",
      colorClass: "bar-faint",
      badge: "Remote Cloud",
      note: "TLS handshake, HTTP transport roundtrip, and VM hypervisor boot",
      highlight: false,
    },
  ];

  return (
    <div className="benchmark-card" aria-label="Empirical Benchmark Comparison">
      <div className="benchmark-header">
        <div>
          <span className="badge-kicker">Empirical Telemetry</span>
          <h2 className="benchmark-title">Startup &amp; Execution Latency</h2>
          <p className="benchmark-subtitle">
            Measured on Apple M5, 10 cores, Rust 1.98. Warmup + 200 samples timed with wall-clock precision.
          </p>
        </div>

        <div className="benchmark-stat-box">
          <div className="benchmark-unit-toggle" role="group" aria-label="Toggle benchmark metric">
            <button
              type="button"
              onClick={() => setActiveUnit("time")}
              className={`unit-btn ${activeUnit === "time" ? "active" : ""}`}
              aria-pressed={activeUnit === "time"}
            >
              Latency (ms)
            </button>
            <button
              type="button"
              onClick={() => setActiveUnit("speedup")}
              className={`unit-btn ${activeUnit === "speedup" ? "active" : ""}`}
              aria-pressed={activeUnit === "speedup"}
            >
              Speedup (×)
            </button>
          </div>

          <div className="benchmark-stat-right">
            <span className="benchmark-stat-label">
              In-Process Delta {activeUnit === "time" ? "(Lower is better)" : "(Higher is better)"}
            </span>
            <span className="benchmark-stat-val">175,000×</span>
            <span className="benchmark-stat-sub">faster than local Docker</span>
          </div>
        </div>
      </div>

      <div className="benchmark-list" role="list">
        {benchmarks.map((item, idx) => {
          const width = activeUnit === "time" ? item.latencyWidth : item.speedupWidth;
          return (
            <div key={idx} className="benchmark-item" role="listitem">
              <div className="benchmark-item-top">
                <div className="item-label-group">
                  <span className={`benchmark-item-label ${item.highlight ? "highlighted" : ""}`}>
                    {item.label}
                  </span>
                  <span className="benchmark-tag">{item.badge}</span>
                </div>
                <span className={`benchmark-item-val ${item.highlight ? "text-accent" : ""}`}>
                  {activeUnit === "time" ? item.latency : item.speedup}
                </span>
              </div>

              {/* Progress Bar Track */}
              <div
                className="benchmark-bar-track"
                role="progressbar"
                aria-valuenow={parseFloat(width)}
                aria-valuemin={0}
                aria-valuemax={100}
                aria-label={`${item.label} ${activeUnit === "time" ? item.latency : item.speedup}`}
              >
                <div
                  className={`benchmark-bar-fill ${item.colorClass}`}
                  style={{ width }}
                />
              </div>

              <span className="benchmark-item-note">{item.note}</span>
            </div>
          );
        })}
      </div>

      <div className="benchmark-footer">
        <span className="text-secondary font-mono text-xs">
          Engine retained in-process: zero fork overhead, zero IPC socket serialization.
        </span>
        <a
          href="https://github.com/rejisterjack/boxwren/blob/main/benches"
          target="_blank"
          rel="noreferrer"
          className="benchmark-code-link"
          aria-label="View benchmark reproduction harnesses on GitHub"
        >
          View criterion.rs reproduction harness ↗
        </a>
      </div>
    </div>
  );
}
