import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { executableBinary, runBinary } from "./cli";
import { bindModulePath, hostedCommand } from "./hosted-command";

const MAX_MODULE = 32 * 1024 * 1024;

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

  const command = hostedCommand(decoded.extension, body);
  if (!command.ok) return command;
  const directory = await mkdtemp(join(tmpdir(), "boxwren-mod-"));
  const modulePath = join(directory, command.moduleName);
  try {
    await writeFile(modulePath, decoded.bytes);
    const bin = await executableBinary();
    const args = bindModulePath(command.args, modulePath);
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

