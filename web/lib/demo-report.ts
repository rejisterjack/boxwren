export type DemoReport = {
  schema_version?: number;
  status?: { kind?: string; message?: string };
  exit_code?: number | null;
  return_value?: number | null;
  duration_ms?: number;
  fuel_used?: number;
  peak_memory?: number;
  stdout?: string;
  stderr?: string;
  events?: Array<{ kind?: string; target?: string; decision?: string }>;
  warnings?: string[];
};

/** Split `boxwren demo --report json` into the named reports. */
export function reportsFromDemo(stdout: string): Record<string, DemoReport> {
  const reports: Record<string, DemoReport> = {};
  for (const chunk of stdout.split("# ").slice(1)) {
    const breakAt = chunk.indexOf("\n");
    const header = (breakAt === -1 ? chunk : chunk.slice(0, breakAt)).trim();
    const name = header.split(":")[0]?.trim();
    const json = breakAt === -1 ? "" : chunk.slice(breakAt + 1).trim();
    if (!name || !json) continue;
    reports[name] = JSON.parse(json) as DemoReport;
  }
  return reports;
}
