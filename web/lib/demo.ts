import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { spawn } from "node:child_process";

import { executableBinary, runBinary } from "./cli";
import { reportsFromDemo, type DemoReport } from "./demo-report";

export const DEMO_SCENARIOS = ["secret", "loop", "network", "clean"] as const;
export type DemoScenario = (typeof DEMO_SCENARIOS)[number];
export type { DemoReport };

const CLEAN_WAT = `(module
  (func (export "run") (result i32)
    i32.const 2
    i32.const 2
    i32.add))`;

const LOOP_WAT = `(module
  (func (export "run")
    (loop br 0)))`;

const SECRET_WAT = `(module
  (import "wasi_snapshot_preview1" "path_open"
    (func $path_open (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $exit (param i32)))
  (memory (export "memory") 1)
  (data (i32.const 8) "/etc/passwd")
  (func (export "_start")
    (call $path_open
      (i32.const 3) (i32.const 0) (i32.const 8) (i32.const 11)
      (i32.const 0) (i64.const 0) (i64.const 0) (i32.const 0) (i32.const 100))
    drop
    (call $exit (i32.const 0))))`;

export function isDemoScenario(value: string): value is DemoScenario {
  return (DEMO_SCENARIOS as readonly string[]).includes(value);
}

export async function runDemo(scenario: DemoScenario, action: "run" | "kill"): Promise<DemoReport> {
  if (action === "kill") return runLoop(true);
  if (scenario === "loop") return runLoop(false);
  if (scenario === "clean") {
    return runWat(CLEAN_WAT, ["--entry", "run", "--timeout", "2s"]);
  }
  if (scenario === "secret") return runWat(SECRET_WAT, ["--timeout", "2s"]);
  const demo = await runBinary(await executableBinary(), ["demo", "--report", "json"], 12_000);
  if (!demo.stdout.trim()) {
    throw new Error(demo.stderr.trim() || "boxwren demo produced no report");
  }
  const reports = reportsFromDemo(demo.stdout);
  const network = reports.network;
  if (!network) throw new Error("boxwren demo did not include the network guest");
  return network;
}

async function runLoop(kill: boolean): Promise<DemoReport> {
  if (!kill) {
    return runWat(LOOP_WAT, ["--entry", "run", "--timeout", "200ms", "--fuel", "4611686018427387903"]);
  }
  const flags = ["--entry", "run", "--timeout", "8s", "--fuel", "4611686018427387903"];
  try {
    return await runWat(LOOP_WAT, flags, { interrupt: true });
  } catch {
    return runWat(LOOP_WAT, flags, { interrupt: true });
  }
}

async function runWat(
  wat: string,
  flags: string[],
  options?: { interrupt?: boolean },
): Promise<DemoReport> {
  const directory = await mkdtemp(join(tmpdir(), "boxwren-demo-"));
  const modulePath = join(directory, "guest.wat");
  try {
    await writeFile(modulePath, wat);
    const bin = await executableBinary();
    const args = ["run", modulePath, "--report", "json", ...flags];
    const result = options?.interrupt
      ? await runAndInterrupt(bin, args)
      : await runBinary(bin, args, 12_000);
    return parseReport(result.stdout, result.stderr);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

function parseReport(stdout: string, stderr: string): DemoReport {
  const json = stdout.trim();
  if (!json) throw new Error(stderr.trim() || "boxwren produced no report");
  try {
    return JSON.parse(json) as DemoReport;
  } catch {
    throw new Error(stderr.trim() || "boxwren produced an unreadable report");
  }
}

function runAndInterrupt(
  bin: string,
  args: string[],
): Promise<{ code: number | null; stdout: string; stderr: string }> {
  return new Promise((resolve, reject) => {
    const child = spawn(bin, args, {
      env: { ...process.env, BOXWREN_READY: "1" },
    });
    let stdout = "";
    let stderr = "";
    let sent = false;
    const interrupt = () => {
      if (sent) return;
      sent = true;
      child.kill("SIGINT");
    };
    // Cold start can take longer than the guest. Signaling before the CLI
    // prints ready kills the process before its handler exists, so there is
    // no report. The backup only covers a missed ready line.
    const backup = setTimeout(interrupt, 3_000);
    child.stdout.setEncoding("utf8");
    child.stderr.setEncoding("utf8");
    child.stdout.on("data", (chunk) => {
      stdout += chunk;
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
      if (stderr.includes("boxwren: ready")) interrupt();
    });
    child.on("error", (err) => {
      clearTimeout(backup);
      reject(err);
    });
    child.on("close", (code) => {
      clearTimeout(backup);
      resolve({ code, stdout, stderr });
    });
  });
}
