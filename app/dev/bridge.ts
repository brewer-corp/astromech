/// <reference types="node" />
/**
 * The dev bridge: `ASTROMECH_WORLD=<dir> pnpm dev` runs the real core in the browser. This Vite
 * plugin starts the headless core on that world (core.ts `startWorld`; the binary is
 * `ASTROMECH_CORE`, default target/debug/astromech-core) when the dev server starts, and serves
 * `POST /__astromech/call` with `{cmd, args}`: the window's call, sent to the core as the shell's
 * proxy sends it (`origin: "app_window"`), answered with the core's reply lines as they come, as
 * NDJSON: each progress line, then the envelope. The page's side is src/lib/bridge.ts, which
 * lib/transport.ts picks when the plugin defines `import.meta.env.VITE_ASTROMECH_BRIDGE` (the
 * call's path). Only the dev server applies the plugin, never `pnpm build`, a preview server or
 * the tests, and it lives outside src/, so no bundle carries it.
 *
 * The core's only endpoint source is the line it prints on stdout. Its stdin is a pipe held for
 * the dev server's life: the core quits when the server closes, or when the server's process
 * goes, even killed outright (finding astromech-1uyn).
 */
import type { IncomingMessage, ServerResponse } from "node:http";
import { resolve } from "node:path";
import type { Plugin } from "vite";
import { coreBinary, send, windowRequest, type WorldCore, startWorld } from "./core.ts";

/** The path the page posts its calls to. */
export const CALL_PATH = "/__astromech/call";

async function body(req: IncomingMessage): Promise<string> {
  let s = "";
  for await (const chunk of req) s += String(chunk);
  return s;
}

/** Answer one call: the core's lines, each as it arrives, or a 4xx/5xx with the reason. */
async function answer(core: Promise<WorldCore>, req: IncomingMessage, res: ServerResponse): Promise<void> {
  if (req.method !== "POST") {
    res.writeHead(405, { allow: "POST" }).end("POST {cmd, args}");
    return;
  }
  let cmd: unknown, args: unknown;
  try {
    ({ cmd, args } = JSON.parse(await body(req)) as { cmd?: unknown; args?: unknown });
  } catch (e) {
    res.writeHead(400).end(`the call isn't JSON: ${String(e)}`);
    return;
  }
  if (typeof cmd !== "string") {
    res.writeHead(400).end("the call has no cmd");
    return;
  }
  try {
    for await (const line of send((await core).contact, windowRequest(cmd, args))) {
      if (!res.headersSent) res.writeHead(200, { "content-type": "application/x-ndjson", "cache-control": "no-store" });
      res.write(`${JSON.stringify(line)}\n`);
    }
    res.end();
  } catch (e) {
    // Before the reply's first line: the core isn't running. After it: the connection dropped,
    // so the page's reader sees no envelope and fails the call with this line's error.
    const error = `the dev bridge couldn't reach the world core: ${e instanceof Error ? e.message : String(e)}`;
    if (!res.headersSent) res.writeHead(502, { "content-type": "application/x-ndjson" });
    res.end(`${JSON.stringify({ ok: false, error })}\n`);
  }
}

export function astromechBridge(): Plugin {
  const world = process.env.ASTROMECH_WORLD;
  return {
    name: "astromech-bridge",
    apply: (_, env) => Boolean(world) && env.command === "serve" && !env.isPreview && !process.env.VITEST,
    config: () => ({ define: { "import.meta.env.VITE_ASTROMECH_BRIDGE": JSON.stringify(CALL_PATH) } }),
    configureServer(server) {
      const dir = resolve(world!);
      const log = server.config.logger;
      const core = startWorld(dir, { core: coreBinary() });
      core.then(
        (c) => log.info(`astromech bridge: ${dir} on the headless core at 127.0.0.1:${c.contact.port}`),
        (e: Error) => log.error(`astromech bridge: ${e.message}`),
      );
      // Unhandled otherwise: each call reports a core that didn't start.
      core.catch(() => {});
      server.httpServer?.once("close", () => void core.then((c) => c.quit()).catch(() => {}));
      server.middlewares.use(CALL_PATH, (req, res) => void answer(core, req, res));
    },
  };
}
