export default function HomePage() {
  return (
    <>
      <section className="hero">
        <p className="hero-brand">Boxwren</p>
        <h1>Run untrusted agent code without trusting it.</h1>
        <p>
          Local-first WebAssembly sandbox. Deny-by-default fuel, memory, wall
          clock, filesystem, network, and environment — plus a kill switch and a
          stable execution report.
        </p>
        <div className="cta-row">
          <a className="btn btn-primary" href="#install">
            Install
          </a>
          <a
            className="btn btn-ghost"
            href="https://github.com/rejisterjack/boxwren"
          >
            GitHub
          </a>
        </div>
      </section>

      <section id="install" className="section">
        <h2>Install</h2>
        <p>Library on crates.io. CLI on GitHub Releases. Python on PyPI.</p>
        <div className="install-grid">
          <div>
            <pre>
              <code>{`cargo add boxwren`}</code>
            </pre>
          </div>
          <div>
            <pre>
              <code>{`# CLI from a release (example: macOS arm64)
curl -L https://github.com/rejisterjack/boxwren/releases/latest/download/boxwren-aarch64-apple-darwin \\
  -o boxwren && chmod +x boxwren
./boxwren demo`}</code>
            </pre>
          </div>
          <div>
            <pre>
              <code>{`pip install boxwren-py`}</code>
            </pre>
          </div>
        </div>
      </section>

      <section id="features" className="section">
        <h2>Features</h2>
        <p>One engine per process. Each run gets its own store, fuel, memory, and deadline.</p>
        <ul className="feature-list">
          <li>
            <strong>Deny by default</strong>
            <span>
              No directories, network, or environment unless you grant them.
            </span>
          </li>
          <li>
            <strong>Hard limits</strong>
            <span>Fuel, memory, wall clock, module size, and capped stdio.</span>
          </li>
          <li>
            <strong>Kill switch</strong>
            <span>
              Stop one run from another thread. Timeout is timed_out, not killed.
            </span>
          </li>
          <li>
            <strong>Execution report</strong>
            <span>
              Schema version 1 JSON: status, fuel, peak memory, filesystem /
              network / env events.
            </span>
          </li>
          <li>
            <strong>Library + CLI + bindings</strong>
            <span>Rust crate, CLI, Python (PyO3), Go via C ABI.</span>
          </li>
        </ul>
      </section>

      <section id="docs" className="section">
        <h2>Docs</h2>
        <p>Honest boundaries. Software fault isolation is not a VM.</p>
        <div className="doc-links">
          <a href="https://github.com/rejisterjack/boxwren/blob/main/docs/ARCHITECTURE.md">
            Architecture
          </a>
          <a href="https://github.com/rejisterjack/boxwren/blob/main/docs/THREAT_MODEL.md">
            Threat model
          </a>
          <a href="https://github.com/rejisterjack/boxwren/blob/main/docs/BENCHMARKS.md">
            Benchmarks
          </a>
          <a href="https://github.com/rejisterjack/boxwren/blob/main/docs/HOSTED.md">
            Hosted
          </a>
          <a href="https://github.com/rejisterjack/boxwren/blob/main/docs/INTEGRATIONS.md">
            Integrations
          </a>
          <a href="https://docs.rs/boxwren">docs.rs</a>
        </div>
      </section>

      <section id="hosted" className="section">
        <h2>Hosted &amp; integrations</h2>
        <p>
          Optional <code>boxwren-server</code> API with accounts and API keys.
          Admit runs with Keywarden; snapshot reports in Undoom. Still SFI — not
          a hypervisor.
        </p>
        <div className="cta-row">
          <a className="btn btn-ghost" href="/dashboard">
            Open dashboard
          </a>
        </div>
      </section>
    </>
  );
}
