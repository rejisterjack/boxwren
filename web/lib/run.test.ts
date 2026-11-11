import assert from "node:assert/strict";
import test from "node:test";

import { hostedCommand } from "./hosted-command.ts";

test("guest flags cannot become host grants", () => {
  const command = hostedCommand("wat", {
    entry: "run",
    args: ["--allow-env", "DATABASE_URL", "--allow-dir", "/:/:rw"],
    fuel: 9_999_999_999,
    wall_ms: 60_000,
    memory_bytes: 999_999_999,
  });
  assert.equal(command.ok, true);
  if (!command.ok) return;
  const end = command.args.indexOf("--");
  assert.ok(end > command.args.indexOf("--timeout"));
  assert.deepEqual(command.args.slice(end + 1), [
    "--allow-env",
    "DATABASE_URL",
    "--allow-dir",
    "/:/:rw",
  ]);
  assert.ok(command.args.includes("1000000000"));
  assert.ok(command.args.includes("33554432"));
  assert.ok(command.args.includes("2000ms"));
  assert.equal(command.args.filter((arg) => arg === "--allow-env").length, 1);
});

test("args must be strings", () => {
  const command = hostedCommand("wasm", {
    args: ["ok", 1] as unknown as string[],
  });
  assert.equal(command.ok, false);
  if (command.ok) return;
  assert.match(command.message, /array of strings/);
});

test("entry that looks like a flag is rejected", () => {
  const command = hostedCommand("wat", { entry: "--allow-env" });
  assert.equal(command.ok, false);
});
