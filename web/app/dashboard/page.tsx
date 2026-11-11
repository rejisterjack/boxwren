"use client";

import { FormEvent, useCallback, useEffect, useMemo, useState } from "react";
import Link from "next/link";

type Account = { id: string; email: string };
type ApiKey = { id: string; name: string; prefix: string; created_at: string };
type RunRow = {
  id: string;
  status: string;
  duration_ms: number;
  fuel_used: number;
  created_at: string;
};

const API = process.env.NEXT_PUBLIC_BOXWREN_API_URL?.replace(/\/$/, "") || "";

function formatDate(isoOrString: string): string {
  try {
    const d = new Date(isoOrString);
    if (isNaN(d.getTime())) return isoOrString;
    return d.toLocaleString(undefined, {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  } catch {
    return isoOrString;
  }
}

export default function DashboardPage() {
  const [token, setToken] = useState("");
  const [draftKey, setDraftKey] = useState("");
  const [email, setEmail] = useState("you@example.com");
  const [account, setAccount] = useState<Account | null>(null);
  const [keys, setKeys] = useState<ApiKey[]>([]);
  const [newKey, setNewKey] = useState<string | null>(null);
  const [runs, setRuns] = useState<RunRow[]>([]);
  const [selected, setSelected] = useState<string | null>(null);
  const [running, setRunning] = useState(false);
  const [report, setReport] = useState<string>("");
  const [error, setError] = useState<string>("");
  const [copiedKey, setCopiedKey] = useState(false);
  const [copiedReport, setCopiedReport] = useState(false);
  const [wat, setWat] = useState(`(module
  (func (export "run") (result i32)
    i32.const 2
    i32.const 2
    i32.add))`);

  const headers = useMemo(() => {
    const h: Record<string, string> = { "Content-Type": "application/json" };
    if (token) h.Authorization = `Bearer ${token}`;
    return h;
  }, [token]);

  const refresh = useCallback(async () => {
    if (!token) return;
    setError("");
    try {
      const [me, keyList, runList] = await Promise.all([
        fetch(`${API}/v1/me`, { headers }).then((r) => {
          if (!r.ok) throw new Error(`me ${r.status}`);
          return r.json();
        }),
        fetch(`${API}/v1/keys`, { headers }).then((r) => {
          if (!r.ok) throw new Error(`keys ${r.status}`);
          return r.json();
        }),
        fetch(`${API}/v1/runs`, { headers }).then((r) => {
          if (!r.ok) throw new Error(`runs ${r.status}`);
          return r.json();
        }),
      ]);
      setAccount(me);
      setKeys(keyList.keys || []);
      setRuns(runList.runs || []);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  }, [headers, token]);

  useEffect(() => {
    const saved = window.localStorage.getItem("boxwren_api_key");
    if (saved) setToken(saved);
  }, []);

  useEffect(() => {
    if (token) {
      window.localStorage.setItem("boxwren_api_key", token);
      void refresh();
    }
  }, [token, refresh]);

  function useExistingKey(event: FormEvent) {
    event.preventDefault();
    const next = draftKey.trim();
    if (!next) return;
    setError("");
    setToken(next);
  }

  async function signup(event: FormEvent) {
    event.preventDefault();
    setError("");
    const res = await fetch(`${API}/v1/accounts`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email }),
    });
    if (!res.ok) {
      setError(await res.text());
      return;
    }
    const body = await res.json();
    setAccount({ id: body.id, email: body.email });
    setToken(body.api_key);
    setNewKey(body.api_key);
  }

  async function createKey() {
    const res = await fetch(`${API}/v1/keys`, {
      method: "POST",
      headers,
      body: JSON.stringify({ name: "dashboard" }),
    });
    if (!res.ok) {
      setError(await res.text());
      return;
    }
    const body = await res.json();
    setNewKey(body.api_key);
    await refresh();
  }

  async function runModule(event: FormEvent) {
    event.preventDefault();
    setError("");
    setRunning(true);
    try {
      const res = await fetch(`${API}/v1/runs`, {
        method: "POST",
        headers,
        body: JSON.stringify({
          wasm_text: wat,
          entry: "run",
        }),
      });
      if (!res.ok) {
        setError(await res.text());
        return;
      }
      const body = await res.json();
      setSelected(body.id);
      setReport(JSON.stringify(body.report ?? {}, null, 2));
      await refresh();
    } finally {
      setRunning(false);
    }
  }

  async function openRun(id: string) {
    setSelected(id);
    const res = await fetch(`${API}/v1/runs/${id}`, { headers });
    if (!res.ok) {
      setError(await res.text());
      return;
    }
    const body = await res.json();
    setReport(JSON.stringify(body.report, null, 2));
  }

  return (
    <div style={{ padding: "2.5rem 0" }}>
      {/* Dashboard Top Header */}
      <div style={{ marginBottom: "2rem" }}>
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", flexWrap: "wrap", gap: "1rem" }}>
          <div>
            <span className="badge-kicker">Hosted Runtime Operator</span>
            <h1 className="section-title">Enclave Dashboard</h1>
            <p className="section-lead">
              Manage accounts, issue scoped API keys, and inspect deterministic WebAssembly execution reports.
            </p>
          </div>
          <Link href="/" className="btn btn-ghost btn-sm">
            ← Back to Home
          </Link>
        </div>
      </div>

      {error ? (
        <div className="panel" style={{ borderColor: "rgba(248, 113, 113, 0.4)", background: "rgba(248, 113, 113, 0.08)", marginBottom: "1.5rem" }}>
          <p style={{ color: "var(--status-kill)", fontFamily: "var(--font-mono)", fontSize: "0.85rem" }}>
            ⚠ {error}
          </p>
        </div>
      ) : null}

      {!token ? (
        <div style={{ display: "flex", flexDirection: "column", alignItems: "center" }}>
          <form className="panel" onSubmit={signup} style={{ maxWidth: 440, width: "100%" }}>
            <h3>Authenticate with Enclave</h3>
            <p className="muted" style={{ fontSize: "0.85rem", marginBottom: "1.25rem" }}>
              Sign up with your email to generate a hosted tenant key or paste an existing key.
            </p>

            <div className="form-row">
              <label htmlFor="email">Operator Email</label>
              <input
                id="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                type="email"
                required
                placeholder="operator@company.com"
              />
            </div>

            <button className="btn btn-primary" type="submit" style={{ width: "100%" }}>
              Create Account &amp; Key
            </button>

          </form>
          <form className="panel" onSubmit={useExistingKey} style={{ maxWidth: 440, width: "100%", marginTop: "1rem" }}>
            <div className="form-row">
              <label htmlFor="token">Existing API Key</label>
              <input
                id="token"
                value={draftKey}
                onChange={(e) => setDraftKey(e.target.value)}
                placeholder="bw_…"
                autoComplete="off"
                spellCheck={false}
              />
            </div>
            <button className="btn btn-secondary" type="submit" style={{ width: "100%" }}>
              Use existing key
            </button>
          </form>
        </div>
      ) : (
        <div style={{ display: "grid", gridTemplateColumns: "1fr", gap: "2rem" }}>
          {/* Top Row: Account & Keys */}
          <div className="panel">
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", flexWrap: "wrap", gap: "1rem", marginBottom: "1rem" }}>
              <div>
                <h3>Active Operator Tenant</h3>
                <p className="muted" style={{ fontFamily: "var(--font-mono)", fontSize: "0.85rem" }}>
                  {account ? `${account.email} · ID: ${account.id}` : "Connecting to telemetry..."}
                </p>
              </div>
              <div style={{ display: "flex", gap: "0.75rem", flexWrap: "wrap" }}>
                <button className="btn btn-secondary btn-sm" type="button" onClick={createKey}>
                  + Issue New API Key
                </button>
                <button
                  className="btn btn-ghost btn-sm"
                  type="button"
                  onClick={() => {
                    window.localStorage.removeItem("boxwren_api_key");
                    setToken("");
                    setDraftKey("");
                    setAccount(null);
                    setNewKey(null);
                    setRuns([]);
                    setKeys([]);
                    setSelected(null);
                    setReport("");
                    setError("");
                  }}
                >
                  Sign Out
                </button>
              </div>
            </div>

            {newKey ? (
              <div
                style={{
                  margin: "1rem 0",
                  padding: "0.85rem 1.25rem",
                  borderRadius: "var(--radius-sm)",
                  background: "var(--brand-emerald-dim)",
                  border: "1px solid var(--border-glow)",
                }}
              >
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "0.4rem" }}>
                  <span className="badge-kicker">New API Key Issued</span>
                  <button
                    type="button"
                    onClick={() => {
                      navigator.clipboard.writeText(newKey);
                      setCopiedKey(true);
                      setTimeout(() => setCopiedKey(false), 2000);
                    }}
                    className="btn btn-ghost btn-sm"
                    style={{ fontSize: "0.72rem", padding: "0.15rem 0.5rem" }}
                  >
                    {copiedKey ? "Copied ✓" : "Copy Key"}
                  </button>
                </div>
                <p style={{ fontSize: "0.82rem", color: "var(--text-secondary)", marginBottom: "0.5rem" }}>
                  Store this key safely; it will not be displayed again:
                </p>
                <code
                  style={{
                    display: "block",
                    padding: "0.5rem",
                    background: "var(--surface-void)",
                    borderRadius: "4px",
                    color: "var(--brand-emerald)",
                    fontFamily: "var(--font-mono)",
                    fontSize: "0.85rem",
                    wordBreak: "break-all",
                  }}
                >
                  {newKey}
                </code>
              </div>
            ) : null}

            <div style={{ overflowX: "auto" }}>
              <table className="table" style={{ marginTop: "1rem" }}>
                <thead>
                  <tr>
                    <th>Key Name</th>
                    <th>Prefix Identifier</th>
                    <th>Created At</th>
                  </tr>
                </thead>
                <tbody>
                  {keys.map((k) => (
                    <tr key={k.id}>
                      <td className="font-semibold text-primary">{k.name}</td>
                      <td>
                        <code style={{ color: "var(--brand-emerald)" }}>{k.prefix}…</code>
                      </td>
                      <td className="muted" style={{ fontFamily: "var(--font-mono)", fontSize: "0.82rem" }}>
                        {formatDate(k.created_at)}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          {/* Bottom Grid: Run WAT module & Runs Explorer */}
          <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(min(100%, 360px), 1fr))", gap: "2rem" }}>
            {/* Run Module Form */}
            <form className="panel" onSubmit={runModule}>
              <h3>Execute WebAssembly Module</h3>
              <p className="muted" style={{ fontSize: "0.85rem", marginBottom: "1rem" }}>
                Submit WebAssembly Text (WAT) or bytecode to the Wasmtime sandbox enforcer.
              </p>
              <div className="form-row">
                <label htmlFor="wat">WASM Text Format (WAT)</label>
                <textarea
                  id="wat"
                  rows={9}
                  value={wat}
                  onChange={(e) => setWat(e.target.value)}
                  disabled={running}
                  style={{ fontFamily: "var(--font-mono)", fontSize: "0.82rem", lineHeight: "1.5" }}
                />
              </div>
              <button className="btn btn-primary" type="submit" disabled={running} style={{ width: "100%" }}>
                {running ? "Executing inside Enclave…" : "Execute in Wasmtime →"}
              </button>
              <p className="muted" style={{ marginTop: "0.85rem", fontSize: "0.78rem", textAlign: "center" }}>
                🔒 Deny-by-default: Filesystem, network egress, and environment are locked.
              </p>
            </form>

            {/* Runs & Audit Log */}
            <div className="panel">
              <h3>Execution History</h3>
              <p className="muted" style={{ fontSize: "0.85rem", marginBottom: "1rem" }}>
                Select a run to view its deterministic Schema v1 JSON report.
              </p>
              <div style={{ maxHeight: 220, overflowY: "auto", border: "1px solid var(--border-subtle)", borderRadius: "var(--radius-sm)", marginBottom: "1rem" }}>
                <table className="table">
                  <thead>
                    <tr>
                      <th>Run ID</th>
                      <th>Status</th>
                      <th>Fuel</th>
                      <th>Duration</th>
                    </tr>
                  </thead>
                  <tbody>
                    {runs.length === 0 ? (
                      <tr>
                        <td colSpan={4} className="muted" style={{ textAlign: "center", padding: "1.5rem" }}>
                          No runs recorded yet. Execute a WAT module to begin.
                        </td>
                      </tr>
                    ) : (
                      runs.map((r) => {
                        const isSuccess = r.status === "completed_ok" || r.status === "success";
                        const isKilled = r.status === "killed";
                        return (
                          <tr key={r.id}>
                            <td>
                              <button
                                type="button"
                                className="btn btn-ghost btn-sm"
                                style={{ padding: "0.2rem 0.5rem", fontFamily: "var(--font-mono)" }}
                                onClick={() => void openRun(r.id)}
                              >
                                {selected === r.id ? "→ " : ""}
                                {r.id.slice(0, 8)}
                              </button>
                            </td>
                            <td>
                              <span
                                className={`badge ${
                                  isSuccess
                                    ? "badge-success"
                                    : isKilled
                                    ? "badge-killed"
                                    : "badge-permission_denied"
                                }`}
                              >
                                {r.status}
                              </span>
                            </td>
                            <td style={{ fontFamily: "var(--font-mono)", fontSize: "0.8rem" }}>{r.fuel_used}</td>
                            <td style={{ fontFamily: "var(--font-mono)", fontSize: "0.8rem" }}>{r.duration_ms} ms</td>
                          </tr>
                        );
                      })
                    )}
                  </tbody>
                </table>
              </div>

              {report ? (
                <div>
                  <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "0.25rem" }}>
                    <span className="badge-kicker">Schema v1 JSON Audit</span>
                    <button
                      type="button"
                      onClick={() => {
                        navigator.clipboard.writeText(report);
                        setCopiedReport(true);
                        setTimeout(() => setCopiedReport(false), 2000);
                      }}
                      className="btn btn-ghost btn-sm"
                      style={{ fontSize: "0.7rem", padding: "0.15rem 0.5rem" }}
                    >
                      {copiedReport ? "Copied ✓" : "Copy JSON"}
                    </button>
                  </div>
                  <pre
                    style={{
                      marginTop: "0.35rem",
                      padding: "0.85rem",
                      background: "var(--surface-void)",
                      border: "1px solid var(--border-subtle)",
                      borderRadius: "var(--radius-sm)",
                      maxHeight: "220px",
                      overflowY: "auto",
                      fontSize: "0.78rem",
                      color: "var(--text-secondary)",
                      fontFamily: "var(--font-mono)",
                    }}
                  >
                    <code>{report}</code>
                  </pre>
                </div>
              ) : null}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
