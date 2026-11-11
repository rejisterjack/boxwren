"use client";

import { useState } from "react";

type TabId = "python" | "rust" | "frameworks" | "cli" | "go";

export default function InstallTabs() {
  const [activeTab, setActiveTab] = useState<TabId>("python");
  const [copiedCmd, setCopiedCmd] = useState(false);
  const [copiedCode, setCopiedCode] = useState(false);

  const tabs: Record<
    TabId,
    {
      label: string;
      cmd: string;
      badge: string;
      fileTitle: string;
      example: string;
      docsUrl: string;
      docsLabel: string;
    }
  > = {
    python: {
      label: "Python (pip)",
      cmd: "pip install boxwren-py",
      badge: "PyPI Wheels (3.12 & 3.13)",
      fileTitle: "agent_sandbox.py",
      example: `import boxwren

# Execute untrusted agent-generated code with strict fuel caps
report = boxwren.run(
    guest_wasm_bytes,
    entry="run",
    fuel=1_000_000,
    max_memory_mb=64
)

print(f"Status: {report['status']}")
if report["status"] == "permission_denied":
    print(f"Trapped attempt: {report['violation_attempted']}")`,
      docsUrl: "https://pypi.org/project/boxwren-py/",
      docsLabel: "PyPI Package Details ↗",
    },
    rust: {
      label: "Rust (Cargo)",
      cmd: "cargo add boxwren",
      badge: "crates.io v0.2.4",
      fileTitle: "main.rs",
      example: `use boxwren::{Engine, Limits, Grants};

let engine = Engine::new()?;
let mut limits = Limits::default();
limits.fuel = Some(1_000_000);

// Execute in-process at 1 µs overhead
let report = engine.execute(&wasm_bytes, "run", &limits, &Grants::default())?;
println!("Execution Status: {:?}", report.status);`,
      docsUrl: "https://docs.rs/boxwren",
      docsLabel: "docs.rs API Reference ↗",
    },
    frameworks: {
      label: "LangChain & CrewAI",
      cmd: "pip install boxwren-py langchain-core",
      badge: "Drop-in Tool Sandbox",
      fileTitle: "langchain_tool.py",
      example: `from langchain.tools import tool
import boxwren

@tool
def execute_agent_code(code_bytes: bytes) -> str:
    """Safely executes untrusted Python/WASM code inside Boxwren sandbox."""
    report = boxwren.run(code_bytes, entry="run", fuel=2_000_000)
    if report["status"] != "success":
        return f"Execution blocked: {report['status']}"
    return f"Execution success: {report.get('output', '')}"`,
      docsUrl: "https://github.com/rejisterjack/boxwren/blob/main/docs/INTEGRATIONS.md",
      docsLabel: "Framework Integration Guide ↗",
    },
    cli: {
      label: "CLI (Binary)",
      cmd: "curl -fsSL https://boxwren.rejisterjack.com/install.sh | sh",
      badge: "Standalone Binary",
      fileTitle: "terminal",
      example: `# Execute standalone guest with fuel limit and JSON telemetry
boxwren run untrusted_guest.wat \\
  --entry run \\
  --fuel 1000000 \\
  --report json`,
      docsUrl: "https://github.com/rejisterjack/boxwren/releases",
      docsLabel: "GitHub Release Binaries ↗",
    },
    go: {
      label: "Go (C ABI)",
      cmd: "go get github.com/rejisterjack/boxwren/bindings/go",
      badge: "CGO / FFI",
      fileTitle: "main.go",
      example: `import "github.com/rejisterjack/boxwren/bindings/go"

engine, err := boxwren.NewEngine()
if err != nil {
    panic(err)
}

report, err := engine.Run(wasmBytes, "run", boxwren.DefaultLimits())
fmt.Printf("Execution Status: %s\\n", report.Status)`,
      docsUrl: "https://github.com/rejisterjack/boxwren/tree/main/bindings/go",
      docsLabel: "Go C-ABI Headers ↗",
    },
  };

  const current = tabs[activeTab];

  const handleCopyCmd = () => {
    navigator.clipboard.writeText(current.cmd);
    setCopiedCmd(true);
    setTimeout(() => setCopiedCmd(false), 2000);
  };

  const handleCopyCode = () => {
    navigator.clipboard.writeText(current.example);
    setCopiedCode(true);
    setTimeout(() => setCopiedCode(false), 2000);
  };

  return (
    <div className="install-tabs-card" aria-label="Rapid Polyglot Onboarding Installation">
      {/* Ecosystem Tabs */}
      <div className="install-tab-nav" role="tablist" aria-label="Ecosystem Choices">
        {(Object.keys(tabs) as TabId[]).map((key) => {
          const tab = tabs[key];
          const isSelected = activeTab === key;
          return (
            <button
              key={key}
              role="tab"
              aria-selected={isSelected}
              className={`install-tab-btn ${isSelected ? "active" : ""}`}
              onClick={() => setActiveTab(key)}
            >
              <span className="tab-label">{tab.label}</span>
              <span className="tab-badge">{tab.badge}</span>
            </button>
          );
        })}
      </div>

      {/* Terminal Command Bar */}
      <div className="install-cmd-bar">
        <span className="terminal-prompt-char" aria-hidden="true">$</span>
        <code className="install-cmd-text">{current.cmd}</code>
        <button
          type="button"
          className="copy-btn"
          onClick={handleCopyCmd}
          aria-label="Copy installation command"
        >
          {copiedCmd ? "Copied!" : "Copy"}
        </button>
      </div>

      {/* Code Snippet Enclave */}
      <div className="install-code-enclave">
        <div className="code-enclave-header">
          <span className="enclave-filename">{current.fileTitle}</span>
          <div className="enclave-actions">
            <a
              href={current.docsUrl}
              target="_blank"
              rel="noreferrer"
              className="enclave-docs-link"
            >
              {current.docsLabel}
            </a>
            <button
              type="button"
              className="copy-btn copy-btn-sm"
              onClick={handleCopyCode}
              aria-label="Copy code example"
            >
              {copiedCode ? "Copied Snippet!" : "Copy Code"}
            </button>
          </div>
        </div>
        <pre className="install-code-block" tabIndex={0} aria-label={`${current.label} integration code`}>
          <code>{current.example}</code>
        </pre>
      </div>
    </div>
  );
}
