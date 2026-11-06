import { db, newSecret, type ApiKey } from "../../../lib/db";
import { gate, isAccount, requireAccount, text } from "../../../lib/api";

export const runtime = "nodejs";

export async function GET(request: Request) {
  const limited = await gate(request);
  if (limited) return limited;
  const account = await requireAccount(request);
  if (!isAccount(account)) return account;
  const rows = await db()`
    SELECT id, name, prefix, created_at
    FROM api_keys
    WHERE account_id = ${account.id}
    ORDER BY created_at DESC
  `;
  const keys: ApiKey[] = rows.map((row) => ({
    id: String(row.id),
    name: String(row.name),
    prefix: String(row.prefix),
    created_at: String(row.created_at),
  }));
  return Response.json({ keys });
}

export async function POST(request: Request) {
  const limited = await gate(request);
  if (limited) return limited;
  const account = await requireAccount(request);
  if (!isAccount(account)) return account;
  let body: { name?: string };
  try {
    body = await request.json();
  } catch {
    return text("invalid JSON", 400);
  }
  const name = body.name?.trim() ?? "";
  if (!name || name.length > 80) return text("invalid key name", 400);
  const key = newSecret();
  const rows = await db()`
    INSERT INTO api_keys (id, account_id, name, prefix, hash)
    VALUES (${key.id}, ${account.id}, ${name}, ${key.prefix}, ${key.hash})
    RETURNING created_at
  `;
  return Response.json({
    id: key.id,
    name,
    api_key: key.raw,
    created_at: String(rows[0]?.created_at ?? ""),
  });
}
