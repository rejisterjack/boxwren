"use client";

import { useState } from "react";

export default function ThreatModelMatrix() {
  const [filter, setFilter] = useState<"all" | "enforced" | "nongoal">("all");

  const guarantees = [
    {
      domain: "Host Address Space",
      threat: "Arbitrary memory read/write to calling host process",
      guarantee: "Strict Software Fault Isolation. Linear memory bounds enforced by Wasmtime instance.",
      status: "enforced",
      classification: "Enforced Boundary",
      details: "Guest pointer arithmetic cannot address memory outside its allocated WebAssembly page block.",
    },
    {
      domain: "Filesystem Access",
      threat: "Reading secrets (~/.ssh, .env) or wiping local project folders",
      guarantee: "Deny-by-default. Path traversal (../../) strictly blocked at preopen boundary via cap-std.",
      status: "enforced",
      classification: "Enforced Boundary",
      details: "Unless a directory is explicitly preopened with a capability grant, syscalls trap with ENOTCAPABLE.",
    },
    {
      domain: "Socket Egress",
      threat: "Data exfiltration, botnet beaconing, LAN port scanning",
      guarantee: "Deny-by-default. Socket operations trap pre-syscall. Outbound DNS resolution disabled.",
      status: "enforced",
      classification: "Enforced Boundary",
      details: "Raw sockets require explicit NetGrant. Zero DNS packets or outbound TCP handshakes leave the host.",
    },
    {
      domain: "Host CPU & RAM",
      threat: "Infinite loops, memory allocation bombs (memory.grow), stdio flooding",
      guarantee: "Strict instruction metering. Fuel counter limits instructions; memory limiters cap peak RAM.",
      status: "enforced",
      classification: "Enforced Boundary",
      details: "CPU burn triggers epoch interrupt; memory requests exceeding limits return -1 safely to guest.",
    },
    {
      domain: "Runaway Threads",
      threat: "Host process hangs when rogue agent script locks calling thread",
      guarantee: "Instant kill switch. Asynchronous KillHandle::kill() halts guest execution from another thread.",
      status: "enforced",
      classification: "Enforced Boundary",
      details: "Host thread regains control in microseconds without crashing the host process or disturbing peers.",
    },
    {
      domain: "Microarchitectural Leaks",
      threat: "Spectre-class and Meltdown hardware timing side channels",
      guarantee: "Explicit Non-Goal. Software Fault Isolation is not constant-time hardware virtualization.",
      status: "nongoal",
      classification: "Explicit Non-Goal",
      details: "Use separate hardware nodes or dedicated cloud instances if defending against speculative execution attacks.",
    },
    {
      domain: "Multi-Tenant Cloud VM",
      threat: "Hostile multi-tenant kernel exploitation across untrusted accounts",
      guarantee: "Explicit Non-Goal. Software Fault Isolation provides in-process speed; use an outer hypervisor for cloud hosting.",
      status: "nongoal",
      classification: "Explicit Non-Goal",
      details: "For multi-tenant SaaS hosting, pair Boxwren with Firecracker MicroVMs or Fly Machines as an outer envelope.",
    },
  ];

  const filteredItems = guarantees.filter((item) => {
    if (filter === "enforced") return item.status === "enforced";
    if (filter === "nongoal") return item.status === "nongoal";
    return true;
  });

  return (
    <div className="threat-matrix-card" aria-label="Security Threat Model and Boundaries">
      <div className="threat-matrix-header">
        <div>
          <span className="badge-kicker">Honest Security Engineering</span>
          <h2 className="section-title">Threat Model &amp; Non-Goals</h2>
          <p className="section-lead">
            We reject security theater. Software Fault Isolation provides unmatched speed and containment for in-process workloads without dishonestly pretending to be a multi-tenant hardware hypervisor.
          </p>
        </div>

        {/* Filter Pills */}
        <div className="threat-filter-group" role="tablist" aria-label="Threat Matrix Filter">
          <button
            type="button"
            role="tab"
            aria-selected={filter === "all"}
            className={`filter-btn ${filter === "all" ? "active" : ""}`}
            onClick={() => setFilter("all")}
          >
            All Vectors ({guarantees.length})
          </button>
          <button
            type="button"
            role="tab"
            aria-selected={filter === "enforced"}
            className={`filter-btn ${filter === "enforced" ? "active" : ""}`}
            onClick={() => setFilter("enforced")}
          >
            Enforced Boundaries (5)
          </button>
          <button
            type="button"
            role="tab"
            aria-selected={filter === "nongoal"}
            className={`filter-btn ${filter === "nongoal" ? "active" : ""}`}
            onClick={() => setFilter("nongoal")}
          >
            Explicit Non-Goals (2)
          </button>
        </div>
      </div>

      {/* Semantic Responsive Data Table conforming to modern-web-guidance */}
      <div
        className="threat-table-wrapper"
        tabIndex={0}
        role="region"
        aria-label="Threat Model and Security Boundaries Data Table"
      >
        <table className="threat-table">
          <caption className="sr-only">
            Detailed breakdown of Boxwren attack vectors, threat descriptions, enforcement guarantees, and non-goals
          </caption>
          <thead>
            <tr>
              <th scope="col">Attack Domain</th>
              <th scope="col">Threat Vector</th>
              <th scope="col">Boxwren Enforcement &amp; Guarantee</th>
              <th scope="col" className="th-right">Classification</th>
            </tr>
          </thead>
          <tbody>
            {filteredItems.map((item, idx) => (
              <tr
                key={idx}
                className={item.status === "enforced" ? "row-enforced" : "row-nongoal"}
              >
                <th scope="row" data-label="Attack Domain">
                  <div className="domain-cell">
                    <span className="domain-name">{item.domain}</span>
                    <span className="domain-detail">{item.details}</span>
                  </div>
                </th>
                <td data-label="Threat Vector" className="threat-cell">
                  {item.threat}
                </td>
                <td data-label="Boxwren Guarantee" className="guarantee-cell">
                  <span className="guarantee-text">{item.guarantee}</span>
                </td>
                <td data-label="Classification" className="status-cell">
                  <span
                    className={`status-badge ${
                      item.status === "enforced" ? "badge-enforced" : "badge-nongoal"
                    }`}
                  >
                    <span className="status-dot" aria-hidden="true" />
                    {item.classification}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
