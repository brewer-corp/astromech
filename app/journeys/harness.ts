/**
 * The journey harness (CONTRIBUTING.md, Tests: journeys). A journey is a user outcome that
 * crosses the page, the real core and a boundary, run by `node --test 'journeys/*.journey.ts'`
 * in app/. For each journey the harness starts:
 *
 * - one world core on a committed world (`startWorld`, app/dev/core.ts);
 * - a Vite dev server on app/vite.config.ts whose dev bridge sends the page's calls to that same
 *   core (`astromechBridge({ core })`, app/dev/bridge.ts): the server never starts or quits one;
 * - a fresh Chromium context at the app window's 1200×800, its page's clock installed at the
 *   world header's `start` and paused there, so the page's timers (polls, debounces, a toast's
 *   dismissal) fire only when the journey calls `runFor`.
 *
 * It ends each journey by closing the page and the server, then quitting the core, which must
 * exit 0 with its home empty and its temp world (`astromech-world-*`) gone. A page that loads a
 * second time (Vite re-optimising its dependencies mid-journey) fails the journey, and so does
 * an error the page throws. On a failure the journey's Playwright trace, a screenshot and the
 * core's log are kept under journeys/results/<journey>/.
 *
 * Determinism: no sleeps, no wall-clock reads and no retries. A journey waits on the core's
 * observations (`wait`, `world_wait` with its own hang guard) or on the page (locator waits, or
 * `waitForFunction` with `polling: "mutation"`); timeouts only guard against a hang.
 */
import { closeSync, copyFileSync, existsSync, globSync, mkdirSync, openSync, readFileSync, rmdirSync, rmSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { after, test } from "node:test";
import { chromium, type Browser, type BrowserContext, type Page, type Request } from "playwright";
import { createServer, type ViteDevServer } from "vite";
import { astromechBridge, CALL_PATH } from "../dev/bridge.ts";
import { ROOT, startWorld, type WorldCore } from "../dev/core.ts";

/** app/. */
export const APP = join(ROOT, "app");
/** Where a failed journey's trace, screenshot and core log go (CI uploads it). */
export const RESULTS = join(APP, "journeys", "results");
/** The pattern the suite runs (`node --test 'journeys/*.journey.ts'` in app/). */
export const PATTERN = "journeys/*.journey.ts";

/** The app window's opening size (app/src-tauri/src/window.rs). */
const WINDOW = { width: 1200, height: 800 };
/** Hang guards, deciding nothing: a page wait, and a whole journey. */
const PAGE_GUARD_MS = 15_000;
const JOURNEY_GUARD_MS = 90_000;

/** What a journey gets. */
export interface Journey {
  /** The page, on the dev server, its clock paused at the world's start. */
  page: Page;
  /** The journey's world core: `step` (`world_step`), `wait` (`world_wait`), `observed`
   * (`world_observed`) and the window's own calls (`callWindow`). */
  core: Pick<WorldCore, "step" | "wait" | "observed" | "callWindow">;
  /** The world's temp folder (`astromech-world-*`), which the core removes when it quits. */
  world: string;
  /** Past onboarding (the window's `onboarding_done`, what Skip setup sends), then load the
   * page at `#/<route>`. Call it once: the page loads once per journey. */
  open(route: string): Promise<void>;
  /** Move the page's clock on by `ms`, firing the timers due. */
  runFor(ms: number): Promise<void>;
  /** The window's calls of `cmd` posted to the dev bridge so far, as the page sent them. */
  calls(cmd: string): Request[];
  /** Resolves once every request the page made before it has reached the harness. */
  settled(): Promise<void>;
}

export interface JourneyOptions {
  /** The world, under the repository's worlds/ (`journeys/lookup-retry`). */
  world: string;
}

let browser: Promise<Browser> | null = null;
after(async () => {
  if (browser) await (await browser).close();
});

/** The world header's `start` (world.jsonl's first line), where the page's clock starts. */
export function worldStart(world: string): number {
  const header = JSON.parse(readFileSync(join(world, "world.jsonl"), "utf8").split("\n")[0]!) as { start?: string };
  const start = header.start ? Date.parse(header.start) : NaN;
  if (Number.isNaN(start)) throw new Error(`${world}/world.jsonl's header has no start time for the page's clock`);
  return start;
}

/** The journeys `node --test` finds, other than the harness's own test. */
export function journeysFound(): string[] {
  return globSync(PATTERN, { cwd: APP }).filter((f) => basename(f) !== "harness.journey.ts");
}

/** A journey: one node:test test with its own world core, dev server and browser context. */
export function journey(name: string, opts: JourneyOptions, fn: (j: Journey) => Promise<void>): void {
  test(name, { timeout: JOURNEY_GUARD_MS }, () => run(name, opts, fn));
}

async function run(name: string, opts: JourneyOptions, fn: (j: Journey) => Promise<void>): Promise<void> {
  if (process.env.ASTROMECH_WORLD) throw new Error("unset ASTROMECH_WORLD: the config's own bridge would start a second core");
  const world = resolve(ROOT, "worlds", opts.world);
  const start = worldStart(world);
  const out = join(RESULTS, name.replace(/[^a-z0-9]+/gi, "-").toLowerCase());
  rmSync(out, { recursive: true, force: true });
  mkdirSync(out, { recursive: true });
  const stderr = openSync(join(out, "core-stderr.log"), "w");
  let core: WorldCore | null = null;
  let server: ViteDevServer | null = null;
  let context: BrowserContext | null = null;
  let page: Page | null = null;
  const problems: string[] = [];
  let failure: unknown = null;
  try {
    core = await startWorld(world, { stderr });
    server = await devServer(core);
    const address = server.httpServer!.address();
    if (!address || typeof address === "string") throw new Error("the dev server has no port");
    const ctx = await (browser ??= chromium.launch()).then((b) => b.newContext({ viewport: WINDOW }));
    context = ctx;
    await ctx.tracing.start({ screenshots: true, snapshots: true });
    page = await ctx.newPage();
    await fn(driver(core, page, `http://127.0.0.1:${address.port}`, start, problems));
  } catch (e) {
    failure = e;
  }
  if (problems.length) failure = new Error(problems.join("\n"), failure ? { cause: failure } : undefined);
  if (failure) {
    if (page && !page.isClosed()) await page.screenshot({ path: join(out, "page.png") }).catch(() => {});
    await context?.tracing.stop({ path: join(out, "trace.zip") }).catch(() => {});
    const log = core && join(core.appDir, "logs", "astromech-core.log");
    if (log && existsSync(log)) copyFileSync(log, join(out, "astromech-core.log"));
  }
  await context?.close();
  await server?.close();
  closeSync(stderr);
  const ended = core && (await endCore(core));
  if (failure) throw failure;
  if (ended) throw new Error(ended);
  rmSync(out, { recursive: true, force: true });
  try {
    rmdirSync(RESULTS);
  } catch {
    // another journey's results are there
  }
}

/** The journey's view of its page and core. Fills `problems` with what fails the journey
 * whatever it waits on: an error the page throws, and a second load. */
function driver(core: WorldCore, page: Page, base: string, start: number, problems: string[]): Journey {
  page.setDefaultTimeout(PAGE_GUARD_MS);
  page.on("pageerror", (e) => problems.push(`the page threw: ${e.stack ?? e.message}`));
  let loads = 0;
  page.on("load", () => {
    if (++loads === 1) return;
    problems.push("the page loaded again mid-journey (did Vite re-optimise a dependency?)");
    void page.close();
  });
  const bridged: Request[] = [];
  page.on("request", (r) => {
    if (new URL(r.url()).pathname === CALL_PATH) bridged.push(r);
  });
  let barriers = 0;
  let opened = false;
  return {
    page,
    core,
    // appDir is <temp world>/local/astromech/data.
    world: resolve(core.appDir, "..", "..", ".."),
    async open(route) {
      if (opened) throw new Error("open() once per journey: the page loads once");
      opened = true;
      await core.callWindow("onboarding_done");
      // Installed, the page's clock runs on with real time; paused at once, it moves only on runFor.
      await page.clock.install({ time: start });
      await page.clock.pauseAt(start);
      await page.goto(`${base}/#/${route}`);
    },
    runFor: (ms) => page.clock.runFor(ms),
    calls: (cmd) => bridged.filter((r) => isCall(r, cmd)),
    async settled() {
      // Requests reach the harness in the order the page makes them: once one made now has
      // arrived, every earlier one has.
      const path = `/__journeys/settled/${++barriers}`;
      await Promise.all([page.waitForRequest((r) => new URL(r.url()).pathname === path), page.evaluate((p) => void fetch(p).catch(() => {}), path)]);
    },
  };
}

/** `r` is the page's call of `cmd` through the dev bridge. */
export function isCall(r: Request, cmd: string): boolean {
  return new URL(r.url()).pathname === CALL_PATH && (r.postDataJSON() as { cmd?: string } | null)?.cmd === cmd;
}

/** The dev server on app/vite.config.ts, its bridge sending to `core`. */
async function devServer(core: WorldCore): Promise<ViteDevServer> {
  const server = await createServer({
    configFile: join(APP, "vite.config.ts"),
    root: APP,
    plugins: [astromechBridge({ core })],
    logLevel: "error",
    // Loopback only, on the first free port from 5180 (`pnpm dev` keeps 5173). No file watching:
    // a source edited during a run reloads nothing.
    server: { host: "127.0.0.1", port: 5180, strictPort: false, watch: null },
    // No dependency pre-bundling: Vite serves svelte's own ES modules, so there is no optimiser
    // to find a new dependency mid-journey and reload the page, and no shared cache for the
    // journeys `node --test` runs side by side to write.
    optimizeDeps: { noDiscovery: true, include: [] },
  });
  await server.listen();
  return server;
}

/** Quit the core; why that fails the journey, or null. */
async function endCore(core: WorldCore): Promise<string | null> {
  const temp = resolve(core.appDir, "..", "..", "..");
  let code: number | null;
  try {
    code = await core.quit();
  } catch (e) {
    core.kill();
    return `quitting the world core: ${e instanceof Error ? e.message : String(e)}`;
  }
  if (code !== 0) return `the world core exited ${code}, not 0`;
  if (existsSync(temp)) return `the world core left its temp world ${temp}`;
  return null;
}
