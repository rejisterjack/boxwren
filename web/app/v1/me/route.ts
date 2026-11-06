import { gate, isAccount, requireAccount } from "../../../lib/api";

export const runtime = "nodejs";

export async function GET(request: Request) {
  const limited = await gate(request);
  if (limited) return limited;
  const account = await requireAccount(request);
  if (!isAccount(account)) return account;
  return Response.json(account);
}
