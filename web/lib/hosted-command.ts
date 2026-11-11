const MAX_FUEL = 1_000_000_000;
const MAX_MEMORY = 32 * 1024 * 1024;
const MAX_WALL_MS = 2_000;
const MAX_GUEST_ARGS = 64;
const MAX_GUEST_ARG = 4096;
/** Replaced with the temp module path after the command line is built. */
const MODULE_SLOT = "\0boxwren-module";

export type HostedRun = {
  entry?: string;
  args?: string[];
  fuel?: number;
  memory_bytes?: number;
  wall_ms?: number;
};

export function hostedCommand(
  extension: "wat" | "wasm",
  body: HostedRun,
):
  | { ok: true; moduleName: string; args: string[] }
  | { ok: false; status: number; message: string } {
  const guest = guestArgs(body.args);
  if (!guest.ok) return guest;
  const entry = body.entry?.trim() || "_start";
  if (entry.length > 256 || entry.includes("\0") || entry.startsWith("-")) {
    return { ok: false, status: 400, message: "invalid entry" };
  }
  const fuel = clamp(body.fuel, MAX_FUEL, MAX_FUEL);
  const memory = clamp(body.memory_bytes, MAX_MEMORY, MAX_MEMORY);
  const wallMs = clamp(body.wall_ms, MAX_WALL_MS, MAX_WALL_MS);
  const args = [
    "run",
    MODULE_SLOT,
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
  // Guest argv must follow `--`. Otherwise clap treats `--allow-env` and
  // `--allow-dir` in that list as host grants.
  if (guest.values.length) args.push("--", ...guest.values);
  return {
    ok: true,
    moduleName: extension === "wasm" ? "guest.wasm" : "guest.wat",
    args,
  };
}

export function bindModulePath(args: string[], modulePath: string): string[] {
  return args.map((arg) => (arg === MODULE_SLOT ? modulePath : arg));
}

function clamp(value: number | undefined, fallback: number, max: number): number {
  if (value == null || !Number.isFinite(value) || value <= 0) return fallback;
  return Math.min(Math.floor(value), max);
}

function guestArgs(
  args: unknown,
): { ok: true; values: string[] } | { ok: false; status: number; message: string } {
  if (args == null) return { ok: true, values: [] };
  if (!Array.isArray(args)) {
    return { ok: false, status: 400, message: "args must be an array of strings" };
  }
  if (args.length > MAX_GUEST_ARGS) return { ok: false, status: 400, message: "too many args" };
  const values: string[] = [];
  for (const arg of args) {
    if (typeof arg !== "string") {
      return { ok: false, status: 400, message: "args must be an array of strings" };
    }
    if (arg.length > MAX_GUEST_ARG) return { ok: false, status: 400, message: "arg is too long" };
    values.push(arg);
  }
  return { ok: true, values };
}
