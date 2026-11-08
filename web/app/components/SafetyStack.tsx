export default function SafetyStack() {
  const steps = [
    {
      num: "01",
      name: "Keywarden",
      role: "Admission & Policy",
      tag: "Preflight Authorization",
      question: "Should the agent execute this tool or payload at all?",
      details:
        "Validates tool signatures, MCP capabilities, user authorization, and credential injection before execution starts.",
      url: "https://github.com/rejisterjack/keywarden",
      btnLabel: "Explore Keywarden ↗",
      isInternal: false,
    },
    {
      num: "02",
      name: "Boxwren",
      role: "Containment Runtime",
      tag: "Software Fault Isolation",
      question: "Execute the code safely with hard caps and zero trust.",
      details:
        "Runs untrusted WebAssembly in-process with deny-by-default capabilities, fuel instruction metering, and an instant kill switch.",
      url: "#install",
      btnLabel: "Embed Boxwren Now →",
      isInternal: true,
      active: true,
    },
    {
      num: "03",
      name: "Undoom",
      role: "Verification & Regression",
      tag: "Deterministic Snapshot",
      question: "Prove the fix prevented the exploit across future model updates.",
      details:
        "Snapshots execution reports and telemetry into repeatable regression tests so model drift never reintroduces vulnerabilities.",
      url: "https://github.com/rejisterjack/undoom",
      btnLabel: "Explore Undoom ↗",
      isInternal: false,
    },
  ];

  return (
    <div className="safety-stack-container" aria-label="Defense in Depth Safety Stack">
      <div className="safety-stack-intro">
        <span className="badge-kicker">Ecosystem Defense-in-Depth</span>
        <h2 className="section-title">The Three Pillars of Agent Safety</h2>
        <p className="section-lead">
          Boxwren provides the core containment tier in a comprehensive, end-to-end agent security lifecycle.
        </p>
      </div>

      {/* Responsive Lifecycle Pipeline Indicator */}
      <div className="safety-pipeline-indicator" role="list" aria-label="Security Lifecycle Stages">
        <div className="pipeline-step-item" role="listitem">
          <span className="pipeline-pill">1. Preflight Admission</span>
          <span className="pipeline-arrow desktop-arrow" aria-hidden="true">──▶</span>
          <span className="pipeline-arrow mobile-arrow" aria-hidden="true">▼</span>
        </div>
        <div className="pipeline-step-item" role="listitem">
          <span className="pipeline-pill active">2. Runtime Containment</span>
          <span className="pipeline-arrow desktop-arrow" aria-hidden="true">──▶</span>
          <span className="pipeline-arrow mobile-arrow" aria-hidden="true">▼</span>
        </div>
        <div className="pipeline-step-item" role="listitem">
          <span className="pipeline-pill">3. Regression Verification</span>
        </div>
      </div>

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
              <span className="safety-step-role">{s.role}</span>

              <p className="safety-step-question">"{s.question}"</p>
              <p className="safety-step-details">{s.details}</p>
            </div>

            <div className="safety-step-footer">
              <a
                href={s.url}
                target={s.isInternal ? undefined : "_blank"}
                rel={s.isInternal ? undefined : "noreferrer"}
                className={`btn btn-sm ${s.active ? "btn-primary" : "btn-ghost"}`}
                style={{ width: "100%" }}
                aria-label={`${s.btnLabel} (${s.name})`}
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
