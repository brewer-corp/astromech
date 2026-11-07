/// <reference types="node" />
/**
 * The dev bridge: `ASTROMECH_WORLD=<dir> pnpm dev` runs the real core in the browser. This Vite
 * plugin starts the headless core on that world (core.ts `startWorld`; the binary is
 * `ASTROMECH_CORE`, default target/debug/astromech-core) when the dev server starts, and serves
 * `POST /__astromech/call` with `{cmd, args}`: the window's call, sent to the core as the shell's
 * proxy sends it (`origin: "app_window"`), answered with the core's reply lines as they come, as
 * NDJSON: each progress line, then the envelope. Only the page it serves may call it (`refusal`).
 * The page's side is src/lib/bridge.ts, which lib/transport.ts picks when the plugin defines
 * `import.meta.env.VITE_ASTROMECH_BRIDGE` (the call's path). Only the dev server applies the
 * plugin, never `pnpm build`, a preview server or the tests, and it lives outside src/, so no
 * bundle carries it.
 *
 * The core's only endpoint source is the line it prints on stdout. Its stdin is a pipe held for
 * the dev server's life: the core quits when the server closes, or when the server's process
 * goes, even killed outright (finding astromech-1uyn).
 *
 * The journey harness (app/journeys/harness.ts) passes a core it started itself instead
 * (`astromechBridge({ core })`, added to the config's plugins when it creates the server): the
 * plugin then applies with no `ASTROMECH_WORLD`, sends the page's calls to that core, and never
 * starts or quits one. Its owner quits it.
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

/** The names the dev server answers to on this machine. */
const LOOPBACK = ["localhost", "127.0.0.1", "[::1]"];

/**
 * Why the dev server must refuse `req`, or null: the bridge sends what it gets to the core as the
 * window, with the core's token, so only the page this dev server serves may call it (finding
 * astromech-igbw). Any web page the browser has open can POST here, and DNS rebinding can make
 * another name resolve to 127.0.0.1, so:
 * - the Host is this server's own, a loopback name with its `port`;
 * - the body is `application/json`, which a page elsewhere can't send without a CORS preflight
 *   this server never grants (a no-cors POST can only be text/plain or a form);
 * - the browser says the request is same-origin (`Sec-Fetch-Site`), or its Origin is this
 *   server's. A non-browser client sends the Origin itself.
 */
export function refusal(req: IncomingMessage, port: number): string | null {
  const host = req.headers.host ?? "";
  if (!LOOPBACK.some((name) => host === `${name}:${port}`)) return `not this dev server's host: ${JSON.stringify(host)}`;
  const type = (req.headers["content-type"] ?? "").split(";")[0]!.trim().toLowerCase();
  if (type !== "application/json") return `the call must be application/json, not ${JSON.stringify(type)}`;
  const site = req.headers["sec-fetch-site"];
  const origin = req.headers.origin;
  if (site !== undefined && site !== "same-origin") return `not from this dev server's page (Sec-Fetch-Site: ${site})`;
  if (site !== "same-origin" && origin !== `http://${host}`) return `not from this dev server's page (Origin: ${origin ?? "none"})`;
  return null;
}

/** Answer one call: the core's lines, each as it arrives, or a 4xx/5xx with the reason. `port`:
 * the dev server's own. */
export async function answer(core: Promise<WorldCore>, req: IncomingMessage, res: ServerResponse, port: number): Promise<void> {
  if (req.method !== "POST") {
    res.writeHead(405, { allow: "POST" }).end("POST {cmd, args}");
    return;
  }
  const refused = refusal(req, port);
  if (refused) {
    req.resume();
    res.writeHead(403, { "content-type": "text/plain" }).end(`the dev bridge refused the call: ${refused}`);
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

export interface BridgeOptions {
  /** A world core the caller started and quits (the journey harness); the plugin starts none. */
  core?: WorldCore;
}

export function astromechBridge(opts: BridgeOptions = {}): Plugin {
  const given = opts.core;
  const world = process.env.ASTROMECH_WORLD;
  return {
    name: "astromech-bridge",
    apply: (_, env) => Boolean(given ?? world) && env.command === "serve" && !env.isPreview && !process.env.VITEST,
    config: () => ({ define: { "import.meta.env.VITE_ASTROMECH_BRIDGE": JSON.stringify(CALL_PATH) } }),
    configureServer(server) {
      const log = server.config.logger;
      let core: Promise<WorldCore>;
      if (given) {
        core = Promise.resolve(given);
      } else {
        const dir = resolve(world!);
        core = startWorld(dir, { core: coreBinary() });
        core.then(
          (c) => log.info(`astromech bridge: ${dir} on the headless core at 127.0.0.1:${c.contact.port}`),
          (e: Error) => log.error(`astromech bridge: ${e.message}`),
        );
        // Unhandled otherwise: each call reports a core that didn't start.
        core.catch(() => {});
        server.httpServer?.once("close", () => void core.then((c) => c.quit()).catch(() => {}));
      }
      server.middlewares.use(CALL_PATH, (req, res) => {
        const address = server.httpServer?.address();
        void answer(core, req, res, address && typeof address === "object" ? address.port : -1);
      });
    },
  };
}
