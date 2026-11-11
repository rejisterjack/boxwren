"use client";

import { useState } from "react";

export default function BentoFeatures() {
  const [activeCard, setActiveCard] = useState<number | null>(null);

  const cards = [
    {
      id: 0,
      title: "Deny-by-Default Capabilities",
      badge: "Zero-Trust WASI",
      tagline: "Software Fault Isolation built on cap-std",
      description:
        "No host filesystem, environment variables, or outbound sockets exist for guest code unless explicitly granted. Path traversal (../../etc/passwd) is trapped preopen borders with zero host side-effects.",
      code: `// All capabilities fail-closed by default
let mut req = RunRequest::new(wasm_bytes);
req.limits.fuel = Some(1_000_000);
// Egress sockets & host FS remain strictly denied`,
      pill: "cap-std enforced",
    },
    {
      id: 1,
      title: "Cross-Thread Kill Switch",
      badge: "Sub-Microsecond Halt",
      tagline: "Instant emergency brake for rogue agent loops",
      description:
        "Terminate any runaway agent execution instantly from another thread without killing the host process or disturbing concurrent executions on the same engine. Safe for production multi-tenant servers.",
      code: `// Emergency cross-thread abort
let handle = run.kill_handle();
handle.kill();
// Store trapped instantly; status becomes "killed"`,
      pill: "KillHandle::kill()",
    },
    {
      id: 2,
      title: "Fuel & Epoch Resource Caps",
      badge: "Deterministic Budgeting",
      tagline: "Hard instruction counters & wall deadlines",
      description:
        "Deterministic fuel metering prevents CPU spin loops. Store memory limiters cap peak linear memory; denied growth returns -1 safely to the guest instead of crashing the host process.",
      code: `// Hard bounds on CPU and memory
limits.max_memory = 32 * 1024 * 1024;
limits.wall_timeout = Duration::from_millis(50);
// Interrupts epoch without process termination`,
      pill: "Wasmtime Fuel",
    },
    {
      id: 3,
      title: "Schema v1 Audit Reports",
      badge: "Machine-Readable",
      tagline: "Cryptographic transparency for security teams",
      description:
        "Deterministic JSON telemetry records attempted file paths, socket targets, duration, peak memory, and exact termination causes. Drop directly into Datadog, Splunk, or regression suites.",
      code: `{
  "schema": 1,
  "status": "permission_denied",
  "attempt": "path_open",
  "target": "/etc/passwd",
  "boundary": "cap-std preopen"
}`,
      pill: "Schema v1 JSON",
    },
    {
      id: 4,
      title: "Polyglot Agent Bindings",
      badge: "Native Performance",
      tagline: "Drop into LangChain, AutoGen, CrewAI, or raw Rust",
      description:
        "Engineered for polyglot AI architectures. Native Rust crate, PyO3 Python bindings (PyPI binary wheels for Python 3.12 & 3.13), standalone CLI binaries, and Go C-ABI headers.",
      code: `import boxwren

# Execute in-process with Python bindings
report = boxwren.run(
    guest_bytes, entry="run", fuel=500_000
)`,
      pill: "Python 3.12+ & Rust",
    },
    {
      id: 5,
      title: "Zero Daemon Fleet Footprint",
      badge: "Local-First Architecture",
      tagline: "No Docker daemons, no hypervisor setup, no bills",
      description:
        "Runs entirely in-process where your agent runs. Zero root privileges, zero Docker daemon crashes, and zero remote cloud sandbox latency or per-minute billing fees.",
      code: `// In-process Wasmtime SFI Engine
// Warm startup: ~1.05 µs (1,057 ns)
// Docker startup: ~185.00 ms
// 0 MB background daemon footprint`,
      pill: "Zero Fleet Overhead",
    },
  ];

  return (
    <div className="bento-grid" role="region" aria-label="Boxwren Architecture Features">
      {cards.map((card) => {
        const isHovered = activeCard === card.id;
        return (
          <div
            key={card.id}
            className={`bento-card ${isHovered ? "bento-card-active" : ""}`}
            onMouseEnter={() => setActiveCard(card.id)}
            onMouseLeave={() => setActiveCard(null)}
          >
            <div className="bento-card-header">
              <div className="bento-badges">
                <span className="badge-kicker">{card.badge}</span>
                <span className="badge-pill">{card.pill}</span>
              </div>
              <h3 className="bento-card-title">{card.title}</h3>
              <p className="bento-tagline">{card.tagline}</p>
            </div>

            <p className="bento-card-desc">{card.description}</p>

            <div className="bento-code-wrap">
              <pre className="bento-code" tabIndex={0} aria-label={`${card.title} code snippet`}>
                <code>{card.code}</code>
              </pre>
            </div>
          </div>
        );
      })}
    </div>
  );
}
