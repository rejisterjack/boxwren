import React from "react";

export default function SafetyStack() {
  const steps = [
    {
      num: "01",
      name: "Keywarden",
      role: "Admission & Policy",
      tag: "Preflight Authorization",
      question: "Should the agent call this tool or execute this payload at all?",
      details:
        "Validates tool signatures, MCP capabilities, user authorization policies, and injects short-lived credential leases before execution begins. Agents never hold raw tokens.",
      url: "https://github.com/rejisterjack/keywarden",
      btnLabel: "Explore Keywarden ↗",
      isInternal: false,
      active: false,
    },
    {
      num: "02",
      name: "Boxwren",
      role: "Runtime Containment",
      tag: "Software Fault Isolation",
      question: "How do we run the generated code safely with zero host trust?",
      details:
        "Executes untrusted WebAssembly in-process with deny-by-default capabilities, fuel instruction metering, hard memory bounds, and an instant cross-thread emergency kill switch.",
      url: "#install",
      btnLabel: "Embed Boxwren Now →",
      isInternal: true,
      active: true,
    },
    {
      num: "03",
      name: "Undoom",
      role: "Regression & Verification",
      tag: "Deterministic Replay",
      question: "How do we prove future model updates won't re-introduce this exploit?",
      details:
        "Snapshots execution telemetry and containment reports into automated regression tests, ensuring model drift and prompt updates never regress safety boundaries.",
      url: "https://github.com/rejisterjack/undoom",
      btnLabel: "Explore Undoom ↗",
      isInternal: false,
      active: false,
    },
  ];

  return (
    <div className="safety-stack-container" aria-label="Defense in Depth Safety Stack">
      <div className="safety-stack-intro">
        <span className="badge-kicker">Ecosystem Defense-in-Depth</span>
        <h2 className="section-title">The Three Pillars of Agent Safety</h2>
        <p className="section-lead">
          Security is an end-to-end lifecycle, not a single checkbox. Boxwren provides the central in-process containment layer within an integrated agent governance trinity.
        </p>
      </div>

      {/* Visual Defense Pipeline Connector */}
      <div className="safety-pipeline-indicator" role="list" aria-label="Security Lifecycle Stages">
        <div className="pipeline-step-item" role="listitem">
          <span className="pipeline-pill">1. Preflight Governance</span>
          <span className="pipeline-arrow desktop-arrow" aria-hidden="true">──▶</span>
          <span className="pipeline-arrow mobile-arrow" aria-hidden="true">▼</span>
        </div>
        <div className="pipeline-step-item" role="listitem">
          <span className="pipeline-pill active">2. In-Process Containment (Boxwren)</span>
          <span className="pipeline-arrow desktop-arrow" aria-hidden="true">──▶</span>
          <span className="pipeline-arrow mobile-arrow" aria-hidden="true">▼</span>
        </div>
        <div className="pipeline-step-item" role="listitem">
          <span className="pipeline-pill">3. Regression Verification</span>
        </div>
      </div>

      {/* Three Pillars Grid */}
      <div className="safety-stack-grid">
        {steps.map((s, idx) => (
          <div
            key={idx}
            className={`safety-step-card ${s.active ? "safety-step-active" : ""}`}
          >
            <div>
              <div className="safety-step-header">
                <span className="safety-step-num">{s.num}</span>
                <span className="safety-step-tag">{s.tag}</span>
              </div>
              <h3 className="safety-step-name">{s.name}</h3>
              <p className="safety-step-role">{s.role}</p>
              <div className="safety-step-question-box">
                <span className="question-prefix">Core Focus:</span>
                <p className="safety-step-question">"{s.question}"</p>
              </div>
              <p className="safety-step-details">{s.details}</p>
            </div>

            <div className="safety-step-footer">
              <a
                href={s.url}
                target={s.isInternal ? undefined : "_blank"}
                rel={s.isInternal ? undefined : "noreferrer"}
                className={`btn btn-sm ${s.active ? "btn-primary" : "btn-ghost"}`}
                style={{ width: "100%", justifyContent: "center" }}
              >
                {s.btnLabel}
              </a>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
