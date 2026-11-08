import Link from "next/link";
import HeroSandbox from "./components/HeroSandbox";
import BenchmarkChart from "./components/BenchmarkChart";
import BentoFeatures from "./components/BentoFeatures";
import SafetyStack from "./components/SafetyStack";
import ThreatModelMatrix from "./components/ThreatModelMatrix";
import InstallTabs from "./components/InstallTabs";

export default function HomePage() {
  return (
    <>
      {/* 1. Command Hero Section */}
      <section className="hero-section" id="hero">
        <span className="badge-kicker">High-Performance Agent Containment</span>
        <h1 className="hero-h1">
          Contain Untrusted Agent Code at <span>1 Microsecond</span> Overhead.
        </h1>
        <p className="hero-desc">
          The in-process WebAssembly execution runtime for autonomous agents.
          Zero Docker daemons. Deny-by-default capabilities. Instant cross-thread kill switch.
        </p>

        <div className="cta-row">
          <a className="btn btn-primary" href="#install">
            Embed Boxwren in 60s →
          </a>
          <a
            className="btn btn-secondary"
            href="https://github.com/rejisterjack/boxwren"
            target="_blank"
            rel="noreferrer"
          >
            GitHub Source (★)
          </a>
          <Link className="btn btn-ghost" href="/dashboard">
            Live Web Playground
          </Link>
        </div>

        {/* Live Interactive Sandbox Enclave */}
        <div id="sandbox" style={{ width: "100%", marginTop: "1.5rem" }}>
          <HeroSandbox />
        </div>

        {/* Operational Telemetry Strip */}
        <div className="telemetry-strip" aria-label="Runtime Telemetry Stats">
          <div className="telemetry-stat">
            <span className="telemetry-num">1.05 µs</span>
            <span className="telemetry-label">Warm In-Process Exec</span>
          </div>
          <div className="telemetry-stat">
            <span className="telemetry-num">6.10 ms</span>
            <span className="telemetry-label">Cold CLI Binary Boot</span>
          </div>
          <div className="telemetry-stat">
            <span className="telemetry-num">0 MB</span>
            <span className="telemetry-label">Container Overhead</span>
          </div>
          <div className="telemetry-stat">
            <span className="telemetry-num">Schema v1</span>
            <span className="telemetry-label">Audit JSON Report</span>
          </div>
        </div>
      </section>

      {/* 2. Empirical Benchmark Telemetry Section */}
      <section id="benchmarks" style={{ padding: "2rem 0" }}>
        <BenchmarkChart />
      </section>

      {/* 3. Tactical Bento Grid Section */}
      <section id="capabilities" style={{ padding: "2rem 0" }}>
        <div style={{ textAlign: "center", marginBottom: "1rem" }}>
          <span className="badge-kicker">Architectural Primitives</span>
          <h2 className="section-title">Engineered for Hostile Generation</h2>
          <p className="section-lead" style={{ marginInline: "auto" }}>
            Software Fault Isolation built on Wasmtime. Hard limits, instant kills, and honest security boundaries.
          </p>
        </div>
        <BentoFeatures />
      </section>

      {/* 4. The Agent Safety Trinity Section */}
      <section id="architecture" style={{ padding: "2rem 0" }}>
        <SafetyStack />
      </section>

      {/* 5. Honest Threat Model & Boundaries Section */}
      <section id="threat-model" style={{ padding: "2rem 0" }}>
        <ThreatModelMatrix />
      </section>

      {/* 6. Polyglot Rapid Onboarding Section */}
      <section id="install" style={{ padding: "3rem 0", display: "flex", flexDirection: "column", alignItems: "center" }}>
        <div style={{ textAlign: "center", marginBottom: "2rem" }}>
          <span className="badge-kicker">Rapid Onboarding</span>
          <h2 className="section-title">Embed in Minutes. No Cloud Required.</h2>
          <p className="section-lead" style={{ marginInline: "auto" }}>
            Published as a Rust crate, PyPI wheels (Python 3.12 &amp; 3.13), standalone CLI binaries, and Go C-ABI headers.
          </p>
        </div>

        <InstallTabs />

        <div style={{ marginTop: "2.5rem", display: "flex", flexWrap: "wrap", justifyContent: "center", gap: "1rem" }}>
          <a
            href="https://github.com/rejisterjack/boxwren/blob/main/docs/ARCHITECTURE.md"
            target="_blank"
            rel="noreferrer"
            className="btn btn-ghost btn-sm"
          >
            Architecture Guide ↗
          </a>
          <a
            href="https://github.com/rejisterjack/boxwren/blob/main/docs/INTEGRATIONS.md"
            target="_blank"
            rel="noreferrer"
            className="btn btn-ghost btn-sm"
          >
            Framework Integrations ↗
          </a>
          <a
            href="https://github.com/rejisterjack/boxwren/blob/main/docs/HOSTED.md"
            target="_blank"
            rel="noreferrer"
            className="btn btn-ghost btn-sm"
          >
            Self-Hosted Server ↗
          </a>
          <a
            href="https://docs.rs/boxwren"
            target="_blank"
            rel="noreferrer"
            className="btn btn-ghost btn-sm"
          >
            docs.rs API Reference ↗
          </a>
        </div>
      </section>

      {/* 7. Bottom Conversion Finale */}
      <section style={{ padding: "1rem 0" }}>
        <div className="footer-cta-card">
          <span className="badge-kicker">Local-First or Cloud Ready</span>
          <h2 className="section-title" style={{ maxWidth: "24ch" }}>
            Ready to Isolate Your Agent's Code Execution?
          </h2>
          <p className="section-lead" style={{ maxWidth: "55ch" }}>
            Stop running generated code bare on your development machines or servers.
            Deploy Boxwren today in three lines of code.
          </p>
          <div className="cta-row" style={{ marginTop: "1rem" }}>
            <a className="btn btn-primary" href="#install">
              Get Started with Boxwren →
            </a>
            <Link className="btn btn-secondary" href="/dashboard">
              Explore Live Dashboard
            </Link>
          </div>
        </div>
      </section>
    </>
  );
}
