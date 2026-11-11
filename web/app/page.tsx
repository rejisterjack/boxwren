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
      {/* 1. Sovereign Command Hero Section */}
      <section className="hero-section" id="hero">
        <div className="hero-intro">
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
            <Link className="btn btn-secondary" href="/dashboard">
              Live Operator Dashboard
            </Link>
            <a
              className="btn btn-ghost"
              href="https://github.com/rejisterjack/boxwren"
              target="_blank"
              rel="noreferrer"
            >
              GitHub Source (★)
            </a>
          </div>
        </div>

        {/* Live Interactive Sandbox Enclave */}
        <div id="sandbox" className="hero-sandbox-wrap">
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
      <section id="benchmarks" className="content-section">
        <BenchmarkChart />
      </section>

      {/* 3. Tactical Bento Grid Section */}
      <section id="capabilities" className="content-section">
        <div className="section-intro-centered">
          <span className="badge-kicker">Architectural Primitives</span>
          <h2 className="section-title">Engineered for Hostile Generation</h2>
          <p className="section-lead">
            Software Fault Isolation built on Wasmtime. Hard limits, instant kills, and honest security boundaries.
          </p>
        </div>
        <BentoFeatures />
      </section>

      {/* 4. The Agent Safety Trinity Section */}
      <section id="architecture" className="content-section">
        <SafetyStack />
      </section>

      {/* 5. Honest Threat Model & Boundaries Section */}
      <section id="threat-model" className="content-section">
        <ThreatModelMatrix />
      </section>

      {/* 6. Polyglot Rapid Onboarding Section */}
      <section id="install" className="content-section install-section">
        <div className="section-intro-centered">
          <span className="badge-kicker">Rapid Polyglot Onboarding</span>
          <h2 className="section-title">Embed in Minutes. Zero Cloud Required.</h2>
          <p className="section-lead">
            Published as a Rust crate, PyPI binary wheels (Python 3.12 &amp; 3.13), standalone Darwin/Linux CLI binaries, and Go C-ABI headers.
          </p>
        </div>

        <InstallTabs />

        <div className="install-quick-links">
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
            Self-Hosted Server Guide ↗
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
      <section className="content-section conversion-section">
        <div className="footer-cta-card">
          <span className="badge-kicker">Local-First or Cloud Ready</span>
          <h2 className="section-title">
            Ready to Isolate Your Agent's Code Execution?
          </h2>
          <p className="section-lead">
            Stop running generated code bare on developer machines or production backend clusters.
            Embed Boxwren today in three lines of code.
          </p>
          <div className="cta-row">
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
