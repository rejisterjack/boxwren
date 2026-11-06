import { chmod, mkdir } from "node:fs/promises";
import { createWriteStream } from "node:fs";
import { pipeline } from "node:stream/promises";
import { Readable } from "node:stream";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const version = process.env.BOXWREN_CLI_VERSION || "v0.2.2";
const root = join(dirname(fileURLToPath(import.meta.url)), "..", "bin");
const release = `https://github.com/rejisterjack/boxwren/releases/download/${version}`;

const assets = [
  ["boxwren-x86_64-unknown-linux-musl", "boxwren"],
  ["boxwren-aarch64-apple-darwin", "boxwren-darwin"],
];

await mkdir(root, { recursive: true });

for (const [asset, name] of assets) {
  const destination = join(root, name);
  const response = await fetch(`${release}/${asset}`);
  if (!response.ok || !response.body) {
    throw new Error(`download ${asset} failed: ${response.status}`);
  }
  await pipeline(Readable.fromWeb(response.body), createWriteStream(destination, { mode: 0o755 }));
  await chmod(destination, 0o755);
  console.log(`fetched ${version}/${asset} -> bin/${name}`);
}
