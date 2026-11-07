/// <reference types="node" />
/**
 * A local control client for the headless core in a world (`astromech-core --world <dir>`,
 * crates/astromech-core/src/world.rs), in Node: the dev bridge (bridge.ts) and the journey
 * driver use it. It speaks the core's control wire (crates/astromech-api/src/control.rs): one
 * JSON request line per TCP connection to 127.0.0.1, then zero or more `{"progress": V}` lines
 * and the envelope, `{"ok": true, "value": V}` or `{"ok": false, "error": "…"}`.
 *
 * [`startWorld`] spawns the core and reads the one line it prints on stdout, its only endpoint
 * source (never `core.json`). The core quits when its stdin reaches end of file, so the client
 * holds the pipe for the core's life: if this process goes, even killed outright, the core
 * follows. Like the Rust tests' helper (crates/astromech-core/tests/support/mod.rs), the child
 * gets an empty home of its own in place of every folder variable, and no proxy variables.
 *
 * Plain Node with erasable TypeScript only, so `node` runs it as is.
 */
import { spawn } from "node:child_process";
import { mkdtempSync, readdirSync, rmSync } from "node:fs";
import { createConnection } from "node:net";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";

/** Where a world core's control API answers (the stdout line's `control`). */
export interface Contact {
  port: number;
  token: string;
}

/** One request (control.rs `Request`, less the token, which the client adds). */
export interface Request {
  cmd: string;
  args?: unknown;
  /** A text argument: the voice tester's phrase or command id, a JSON change. */
  text?: string;
  /** The page to show (`open_window`). */
  route?: string;
  /** `app_window` for the window's calls, which the core refuses `local` commands. */
  origin?: "app_window";
}

/** One line of the core's reply. */
export type Line = { progress: unknown } | { ok: true; value: unknown } | { ok: false; error: string };

/** The repository's root (this file is app/dev/core.ts). */
export const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

/** The core binary: `ASTROMECH_CORE`, else the debug build. */
export function coreBinary(): string {
  const exe = process.platform === "win32" ? "astromech-core.exe" : "astromech-core";
  return resolve(process.env.ASTROMECH_CORE ?? join(ROOT, "target", "debug", exe));
}

/** The status poll's read bound, as the shell's (control.rs `SHORT`): a core that stopped
 * answering reads as not running. Every other call waits until the core answers. */
const SHORT_MS = 10_000;

/** The window's call as the shell's proxy sends it (control.rs `window_request`): stamped
 * `app_window`, and a string `args` sent as the request's `text`. */
export function windowRequest(cmd: string, args?: unknown): Request {
  if (typeof args === "string") return { cmd, text: args, origin: "app_window" };
  return args == null ? { cmd, origin: "app_window" } : { cmd, args, origin: "app_window" };
}

/** Send `req` to the core at `to`, and give each line of its reply as it arrives; the last is
 * the envelope. A connection that closes before the envelope, or a line that isn't JSON, throws. */
export async function* send(to: Contact, req: Request): AsyncGenerator<Line> {
  const socket = createConnection({ host: "127.0.0.1", port: to.port });
  if (req.cmd === "status") socket.setTimeout(SHORT_MS, () => socket.destroy(new Error(`no reply to status in ${SHORT_MS} ms`)));
  socket.write(`${JSON.stringify({ ...req, token: to.token })}\n`);
  try {
    for await (const text of createInterface({ input: socket, crlfDelay: Infinity })) {
      const line = JSON.parse(text) as Line;
      yield line;
      if ("ok" in line) return;
    }
    throw new Error(`the core closed the connection before answering ${req.cmd}`);
  } finally {
    socket.destroy();
  }
}

/** Send `req` and wait for its value; the core's error rejects. Progress goes to `onProgress`. */
export async function call(to: Contact, req: Request, onProgress?: (p: unknown) => void): Promise<unknown> {
  for await (const line of send(to, req)) {
    if ("progress" in line) onProgress?.(line.progress);
    else if (line.ok) return line.value;
    else throw new Error(line.error);
  }
  throw new Error(`no reply to ${req.cmd}`);
}

/** The folder variables the core and its libraries could fall back to (support/mod.rs). */
const HOME_VARS = ["HOME", "XDG_DATA_HOME", "XDG_CONFIG_HOME", "XDG_CACHE_HOME", "XDG_STATE_HOME", "LOCALAPPDATA", "APPDATA", "USERPROFILE"];
/** The variables ureq takes a proxy from. */
const PROXY_VARS = ["ALL_PROXY", "all_proxy", "HTTPS_PROXY", "https_proxy", "HTTP_PROXY", "http_proxy"];

export interface WorldOptions {
  /** The core binary (default {@link coreBinary}). */
  core?: string;
  /** More variables for the core, set after the client's own. */
  env?: Record<string, string>;
  /** Where the core's stderr (its log lines) goes; default this process's. */
  stderr?: "inherit" | "ignore";
}

/** A running world core. */
export interface WorldCore {
  contact: Contact;
  /** Its app folder, under the world's temp folder. */
  appDir: string;
  /** Resolves with the exit code once the core has exited. */
  exited: Promise<number | null>;
  /** A local control client's call (no origin), as a test or driver makes it. */
  call(cmd: string, args?: unknown): Promise<unknown>;
  /** The window's call, as the shell's proxy sends it. */
  callWindow(cmd: string, args?: unknown, onProgress?: (p: unknown) => void): Promise<unknown>;
  /** `world_step` to the mark `to` (none: to the end). */
  step(to?: string): Promise<unknown>;
  /** `world_wait`: the first observation from `since` with every field of `match`. */
  wait(match: Record<string, unknown>, since?: number): Promise<unknown>;
  /** `world_observed` from `since`: `{observed, next}`. */
  observed(since?: number): Promise<{ observed: Record<string, unknown>[]; next: number }>;
  /** `quit`, then wait for the exit. Rejects if the core wrote into its home. */
  quit(): Promise<number | null>;
  /** End the core at once: its stdin closes, which quits it, and it is killed if still running. */
  kill(): void;
}

/** The empty homes of the world cores this process started, removed when it ends. */
const homes = new Set<string>();

function removeHome(home: string): void {
  homes.delete(home);
  rmSync(home, { recursive: true, force: true });
}

/** On exit, Ctrl+C or a kill, remove what's left, then end as the signal would have (the core
 * itself quits when its stdin closes with this process). */
let hooked = false;
function removeHomesOnExit(): void {
  if (hooked) return;
  hooked = true;
  process.once("exit", () => [...homes].forEach(removeHome));
  for (const sig of ["SIGINT", "SIGTERM", "SIGHUP"] as const) {
    process.once(sig, () => {
      [...homes].forEach(removeHome);
      process.kill(process.pid, sig);
    });
  }
}

/** Start the core on the world folder `world` and wait for its endpoint line. */
export async function startWorld(world: string, opts: WorldOptions = {}): Promise<WorldCore> {
  const core = opts.core ?? coreBinary();
  removeHomesOnExit();
  const home = mkdtempSync(join(tmpdir(), "astromech-world-home-"));
  homes.add(home);
  const env: Record<string, string | undefined> = { ...process.env };
  for (const k of PROXY_VARS) delete env[k];
  for (const k of HOME_VARS) env[k] = home;
  Object.assign(env, opts.env);
  const child = spawn(core, ["--world", resolve(world)], { env, stdio: ["pipe", "pipe", opts.stderr ?? "inherit"] });
  const exited = new Promise<number | null>((done) => child.once("exit", (code) => done(code)));
  const spawned = new Promise<never>((_, fail) => child.once("error", (e) => fail(new Error(`starting ${core}: ${e.message}`))));
  const lines = createInterface({ input: child.stdout!, crlfDelay: Infinity })[Symbol.asyncIterator]();
  const first = lines.next().then((r) => (r.done ? null : r.value));
  const cleanUp = () => removeHome(home);
  let text: string | null;
  try {
    text = await Promise.race([first, spawned, exited.then(() => null)]);
  } catch (e) {
    cleanUp();
    throw e;
  }
  let line: { control: Contact; app_dir: string };
  try {
    if (text === null) throw new Error(`astromech-core --world ${world} printed no endpoint line; it exited ${await exited}`);
    line = JSON.parse(text) as { control: Contact; app_dir: string };
    if (!line.control?.port || !line.control.token) throw new Error(`astromech-core --world printed ${JSON.stringify(text)}, not its endpoint line`);
  } catch (e) {
    child.kill();
    await exited;
    cleanUp();
    throw e;
  }
  // The rest of stdout is drained, so the core never blocks on a full pipe.
  void (async () => {
    for (let r = await lines.next(); !r.done; r = await lines.next());
  })();
  // The home also goes when the core exits, unless `quit` is checking it first.
  let quitting = false;
  void exited.then(() => quitting || cleanUp());
  const contact = line.control;
  const local = (cmd: string, args?: unknown) => call(contact, args == null ? { cmd } : { cmd, args });
  return {
    contact,
    appDir: line.app_dir,
    exited,
    call: local,
    callWindow: (cmd, args, onProgress) => call(contact, windowRequest(cmd, args), onProgress),
    step: (to) => local("world_step", to == null ? null : { to }),
    wait: (match, since) => local("world_wait", since == null ? { match } : { match, since }),
    observed: (since) => local("world_observed", since == null ? null : { since }) as Promise<{ observed: Record<string, unknown>[]; next: number }>,
    async quit() {
      quitting = true;
      try {
        await local("quit");
        const code = await exited;
        const left = readdirSync(home);
        if (left.length) throw new Error(`the world core wrote into its home: ${left.join(", ")}`);
        return code;
      } finally {
        cleanUp();
      }
    },
    kill() {
      child.stdin!.end();
      if (child.exitCode === null) child.kill();
    },
  };
}
