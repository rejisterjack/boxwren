import { gate, isAccount, requireAccount, text } from "../../../../../lib/api";

export const runtime = "nodejs";

export async function POST(request: Request, _context: { params: Promise<{ id: string }> }) {
  const limited = await gate(request);
  if (limited) return limited;
  const account = await requireAccount(request);
  if (!isAccount(account)) return account;
  return text("live kill requires the local boxwren-server", 409);
}
