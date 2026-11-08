export default function BentoFeatures() {
  const cards = [
    {
      title: "Deny-by-Default Capabilities",
      badge: "Zero-Trust WASI",
      description:
        "No filesystem, socket, or environment access unless explicitly granted. Path traversal (`..`) is strictly trapped by cap-std at preopen borders.",
      code: `// All host capabilities fail-closed by default
let mut req = RunRequest::new(wasm_bytes);
req.limits.fuel = Some(1_000_000);
// Egress sockets & host FS remain strictly denied`,
    },
    {
      title: "Cross-Thread Kill Switch",
      badge: "Instant Abort",
      description:
        "Terminate any runaway run instantly from another thread without killing the host process or disturbing concurrent executions on the same engine.",
      code: `// Asynchronously trigger emergency halt
let handle = run.kill_handle();
handle.kill();
// Traps store: status becomes "killed"`,
    },
    {
      title: "Fuel & Epoch Trapping",
      badge: "Resource Caps",
      description:
        "Deterministic fuel metering prevents CPU spin loops. Store memory limiters cap peak RAM; denied growth returns -1 safely to the guest.",
      code: `// Hard bounds on CPU and memory
limits.max_memory = 32 * 1024 * 1024;
limits.wall_timeout = Duration::from_millis(50);
// Interrupts epoch without process termination`,
    },
    {
      title: "Schema v1 Audit Reports",
      badge: "Observability",
      description:
        "Deterministic JSON telemetry records attempted file paths, socket targets, duration, peak memory, and exact termination causes for compliance.",
      code: `{
  "schema": 1,
  "status": "permission_denied",
  "attempt": "path_open",
  "target": "/etc/passwd"
}`,
    },
    {
      title: "Polyglot Language Ecosystem",
      badge: "Embed Anywhere",
      description:
        "Drop into any AI workflow. Native Rust crate, PyO3 Python bindings (PyPI wheels for 3.12/3.13), standalone CLI, and Go C-ABI headers.",
      code: `import boxwren

# Execute in-process with Python bindings
report = boxwren.run(
    guest_bytes, entry="run", fuel=500_000
)`,
    },
    {
      title: "Zero Daemon Fleet Footprint",
      badge: "Local-First",
      description:
        "No Docker daemon to manage, no hypervisor setup, and no remote network latency. Embed directly in-process where your agent runs.",
      code: `// In-process Wasmtime SFI Engine
// Warm startup: ~1.05 µs (1,057 ns)
// Docker startup: ~185.00 ms
// Zero container fleet maintenance`,
    },
  ];

  return (
    <div className="bento-grid">
      {cards.map((c, idx) => (
        <article key={idx} className="bento-card">
          <div>
            <div className="bento-top">
              <span className="bento-badge">{c.badge}</span>
              <span className="bento-dot" aria-hidden="true" />
            </div>
            <h3 className="bento-title">{c.title}</h3>
            <p className="bento-desc">{c.description}</p>
          </div>
          <pre className="bento-code">
            <code>{c.code}</code>
          </pre>
        </article>
      ))}
    </div>
  );
}
