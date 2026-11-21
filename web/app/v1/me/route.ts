import { db } from "../../../lib/db";
import { gate, isAccount, requireAccount } from "../../../lib/api";

export const runtime = "nodejs";

export async function GET(request: Request) {
  const limited = await gate(request);
  if (limited) return limited;
  const account = await requireAccount(request);
  if (!isAccount(account)) return account;
  return Response.json(account);
}

export async function DELETE(request: Request) {
  const limited = await gate(request);
  if (limited) return limited;
  const account = await requireAccount(request);
  if (!isAccount(account)) return account;
  const sql = db();
  await sql.transaction([
    sql`DELETE FROM runs WHERE account_id = ${account.id}`,
    sql`DELETE FROM api_keys WHERE account_id = ${account.id}`,
    sql`DELETE FROM accounts WHERE id = ${account.id}`,
  ]);
  return new Response(null, { status: 204 });
}
