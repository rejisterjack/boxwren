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
  const rows = await db()`
    DELETE FROM api_keys
    WHERE id = ${id} AND account_id = ${account.id}
    RETURNING id
  `;
  if (!rows.length) return text("key not found", 404);
  return new Response(null, { status: 204 });
}