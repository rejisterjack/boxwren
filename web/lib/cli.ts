import { spawn } from "node:child_process";
import { chmod, copyFile, mkdtemp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

export function sourceBinary(): string {
  const name = process.platform === "darwin" && process.arch === "arm64" ? "boxwren-darwin" : "boxwren";
  return join(process.cwd(), "bin", name);
}

let cachedBinary: string | null = null;

export async function executableBinary(): Promise<string> {
  if (cachedBinary) return cachedBinary;
  const directory = await mkdtemp(join(tmpdir(), "boxwren-"));
  const destination = join(directory, "boxwren");
  await copyFile(sourceBinary(), destination);
  await chmod(destination, 0o755);
  cachedBinary = destination;
  return destination;
}

export function runBinary(
  bin: string,
  args: string[],
  timeoutMs: number,
): Promise<{ code: number | null; stdout: string; stderr: string }> {
  return new Promise((resolve, reject) => {
    const child = spawn(bin, args, { timeout: timeoutMs });
    let stdout = "";
    let stderr = "";
    child.stdout.setEncoding("utf8");
    child.stderr.setEncoding("utf8");
    child.stdout.on("data", (chunk) => {
      stdout += chunk;
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
    });
    child.on("error", reject);
    child.on("close", (code) => resolve({ code, stdout, stderr }));
  });
}
