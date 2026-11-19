import { createHash, randomBytes, randomUUID } from "node:crypto";

import { neon } from "@neondatabase/serverless";

import { assertDatabaseUrl } from "./env";

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
  return neon(assertDatabaseUrl());
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
