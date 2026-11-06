import { db } from "../../../../lib/db";
import { gate, isAccount, requireAccount, text } from "../../../../lib/api";

export const runtime = "nodejs";

export async function GET(request: Request, context: { params: Promise<{ id: string }> }) {
  const limited = await gate(request);
  if (limited) return limited;
  const account = await requireAccount(request);
  if (!isAccount(account)) return account;
  const { id } = await context.params;
  const rows = await db()`
    SELECT id, status, report_json
    FROM runs
    WHERE account_id = ${account.id} AND id = ${id}
  `;
  const row = rows[0];
  if (!row) return text("run not found", 404);
  return Response.json({
    id: String(row.id),
    status: String(row.status),
    report: row.report_json ?? {},
  });
}
