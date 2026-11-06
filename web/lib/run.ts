import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { executableBinary, runBinary } from "./cli";

const MAX_FUEL = 1_000_000_000;
const MAX_MEMORY = 32 * 1024 * 1024;
const MAX_MODULE = 32 * 1024 * 1024;
const MAX_WALL_MS = 2_000;

export type RunBody = {
  wasm_base64?: string;
  wasm_text?: string;
  entry?: string;
  args?: string[];
  fuel?: number;
  memory_bytes?: number;
  wall_ms?: number;
  wait?: boolean;
};

export async function executeRun(body: RunBody): Promise<
  | { ok: true; status: string; report: unknown }
  | { ok: false; status: number; message: string }
> {
  if (body.wait === false) {
    return {
      ok: false,
      status: 400,
      message: "live kill requires the local boxwren-server",
    };
  }
  const decoded = decodeModule(body);
  if (!decoded.ok) return decoded;

  const entry = body.entry?.trim() || "_start";
  const fuel = clamp(body.fuel, MAX_FUEL, MAX_FUEL);
  const memory = clamp(body.memory_bytes, MAX_MEMORY, MAX_MEMORY);
  const wallMs = clamp(body.wall_ms, MAX_WALL_MS, MAX_WALL_MS);
  const directory = await mkdtemp(join(tmpdir(), "boxwren-mod-"));
  const modulePath = join(directory, decoded.extension === "wasm" ? "guest.wasm" : "guest.wat");
  try {
    await writeFile(modulePath, decoded.bytes);
    const bin = await executableBinary();
    const args = [
      "run",
      modulePath,
      "--report",
      "json",
      "--fuel",
      String(fuel),
      "--memory",
      String(memory),
      "--timeout",
      `${wallMs}ms`,
    ];
    if (entry !== "_start") args.push("--entry", entry);
    for (const arg of body.args ?? []) args.push(arg);
    const result = await runBinary(bin, args, 12_000);
    if (result.code === 2 || !result.stdout.trim()) {
      return { ok: false, status: 400, message: result.stderr.trim() || "run rejected" };
    }
    let report: { status?: { kind?: string }; duration_ms?: number; fuel_used?: number };
    try {
      report = JSON.parse(result.stdout);
    } catch {
      return { ok: false, status: 500, message: "boxwren did not return a report" };
    }
    return { ok: true, status: report.status?.kind || "rejected", report };
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

function decodeModule(
  body: RunBody,
): { ok: true; bytes: Buffer; extension: "wat" | "wasm" } | { ok: false; status: number; message: string } {
  if (body.wasm_base64) {
    const bytes = Buffer.from(body.wasm_base64, "base64");
    if (!bytes.length) return { ok: false, status: 400, message: "wasm_base64 is empty" };
    if (bytes.length > MAX_MODULE) return { ok: false, status: 400, message: "module exceeds 32 MiB" };
    return { ok: true, bytes, extension: "wasm" };
  }
  if (typeof body.wasm_text === "string") {
    const bytes = Buffer.from(body.wasm_text);
    if (!bytes.length) return { ok: false, status: 400, message: "wasm_text is empty" };
    if (bytes.length > MAX_MODULE) return { ok: false, status: 400, message: "module exceeds 32 MiB" };
    const wasm = bytes.subarray(0, 4).toString() === "\0asm";
    return { ok: true, bytes, extension: wasm ? "wasm" : "wat" };
  }
  return { ok: false, status: 400, message: "provide wasm_base64 or wasm_text" };
}

function clamp(value: number | undefined, fallback: number, max: number): number {
  if (value == null || !Number.isFinite(value) || value <= 0) return fallback;
  return Math.min(Math.floor(value), max);
}
