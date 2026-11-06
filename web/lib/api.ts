import { db, sha256, type Account } from "./db";

const LIMIT = 60;

export function text(message: string, status: number): Response {
  return new Response(message, { status, headers: { "content-type": "text/plain" } });
}

export async function gate(request: Request): Promise<Response | null> {
  const header = request.headers.get("authorization");
  const bucket = header
    ? `auth:${sha256(header).slice(0, 32)}`
    : `ip:${clientIp(request)}`;
  const sql = db();
  const rows = await sql`
    INSERT INTO rate_limits (bucket, window_start, count)
    VALUES (${bucket}, date_trunc('minute', now()), 1)
    ON CONFLICT (bucket, window_start)
    DO UPDATE SET count = rate_limits.count + 1
    RETURNING count
  `;
  const count = Number(rows[0]?.count ?? 0);
  if (count > LIMIT) return text("rate limit exceeded", 429);
  return null;
}

export async function requireAccount(request: Request): Promise<Account | Response> {
  const header = request.headers.get("authorization") ?? "";
  const match = /^Bearer\s+(\S+)/i.exec(header);
  if (!match) return text("missing Authorization", 401);
  const rows = await db()`
    SELECT a.id, a.email
    FROM accounts a
    JOIN api_keys k ON k.account_id = a.id
    WHERE k.hash = ${sha256(match[1])}
  `;
  const row = rows[0];
  if (!row) return text("invalid API key", 401);
  return { id: String(row.id), email: String(row.email) };
}

export function isAccount(value: Account | Response): value is Account {
  return !(value instanceof Response);
}

function clientIp(request: Request): string {
  const forwarded = request.headers.get("x-forwarded-for");
  if (forwarded) return forwarded.split(",")[0]?.trim() || "anon";
  return request.headers.get("x-real-ip") || "anon";
}
