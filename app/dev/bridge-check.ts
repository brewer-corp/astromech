/// <reference types="node" />
/**
 * CI's check that the dev bridge answers with the real core (`pnpm bridge-check`, after
 * `cargo test` has built target/debug/astromech-core): the dev server with the plugin on
 * worlds/minimal, `status` posted the way the page posts it (lib/bridge.ts), and its
 * `status_line` set against the one a core of its own in the same world gives, and its config
 * path checked to be a world core's temp copy (the browser preview's fake says "Waiting for
 * Elite" too, but has no such path). A call from another origin must be refused. Exits 1 on a
 * mismatch.
 */
import { join } from "node:path";
import { createServer } from "vite";
import { ROOT, startWorld } from "./core.ts";

const world = join(ROOT, "worlds", "minimal");
process.env.ASTROMECH_WORLD = world;
const server = await createServer({ configFile: join(ROOT, "app", "vite.config.ts"), server: { port: 0, strictPort: false }, logLevel: "error" });
await server.listen();
const own = await startWorld(world, { stderr: "ignore" });
let failed = false;
try {
  const address = server.httpServer!.address();
  if (!address || typeof address === "string") throw new Error("the dev server has no port");
  const url = `http://127.0.0.1:${address.port}/__astromech/call`;
  const body = JSON.stringify({ cmd: "status", args: null });
  // Another site's no-cors POST is refused (finding astromech-igbw); the page's own call isn't.
  const foreign = await fetch(url, { method: "POST", body, headers: { "content-type": "text/plain", origin: "https://evil.example" } });
  if (foreign.status !== 403) {
    console.error(`bridge-check: a call from another origin got ${foreign.status}, not 403: ${await foreign.text()}`);
    failed = true;
  }
  const res = await fetch(url, { method: "POST", body, headers: { "content-type": "application/json", origin: `http://127.0.0.1:${address.port}` } });
  const lines = (await res.text()).trim().split("\n").map((l) => JSON.parse(l) as { ok?: boolean; value?: { status_line?: unknown; config?: unknown }; error?: string });
  const reply = lines.at(-1);
  const core = (await own.callWindow("status")) as { status_line: string };
  if (failed) {
    // reported above
  } else if (!reply?.ok || typeof reply.value?.status_line !== "string") {
    console.error(`bridge-check: the bridge answered status with ${JSON.stringify(lines)}`);
    failed = true;
  } else if (typeof reply.value.config !== "string" || !/astromech-world-[^/\\]+[/\\]world[/\\]astromech\.toml$/.test(reply.value.config)) {
    console.error(`bridge-check: the bridge's status names the config ${JSON.stringify(reply.value.config)}, not a world core's`);
    failed = true;
  } else if (reply.value.status_line !== core.status_line) {
    console.error(`bridge-check: the bridge's status_line is ${JSON.stringify(reply.value.status_line)}, the core's ${JSON.stringify(core.status_line)}`);
    failed = true;
  } else {
    console.log(`bridge-check: the bridge on worlds/minimal gives the core's status_line, ${JSON.stringify(core.status_line)}, from ${reply.value.config}`);
  }
} finally {
  await own.quit();
  await server.close();
}
process.exit(failed ? 1 : 0);
