import { randomUUID } from "node:crypto";

import { db, newSecret } from "../../../lib/db";
import { gate, text } from "../../../lib/api";

export const runtime = "nodejs";

export async function POST(request: Request) {
  const limited = await gate(request);
  if (limited) return limited;
  let body: { email?: string };
  try {
    body = await request.json();
  } catch {
    return text("invalid JSON", 400);
  }
  const email = body.email?.trim().toLowerCase() ?? "";
  if (!email.includes("@") || email.length > 320) return text("invalid email", 400);

  const accountId = randomUUID();
  const key = newSecret();
  const sql = db();
  try {
    await sql.transaction([
      sql`INSERT INTO accounts (id, email) VALUES (${accountId}, ${email})`,
      sql`INSERT INTO api_keys (id, account_id, name, prefix, hash)
          VALUES (${key.id}, ${accountId}, ${"default"}, ${key.prefix}, ${key.hash})`,
    ]);
  } catch (error) {
    if (isUnique(error)) return text("account already exists", 409);
    throw error;
  }
  return Response.json({ id: accountId, email, api_key: key.raw });
}

function isUnique(error: unknown): boolean {
  return typeof error === "object" && error !== null && "code" in error && error.code === "23505";
}
