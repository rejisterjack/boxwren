import assert from "node:assert/strict";
import test from "node:test";

import { reportsFromDemo } from "./demo-report.ts";

test("splits a boxwren demo transcript into named reports", () => {
  const stdout = `# secret: blocked
{"schema_version":1,"status":{"kind":"success"},"events":[{"kind":"filesystem","target":"/etc/passwd","decision":"denied"}]}
# network: blocked
{"schema_version":1,"status":{"kind":"success"},"events":[{"kind":"network","target":"127.0.0.1:1","decision":"denied"}]}
`;
  const reports = reportsFromDemo(stdout);
  assert.equal(reports.secret?.events?.[0]?.decision, "denied");
  assert.equal(reports.network?.events?.[0]?.target, "127.0.0.1:1");
});
