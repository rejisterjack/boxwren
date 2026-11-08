"use client";

export default function ThreatModelMatrix() {
  const guarantees = [
    {
      domain: "Host Address Space",
      threat: "Arbitrary memory reads/writes to host process",
      guarantee: "Strictly Isolated. Linear memory confined inside Wasmtime instance.",
      status: "enforced",
    },
    {
      domain: "Filesystem Access",
      threat: "Reading secrets (`~/.ssh`, `.env`) or overwriting system files",
      guarantee: "Deny-by-default. Path traversal (`..`) blocked at preopen boundary via cap-std.",
      status: "enforced",
    },
    {
      domain: "Socket Egress",
      threat: "Data exfiltration, botnet beacons, port scanning",
      guarantee: "Deny-by-default. Socket checks run pre-syscall. Hostname DNS disabled unless granted.",
      status: "enforced",
    },
    {
      domain: "Host CPU & RAM",
      threat: "Infinite CPU loops, memory bombs (`memory.grow`), stdio flood",
      guarantee: "Strict caps. Fuel instruction metering, memory limiters, and epoch wall deadlines.",
      status: "enforced",
    },
    {
      domain: "Runaway Threads",
      threat: "Agent generation locks calling thread",
      guarantee: "Instant kill switch. `KillHandle::kill()` halts execution from another thread.",
      status: "enforced",
    },
    {
      domain: "Microarchitectural Leaks",
      threat: "Spectre-class hardware side channels",
      guarantee: "Non-goal. SFI is software fault isolation, not constant-time hardware virtualization.",
      status: "non-goal",
    },
    {
      domain: "Multi-Tenant Cloud",
      threat: "Hostile multi-tenant kernel exploitation",
      guarantee: "Non-goal. Use an outer hypervisor (Firecracker/VM) for multi-tenant cloud defense.",
      status: "non-goal",
    },
  ];

  return (
    <div className="threat-matrix-card" aria-label="Security Threat Model and Non Goals">
      <div className="threat-matrix-header">
        <div>
          <span className="badge-kicker">Transparent Engineering</span>
          <h2 className="section-title">Threat Model &amp; Non-Goals</h2>
          <p className="section-lead">
            We reject security theater. Software Fault Isolation delivers extreme speed and safety for in-process workloads without claiming to be a multi-tenant hypervisor.
          </p>
        </div>
      </div>

      {/* Desktop Table View */}
      <div
        className="threat-table-wrap desktop-table"
        role="region"
        tabIndex={0}
        aria-label="Threat Model and Security Boundaries Table"
      >
        <table className="threat-table">
          <caption className="sr-only">Detailed breakdown of Boxwren threat vectors, enforcement guarantees, and non-goals</caption>
          <thead>
            <tr>
              <th scope="col">Attack Domain</th>
              <th scope="col">Threat Vector</th>
              <th scope="col">Boxwren Enforcement</th>
              <th scope="col" className="th-right">Classification</th>
            </tr>
          </thead>
          <tbody>
            {guarantees.map((item, idx) => (
              <tr key={idx} className={item.status === "enforced" ? "row-enforced" : "row-nongoal"}>
                <td className="threat-domain">{item.domain}</td>
                <td className="threat-threat">{item.threat}</td>
                <td className="threat-guarantee">{item.guarantee}</td>
                <td className="td-right">
                  <span
                    className={`status-pill ${
                      item.status === "enforced" ? "status-enforced" : "status-nongoal"
                    }`}
                  >
                    {item.status === "enforced" ? "Enforced by SFI" : "Non-Goal (By Design)"}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Mobile Card-Based Responsive View (< 768px) */}
      <div className="threat-mobile-cards" role="list" aria-label="Threat Model Security Cards">
        {guarantees.map((item, idx) => (
          <div
            key={idx}
            className={`threat-card ${item.status === "enforced" ? "card-enforced" : "card-nongoal"}`}
            role="listitem"
          >
            <div className="threat-card-top">
              <span className="threat-card-domain">{item.domain}</span>
              <span
                className={`status-pill ${
                  item.status === "enforced" ? "status-enforced" : "status-nongoal"
                }`}
              >
                {item.status === "enforced" ? "Enforced by SFI" : "Non-Goal (By Design)"}
              </span>
            </div>
            <div className="threat-card-body">
              <div className="threat-card-row">
                <span className="threat-card-label">Threat Vector</span>
                <span className="threat-card-val text-secondary">{item.threat}</span>
              </div>
              <div className="threat-card-row">
                <span className="threat-card-label">Enforcement Guarantee</span>
                <span className="threat-card-val text-accent font-mono">{item.guarantee}</span>
              </div>
            </div>
          </div>
        ))}
      </div>

      <div className="threat-matrix-footer">
        <span className="footer-doc-note">
          Detailed architectural specifications and security boundaries are documented in tree:
        </span>
        <div className="threat-links">
          <a
            href="https://github.com/rejisterjack/boxwren/blob/main/docs/THREAT_MODEL.md"
            target="_blank"
            rel="noreferrer"
            className="threat-doc-badge"
            aria-label="Read THREAT_MODEL.md documentation on GitHub"
          >
            THREAT_MODEL.md ↗
          </a>
          <a
            href="https://github.com/rejisterjack/boxwren/blob/main/docs/ARCHITECTURE.md"
            target="_blank"
            rel="noreferrer"
            className="threat-doc-badge"
            aria-label="Read ARCHITECTURE.md documentation on GitHub"
          >
            ARCHITECTURE.md ↗
          </a>
          <a
            href="https://github.com/rejisterjack/boxwren/blob/main/docs/HOSTED.md"
            target="_blank"
            rel="noreferrer"
            className="threat-doc-badge"
            aria-label="Read HOSTED.md documentation on GitHub"
          >
            HOSTED.md ↗
          </a>
        </div>
      </div>
    </div>
  );
}
