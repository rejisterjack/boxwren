import { isDemoScenario, runDemo } from "../../../lib/demo";

export const runtime = "nodejs";
export const maxDuration = 15;

const WINDOW_MS = 60_000;
const LIMIT = 30;
const hits = new Map<string, { start: number; count: number }>();

export async function POST(request: Request) {
  const limited = rateLimit(clientIp(request));
  if (limited) return text(limited, 429);

  let body: { scenario?: string; action?: string };
  try {
    body = await request.json();
  } catch {
    return text("invalid JSON", 400);
  }
  const scenario = body.scenario?.trim() ?? "";
  if (!isDemoScenario(scenario)) return text("unknown scenario", 400);
  const action = body.action === "kill" ? "kill" : "run";

  try {
    const report = await runDemo(scenario, action);
    return Response.json({ scenario, action, report });
  } catch (error) {
    const message = error instanceof Error ? error.message : "demo failed";
    return text(message, 503);
  }
}

function rateLimit(ip: string): string | null {
  const now = Date.now();
  if (hits.size > 1024) {
    for (const [key, row] of hits) {
      if (now - row.start >= WINDOW_MS) hits.delete(key);
    }
  }
  const row = hits.get(ip);
  if (!row || now - row.start >= WINDOW_MS) {
    hits.set(ip, { start: now, count: 1 });
    return null;
  }
  row.count += 1;
  if (row.count > LIMIT) return "rate limit exceeded";
  return null;
}

function clientIp(request: Request): string {
  const forwarded = request.headers.get("x-forwarded-for");
  if (forwarded) return forwarded.split(",")[0]?.trim() || "anon";
  return request.headers.get("x-real-ip") || "anon";
}

function text(message: string, status: number): Response {
  return new Response(message, { status, headers: { "content-type": "text/plain" } });
}
