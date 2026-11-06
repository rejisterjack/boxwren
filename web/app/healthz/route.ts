export const runtime = "nodejs";

export function GET() {
  return new Response("ok", { headers: { "content-type": "text/plain" } });
}
