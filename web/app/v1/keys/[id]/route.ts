import { db } from "../../../../lib/db";
import { gate, isAccount, requireAccount, text } from "../../../../lib/api";

export const runtime = "nodejs";

export async function DELETE(
  request: Request,
  context: { params: Promise<{ id: string }> },
) {
  const limited = await gate(request);
  if (limited) return limited;
  const account = await requireAccount(request);
  if (!isAccount(account)) return account;
  const { id } = await context.params;
  const sql = db();
  const existing = await sql`
    SELECT id FROM api_keys WHERE account_id = ${account.id}
  `;
  const owns = existing.some((row) => String(row.id) === id);
  if (!owns) return text("key not found", 404);
  if (existing.length <= 1) return text("cannot revoke the last API key", 400);
  const rows = await sql`
    DELETE FROM api_keys
    WHERE id = ${id} AND account_id = ${account.id}
    RETURNING id
  `;
  if (!rows.length) return text("key not found", 404);
  return new Response(null, { status: 204 });
}