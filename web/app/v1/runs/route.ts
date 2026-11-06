import { randomUUID } from "node:crypto";

import { db } from "../../../lib/db";
import { gate, isAccount, requireAccount, text } from "../../../lib/api";
import { executeRun, type RunBody } from "../../../lib/run";

export const runtime = "nodejs";
export const maxDuration = 15;

export async function GET(request: Request) {
  const limited = await gate(request);
  if (limited) return limited;
  const account = await requireAccount(request);
  if (!isAccount(account)) return account;
  const rows = await db()`
    SELECT id, status, duration_ms, fuel_used, created_at
    FROM runs
    WHERE account_id = ${account.id}
    ORDER BY created_at DESC
    LIMIT 100
  `;
  return Response.json({
    runs: rows.map((row) => ({
      id: String(row.id),
      status: String(row.status),
      duration_ms: Number(row.duration_ms),
      fuel_used: Number(row.fuel_used),
      created_at: String(row.created_at),
    })),
  });
}

export async function POST(request: Request) {
  const limited = await gate(request);
  if (limited) return limited;
  const account = await requireAccount(request);
  if (!isAccount(account)) return account;
  let body: RunBody;
  try {
    body = await request.json();
  } catch {
    return text("invalid JSON", 400);
  }
  const admitted = await preflight(account.id, body.entry?.trim() || "_start");
  if (admitted) return admitted;

  const outcome = await executeRun(body);
  if (!outcome.ok) return text(outcome.message, outcome.status);
  const report = outcome.report as { duration_ms?: number; fuel_used?: number };
  const id = randomUUID();
  await db()`
    INSERT INTO runs (id, account_id, status, duration_ms, fuel_used, report_json)
    VALUES (
      ${id},
      ${account.id},
      ${outcome.status},
      ${Number(report.duration_ms ?? 0)},
      ${Number(report.fuel_used ?? 0)},
      ${JSON.stringify(outcome.report)}::jsonb
    )
  `;
  return Response.json({ id, status: outcome.status, report: outcome.report });
}

async function preflight(accountId: string, entry: string): Promise<Response | null> {
  const url = process.env.KEYWARDEN_URL;
  if (!url) return null;
  const response = await fetch(url, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ action: "boxwren.run", account_id: accountId, entry }),
  });
  if (response.ok) return null;
  const message = await response.text();
  return text(message || "keywarden denied run", 403);
}
