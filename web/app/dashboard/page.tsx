"use client";

import { FormEvent, useCallback, useEffect, useMemo, useState } from "react";

type Account = { id: string; email: string };
type ApiKey = { id: string; name: string; prefix: string; created_at: string };
type RunRow = {
  id: string;
  status: string;
  duration_ms: number;
  fuel_used: number;
  created_at: string;
};

const API =
  process.env.NEXT_PUBLIC_BOXWREN_API_URL?.replace(/\/$/, "") ||
  "http://127.0.0.1:8787";

export default function DashboardPage() {
  const [token, setToken] = useState("");
  const [email, setEmail] = useState("you@example.com");
  const [account, setAccount] = useState<Account | null>(null);
  const [keys, setKeys] = useState<ApiKey[]>([]);
  const [newKey, setNewKey] = useState<string | null>(null);
  const [runs, setRuns] = useState<RunRow[]>([]);
  const [selected, setSelected] = useState<string | null>(null);
  const [report, setReport] = useState<string>("");
  const [error, setError] = useState<string>("");
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
    setReport(JSON.stringify(body.report, null, 2));
    await refresh();
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
    <div style={{ paddingTop: "2rem" }}>
      <h1 style={{ fontFamily: "var(--font-display)", fontSize: "2rem" }}>
        Dashboard
      </h1>
      <p className="muted">
        Create an account on the hosted API, manage keys, and inspect run
        reports. Point{" "}
        <code>NEXT_PUBLIC_BOXWREN_API_URL</code> at your{" "}
        <code>boxwren-server</code>.
      </p>

      {error ? (
        <p className="panel" style={{ color: "#e8a0a0" }}>
          {error}
        </p>
      ) : null}

      {!token ? (
        <form className="panel" onSubmit={signup} style={{ maxWidth: 420 }}>
          <h3>Create account</h3>
          <div className="form-row">
            <label htmlFor="email">Email</label>
            <input
              id="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              type="email"
              required
            />
          </div>
          <button className="btn btn-primary" type="submit">
            Sign up
          </button>
          <p className="muted" style={{ marginTop: "0.75rem", fontSize: "0.85rem" }}>
            Or paste an existing API key below.
          </p>
          <div className="form-row">
            <label htmlFor="token">API key</label>
            <input
              id="token"
              value={token}
              onChange={(e) => setToken(e.target.value)}
              placeholder="bw_..."
            />
          </div>
        </form>
      ) : (
        <div style={{ display: "grid", gap: "1.25rem" }}>
          <div className="panel">
            <h3>Account</h3>
            <p className="muted">
              {account ? `${account.email} · ${account.id}` : "Loading…"}
            </p>
            {newKey ? (
              <p>
                New key (copy now): <code>{newKey}</code>
              </p>
            ) : null}
            <button className="btn btn-ghost" type="button" onClick={createKey}>
              Create API key
            </button>
            <table className="table" style={{ marginTop: "1rem" }}>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Prefix</th>
                  <th>Created</th>
                </tr>
              </thead>
              <tbody>
                {keys.map((k) => (
                  <tr key={k.id}>
                    <td>{k.name}</td>
                    <td>
                      <code>{k.prefix}…</code>
                    </td>
                    <td>{k.created_at}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>

          <form className="panel" onSubmit={runModule}>
            <h3>Run WAT module</h3>
            <div className="form-row">
              <label htmlFor="wat">Module text</label>
              <textarea
                id="wat"
                rows={8}
                value={wat}
                onChange={(e) => setWat(e.target.value)}
              />
            </div>
            <button className="btn btn-primary" type="submit">
              Execute
            </button>
          </form>

          <div className="panel">
            <h3>Runs</h3>
            <table className="table">
              <thead>
                <tr>
                  <th>ID</th>
                  <th>Status</th>
                  <th>Fuel</th>
                  <th>ms</th>
                </tr>
              </thead>
              <tbody>
                {runs.map((r) => (
                  <tr key={r.id}>
                    <td>
                      <button
                        type="button"
                        className="btn btn-ghost"
                        style={{ padding: "0.2rem 0.4rem" }}
                        onClick={() => void openRun(r.id)}
                      >
                        {selected === r.id ? "→ " : ""}
                        {r.id.slice(0, 8)}
                      </button>
                    </td>
                    <td>{r.status}</td>
                    <td>{r.fuel_used}</td>
                    <td>{r.duration_ms}</td>
                  </tr>
                ))}
              </tbody>
            </table>
            {report ? (
              <pre style={{ marginTop: "1rem" }}>
                <code>{report}</code>
              </pre>
            ) : null}
          </div>
        </div>
      )}
    </div>
  );
}
