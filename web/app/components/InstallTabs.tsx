"use client";

import { useState } from "react";

export default function InstallTabs() {
  const [activeTab, setActiveTab] = useState<"rust" | "python" | "cli" | "go">("rust");
  const [copiedCmd, setCopiedCmd] = useState(false);
  const [copiedSnippet, setCopiedSnippet] = useState(false);

  const snippets = {
    rust: {
      label: "Rust (Cargo)",
      cmd: "cargo add boxwren",
      badge: "crates.io",
      fileTitle: "main.rs",
      example: `use boxwren::{Engine, Limits, Grants};

let engine = Engine::new()?;
let mut limits = Limits::default();
limits.fuel = Some(1_000_000);

let report = engine.execute(&wasm_bytes, "run", &limits, &Grants::default())?;
println!("Status: {:?}", report.status);`,
    },
    python: {
      label: "Python (pip)",
      cmd: "pip install boxwren-py",
      badge: "PyPI 3.12 / 3.13",
      fileTitle: "agent.py",
      example: `import boxwren

# Run untrusted guest code with fuel cap
report = boxwren.run(
    guest_wasm_bytes,
    entry="run",
    fuel=1_000_000
)
print(f"Status: {report['status']}")`,
    },
    cli: {
      label: "CLI (Binary)",
      cmd: "curl -fsSL https://boxwren.rejisterjack.com/install.sh | sh",
      badge: "Darwin / Linux",
      fileTitle: "terminal",
      example: `# Execute standalone guest with fuel limit and JSON telemetry
boxwren run untrusted_guest.wat \\
  --entry run \\
  --fuel 1000000 \\
  --report json`,
    },
    go: {
      label: "Go (C ABI)",
      cmd: "go get github.com/rejisterjack/boxwren/bindings/go",
      badge: "CGO / FFI",
      fileTitle: "main.go",
      example: `import "github.com/rejisterjack/boxwren/bindings/go"

engine, err := boxwren.NewEngine()
report, err := engine.Run(wasmBytes, "run", boxwren.DefaultLimits())
fmt.Printf("Status: %s\\n", report.Status)`,
    },
  };

  const current = snippets[activeTab];

  const handleCopyCmd = () => {
    navigator.clipboard.writeText(current.cmd);
    setCopiedCmd(true);
    setTimeout(() => setCopiedCmd(false), 2000);
  };

  const handleCopySnippet = () => {
    navigator.clipboard.writeText(current.example);
    setCopiedSnippet(true);
    setTimeout(() => setCopiedSnippet(false), 2000);
  };

  return (
    <div className="install-tabs-widget" aria-label="Installation and Quickstart Codes">
      <div className="install-tabs-nav" role="tablist" aria-label="Installation Methods">
        {(Object.keys(snippets) as Array<keyof typeof snippets>).map((key) => (
          <button
            key={key}
            role="tab"
            aria-selected={activeTab === key}
            onClick={() => {
              setActiveTab(key);
              setCopiedCmd(false);
              setCopiedSnippet(false);
            }}
            className={`install-tab-btn ${activeTab === key ? "active" : ""}`}
          >
            {snippets[key].label}
          </button>
        ))}
      </div>

      {/* Main Terminal Command Box */}
      <div className="install-tab-content">
        <div className="cmd-line-group">
          <span className="terminal-prompt" aria-hidden="true">$</span>
          <code className="install-cmd">{current.cmd}</code>
        </div>
        <div className="install-actions">
          <span className="install-badge">{current.badge}</span>
          <button
            type="button"
            onClick={handleCopyCmd}
            className="btn-copy"
            aria-label={`Copy command: ${current.cmd}`}
          >
            {copiedCmd ? "Copied ✓" : "Copy Command"}
          </button>
        </div>
      </div>

      {/* Code Snippet Preview */}
      <div className="install-snippet-wrap">
        <div className="snippet-header">
          <span className="snippet-filename">{current.fileTitle}</span>
          <div className="snippet-header-actions">
            <span className="snippet-quickstart-label">Three-Line Embed</span>
            <button
              type="button"
              onClick={handleCopySnippet}
              className="btn-copy-snippet"
              aria-label={`Copy ${current.fileTitle} code snippet`}
            >
              {copiedSnippet ? "Copied ✓" : "Copy Code"}
            </button>
          </div>
        </div>
        <pre className="snippet-code">
          <code>{current.example}</code>
        </pre>
      </div>
    </div>
  );
}
