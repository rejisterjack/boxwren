import { createHash, randomBytes, randomUUID } from "node:crypto";

import { neon } from "@neondatabase/serverless";

export type Account = { id: string; email: string };
export type ApiKey = { id: string; name: string; prefix: string; created_at: string };
export type RunSummary = {
  id: string;
  status: string;
  duration_ms: number;
  fuel_used: number;
  created_at: string;
};

export function db() {
  const url = process.env.DATABASE_URL;
  if (!url) {
    throw new Error("DATABASE_URL is not set");
  }
  return neon(url);
}

export function sha256(value: string): string {
  return createHash("sha256").update(value).digest("hex");
}

export function newSecret(): { id: string; raw: string; prefix: string; hash: string } {
  const raw = `bw_${randomBytes(24).toString("hex")}`;
  return {
    id: randomUUID(),
    raw,
    prefix: raw.slice(0, 10),
    hash: sha256(raw),
  };
}
