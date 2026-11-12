/**
 * Recorded cores for the browser preview and component tests (no Tauri, no Windows): what the
 * core's control API answers on a PC in one of three states, picked with `?core=<scenario>`.
 * - `empty`: Elite never ran here, no sticks, no microphone, nothing set up;
 * - `new`: a fresh install on a PC with Elite, two sticks and two microphones;
 * - `ready`: the same PC after onboarding.
 * Commands mutate the in-memory state the way the core would. Synthetic data only.
 *
 * Any scenario (or none) can also be made slow or broken, to see a page's loading and error states:
 * - `?fail=<name>,<name>` (or `?fail=all`): those calls are rejected, as a failed call is;
 * - `?slow=<ms>` or `?slow=never`: every call waits that long, or never answers;
 *   `?slow=never:<name>,<name>` limits it to the named calls.
 * A name is the command the window sends: `logbook`, `session_summary`, `engineering_fleet`,
 * `materials_inventory`, `eddn_log`, `status`, and so on (the `call` name in lib/*.ts).
 * `?core=empty` makes the sample-backed pages answer as a commander with no history (previewData,
 * used by shell.ts), and the network lookups fail for want of a position unless a system is typed
 * (previewLookup). Without `?core=` the core is not running: core commands fail as the window's
 * calls do then, except the few with a sample (NO_CORE below).
 *
 * `?core=ready&route=exact` starts on a route, for the route strip: fixtures/plot.json's exact
 * route, with the commander in its third waypoint. `?core=ready&route=timeline` starts on
 * fixtures/route.json's: a neutron route whose flown leg lists its jumps from the galaxy map,
 * with inhabited stops' details, answered as recorded until a command changes the route. Without
 * either the preview has no route.
 */
import type { Biology } from "../lib/api/types/Biology";
import type { CalibrationStep } from "../lib/api/types/CalibrationStep";
import type { Colour } from "../lib/api/types/Colour";
import type { TextSize } from "../lib/api/types/TextSize";
import type { Done } from "../lib/api/types/Done";
import type { Fleet } from "../lib/api/types/Fleet";
import type { EddnHead } from "../lib/api/types/EddnHead";
import type { Mining } from "../lib/api/types/Mining";
import type { Ores } from "../lib/api/types/Ores";
import type { Outbox } from "../lib/api/types/Outbox";
import type { RouteReply } from "../lib/api/types/RouteReply";
import type { RouteStop } from "../lib/api/types/RouteStop";
import type { Scoop } from "../lib/api/types/Scoop";
import type { ShipStatus } from "../lib/api/types/ShipStatus";
import type { CoreCmd, CoreStatus, Level } from "../lib/core";
import type { EdsmStatus } from "../lib/edsm";
import type { Plotted, Route, RouteView, Waypoint } from "../lib/route";
import type { ArgList, Args, Reply } from "../lib/transport";
import type { DryRun, LearnedPhrase } from "../lib/voice";
import eddnFixture from "./fixtures/eddn-log.json";
import biologyFixture from "./fixtures/exobiology.json";
import miningFixture from "./fixtures/mining.json";
import shipFixture from "./fixtures/ship-status.json";
import plotSample from "./fixtures/plot.json";
import routeFixture from "./fixtures/route.json";
import voiceFixture from "./fixtures/voice.json";
import { previewQuery } from "./query";
import { labelled, recorded, type Handler, type Handlers } from "./recorded";

const eddnSample = recorded<Outbox<EddnHead>>(eddnFixture);
const biologySample = recorded<Done<Biology>>(biologyFixture);
const miningSample = recorded<Mining>(miningFixture);
const routeSample = recorded<{ plotted: Plotted; view: RouteView }>(routeFixture);
const exactSample = recorded<Plotted>(plotSample.exact);
/** The ship world's replies: `flying` is the status bar's sample (`refitted` and its fleet, the same
 * ship after a refit, are the bar's test's). */
const shipSample = recorded<{ flying: Done<ShipStatus>; refitted: Done<ShipStatus>; refitted_fleet: Fleet }>(shipFixture).flying;

export type Scenario = "empty" | "new" | "ready";

const BINDINGS = "C:\\Users\\you\\AppData\\Local\\Frontier Developments\\Elite Dangerous\\Options\\Bindings";
const JOURNALS = "C:\\Users\\you\\Saved Games\\Frontier Developments\\Elite Dangerous";
const MICS = ["Microphone (Example USB Mic)", "Headset Microphone (Example VR Headset)"];
const PHRASES: [string, string, string] = ["gear down", "deploy hardpoints", "request docking"];

interface State {
  scenario: Scenario;
  elite: boolean;
  bindingsDir: string;
  mics: string[];
  ptt: string | null;
  kill: string | null;
  mic: string;
  models: boolean;
  modelsProgress: string | null;
  onboarded: boolean;
  gates: Record<string, number>;
  run: number | null;
  voice: { speaker: string; speed: number; colour: Colour };
  /** `[window] text_size`. */
  textSize: TextSize;
  previews: string[];
  /** Prospector thresholds by ore (null: off), as the core's `prospector_thresholds` lists. */
  prospector: Record<string, number | null>;
  /** Callouts the config changes, by id (core: `[callouts.<id>]`). */
  callouts: Record<string, { enabled?: boolean; cooldown_s?: number }>;
  eddn: boolean;
  /** EDSM: the switch, the commander name, and whether a key is "in the keyring". */
  edsm: { enabled: boolean; commander: string; key: boolean };
  /** The route the core follows (core: route.rs `Desk`); `here` is where the journal says we are. */
  route: Route | null;
  /** The route a galaxy-map plot replaced. */
  parked: Route | null;
  /** `?route=timeline`: the recorded view, answered until a command changes the route. */
  timeline: RouteView | null;
  here: string | null;
  /** Why the clipboard turns writes down (tests: `fakeClipboard`), and the last write's error. */
  clipFails: string | null;
  clipError: string | null;
}

let state: State | null = null;

/** A few of the core's ores (crates/astromech-announce/src/prospector.rs `ORES`). */
const PROSPECTOR_NAMES: Record<string, string> = { lowtemperaturediamond: "Low Temperature Diamonds", painite: "Painite", platinum: "Platinum" };
const PROSPECTOR_DEFAULTS: Record<string, number | null> = { lowtemperaturediamond: 25, painite: 25, platinum: 20 };

/** A few of the core's callouts (crates/astromech-announce `catalog()`). */
const CALLOUTS = [
  { id: "arrival", title: "Arrival briefing", default_enabled: true, default_cooldown_s: 0 },
  { id: "powerplay_arrival", title: "Powerplay arrival", default_enabled: false, default_cooldown_s: 0 },
  { id: "scan", title: "Scans", default_enabled: true, default_cooldown_s: 30 },
  { id: "hge", title: "High Grade Emissions", default_enabled: true, default_cooldown_s: 30 },
];

function calloutsReply(st: State): Reply<"callouts"> {
  return {
    ok: true,
    callouts: CALLOUTS.map((c) => ({
      ...c,
      enabled: st.callouts[c.id]?.enabled ?? c.default_enabled,
      cooldown_s: st.callouts[c.id]?.cooldown_s ?? c.default_cooldown_s,
    })),
  };
}

function prospectorReply(st: State): Done<Ores> {
  const ores = Object.keys(PROSPECTOR_DEFAULTS).map((ore) => ({
    ore,
    name: PROSPECTOR_NAMES[ore] ?? ore,
    default: PROSPECTOR_DEFAULTS[ore] ?? null,
    threshold: st.prospector[ore] ?? null,
  }));
  return { ok: true, ores };
}

/** `?fail=` / `?slow=` entries: `all` or names; null when the parameter is absent. */
function names(list: string | null): { all: boolean; some: Set<string> } | null {
  if (list === null) return null;
  const some = new Set(list.split(",").map((x) => x.trim()).filter(Boolean));
  return { all: some.has("all") || some.size === 0, some };
}

/**
 * Waits or fails as the URL asks for this call (see the top of this file). Every sample-backed
 * call goes through here, so each page can be seen loading and failing.
 */
export async function previewGate(cmd: string): Promise<void> {
  if (typeof location === "undefined") return;
  const q = previewQuery();
  const fail = names(q.get("fail"));
  const slow = q.get("slow");
  if (slow) {
    const [when, list] = slow.split(":") as [string, string | undefined];
    const only = names(list ?? null);
    if (!only || only.all || only.some.has(cmd)) {
      if (when === "never") await new Promise<never>(() => {});
      else if (Number(when) > 0) await sleep(Number(when));
    }
  }
  if (fail && (fail.all || fail.some.has(cmd))) throw `preview: ${cmd} failed on purpose (?fail=)`;
}

/** What a sample-backed call answers in the browser: the sample, or `empty` under `?core=empty`. */
export async function previewData<T>(cmd: string, sample: () => T, empty: () => T): Promise<T> {
  await previewGate(cmd);
  return fakeScenario() === "empty" ? empty() : sample();
}

/**
 * What a network lookup (Spansh, Ardent) answers in the browser: the sample. Under `?core=empty`
 * with no typed system it fails the way the core does without a position in the journal
 * (`search_origin`); a typed system still searches, as it does on a PC with no journal.
 */
export async function previewLookup<T>(cmd: string, origin: string | null | undefined, sample: () => T): Promise<T> {
  await previewGate(cmd);
  if (fakeScenario() === "empty" && !origin?.trim()) throw "no position in the journal yet";
  return sample();
}

export function fakeScenario(): Scenario | null {
  if (typeof location === "undefined") return null;
  const s = previewQuery().get("core");
  return s === "empty" || s === "new" || s === "ready" ? s : null;
}

function fresh(scenario: Scenario): State {
  const hw = scenario !== "empty";
  const done = scenario === "ready";
  return {
    scenario,
    elite: hw,
    bindingsDir: BINDINGS,
    mics: hw ? MICS : [],
    ptt: done ? "231D3201 Joy_4" : null,
    kill: done ? "Keyboard Key_ScrollLock" : null,
    mic: done ? "Example USB" : "",
    models: done,
    modelsProgress: null,
    onboarded: done,
    gates: done ? { [MICS[0] ?? ""]: -51 } : {},
    run: null,
    voice: { speaker: "bf_emma", speed: 1, colour: "cockpit" },
    textSize: "normal",
    previews: [],
    prospector: { ...PROSPECTOR_DEFAULTS },
    callouts: {},
    eddn: false,
    edsm: { enabled: false, commander: "", key: false },
    route: done ? startingRoute() : null,
    parked: null,
    timeline: done && previewQuery().get("route") === "timeline" ? structuredClone(routeSample.view) : null,
    here: done ? startingHere() : null,
    clipFails: null,
    clipError: null,
  };
}

/** `?route=exact`: the exact route from fixtures/plot.json, with the commander in its third
 * waypoint, so the fourth is next. */
function startingRoute(): Route | null {
  const scenario = previewQuery().get("route");
  if (scenario === "timeline") return structuredClone(routeSample.view.route);
  if (scenario !== "exact") return null;
  const { name, source, waypoints, plot } = exactSample;
  return { name, source, waypoints: structuredClone(waypoints), next: 3, plot };
}

/** Where the commander starts: the recorded timeline's system, or the exact route's third stop. */
function startingHere(): string {
  if (previewQuery().get("route") === "timeline") return routeSample.view.here ?? "Sol";
  return startingRoute()?.waypoints[2]?.system ?? "Sol";
}

const sameSystem = (a: string, b: string) => a.trim().toLowerCase() === b.trim().toLowerCase();

/** As the core's `Route::new`: blank systems dropped, stops in one system merged. */
function cleanStops(list: Waypoint[]): Waypoint[] {
  const out: Waypoint[] = [];
  for (const w of list) {
    const system = w.system.trim();
    if (!system) continue;
    const last = out.at(-1);
    if (last && sameSystem(last.system, system)) {
      if (w.label && w.label !== last.label) last.label = last.label ? `${last.label}, ${w.label}` : w.label;
      if (last.jumps != null) last.jumps += w.jumps ?? 0;
      if (last.ly != null) last.ly += w.ly ?? 0;
    } else {
      const { label, ...rest } = w;
      out.push({ ...rest, system, ...(label ? { label } : {}) });
    }
  }
  return out;
}

/** As the core's `parse_list`, roughly: a CSV's "System Name" column, or one system per line. */
function parseList(text: string): Waypoint[] {
  const lines = text.split("\n").map((l) => l.trim()).filter(Boolean);
  const cells = (l: string) => l.split(",").map((c) => c.trim().replace(/^"|"$/g, ""));
  const col = lines[0] ? cells(lines[0]).findIndex((h) => /^system( name)?$/i.test(h)) : -1;
  const rows = col >= 0 && lines[0]?.includes(",") ? lines.slice(1).map((l) => cells(l)[col] ?? "") : lines.map((l) => l.replace(/^"|"$/g, ""));
  return rows.map((system) => ({ system }));
}

/** As the core's `Route::jumps_left`: null when a waypoint still to reach doesn't know its jumps. */
function jumpsLeft(r: Route): number | null {
  const ahead = r.waypoints.slice(r.next);
  return ahead.every((w) => w.jumps != null) ? ahead.reduce((n, w) => n + (w.jumps ?? 0), 0) : null;
}

/** As the core's `Scoop::of`: from the star class (KGBFOAM), else the plotter's flag. */
function scoop(starClass: string | null | undefined, flag: boolean | null | undefined): Scoop {
  const known = starClass != null ? "OBAFGKM".includes(starClass.split("_")[0] ?? "-") && starClass.split("_")[0]?.length === 1 : flag;
  return known == null ? "unknown" : known ? "yes" : "no";
}

/** As the core's timeline, without the galaxy map's route: each stop with what its source gave,
 * and the plotter's listed jumps; the commander's place on the flown leg when it lists them. */
function timeline(r: Route, here: string | null): { stops: RouteStop[]; position: number | null } {
  let position: number | null = null;
  const stops = r.waypoints.map((w, i): RouteStop => {
    const prev = r.waypoints[i - 1];
    if (i === r.next && w.hops && prev && here) {
      const systems = [prev.system, ...w.hops.map((h) => h.system)];
      const at = systems.map((s) => sameSystem(s, here)).lastIndexOf(true);
      position = at >= 0 ? at : null;
    }
    const neutron = (c: string | null | undefined, flag: boolean | null | undefined) => (c != null ? c === "N" : (flag ?? null));
    return {
      system: w.system,
      label: w.label ?? null,
      star_class: w.star_class ?? null,
      scoopable: scoop(w.star_class, w.scoopable),
      neutron: neutron(w.star_class, w.neutron),
      jumps: w.jumps ?? null,
      ly: w.ly ?? null,
      hops: w.hops
        ? w.hops.map((h) => ({ system: h.system, star_class: h.star_class ?? null, scoopable: scoop(h.star_class, h.scoopable), neutron: neutron(h.star_class, h.neutron) }))
        : null,
      hops_from: w.hops ? "plotter" : null,
      details: null,
    };
  });
  return { stops, position };
}

/** The preview's clipboard takes the text unless a test made it fail (`fakeClipboard`). */
function routeView(st: State): RouteView {
  if (st.timeline) return { ...structuredClone(st.timeline), clipboard_error: st.clipError };
  const r = st.route;
  return {
    ...(r ? timeline(r, st.here) : { stops: [], position: null }),
    route: r ? structuredClone(r) : null,
    here: st.here,
    next: r?.waypoints[r.next] ?? null,
    remaining: r ? r.waypoints.length - r.next : 0,
    jumps_left: r ? jumpsLeft(r) : null,
    clipboard_error: st.clipError,
    parked: st.parked ? structuredClone(st.parked) : null,
  };
}

/** The core's `route::GAME`: the source of a route plotted in the galaxy map. */
const GAME_SOURCE = "galaxy map";
const ahead = (r: Route, system: string) => r.waypoints.slice(r.next).some((w) => sameSystem(w.system, system));

/** As the core on resuming a parked route: already in a waypoint ahead, it carries on after it. */
function skipReached(st: State): void {
  const r = st.route;
  if (!r || !st.here) return;
  const i = r.waypoints.findIndex((w, j) => j >= r.next && sameSystem(w.system, st.here!));
  if (i >= 0) r.next = i + 1;
}

/** The commander plotted `systems` in the galaxy map, as the core's `Desk::plotted` takes it
 * (tests): a leg of the route followed changes nothing, a plot onto the parked route resumes it,
 * anything else is followed and parks an unfinished route of ours. */
export function fakePlotInGame(systems: string[]): void {
  const st = changed(s());
  const dest = systems.at(-1);
  if (!dest || systems.length < 2) return;
  const r = st.route;
  if (r && r.source !== GAME_SOURCE && ahead(r, dest)) return;
  if (st.parked && ahead(st.parked, dest)) {
    st.route = st.parked;
    st.parked = null;
    skipReached(st);
    return;
  }
  if (r && r.source !== GAME_SOURCE && r.next < r.waypoints.length) st.parked = r;
  st.route = { name: `Route to ${dest}`, source: GAME_SOURCE, waypoints: cleanStops(systems.slice(1).map((system) => ({ system, jumps: 1 }))), next: 0 };
}

/** An unfinished route of our own (not the galaxy map's). */
const ours = (r: Route | null): r is Route => !!r && r.source !== GAME_SOURCE && r.next < r.waypoints.length;

/** As the core's `Desk::set`: follow `waypoints`, parking the unfinished route of ours it replaces. */
function follow(st: State, name: string, source: string, waypoints: Waypoint[], plot?: Route["plot"]): void {
  const first = waypoints[0];
  const next = st.here && first && sameSystem(first.system, st.here) ? 1 : 0;
  if (ours(st.route)) st.parked = st.route;
  st.route = { name, source, waypoints, next, ...(plot ? { plot } : {}) };
}

/** As the core's `Desk::add` on a route of ours: the stop becomes the next waypoint, merged with
 * the next one or the one just passed when it is in the same system; already in its system, it
 * counts as reached. */
function addNext(st: State, r: Route, stop: Waypoint): void {
  const merge = (a?: string | null, b?: string | null) => (a && b && a !== b ? `${a}, ${b}` : (a ?? b));
  const at = r.waypoints[r.next];
  const prev = r.waypoints[r.next - 1];
  if (at && sameSystem(at.system, stop.system)) at.label = merge(stop.label, at.label);
  else if (prev && sameSystem(prev.system, stop.system)) {
    prev.label = merge(prev.label, stop.label);
    r.next -= 1;
  } else {
    // The leg after the new stop starts there now: it is unknown.
    if (at) {
      delete at.jumps;
      delete at.ly;
      delete at.hops;
    }
    r.waypoints.splice(r.next, 0, stop);
  }
  const next = r.waypoints[r.next];
  if (st.here && next && sameSystem(next.system, st.here)) r.next += 1;
}

/** Tests: the clipboard turns writes down with `error` from the next one on, or takes them again. */
export function fakeClipboard(error: string | null): void {
  s().clipFails = error;
}

/** A live arrival in `system`, as the core's `Desk::on_system` takes it (tests): the commander is
 * there, and a waypoint ahead is reached. */
export function fakeJump(system: string): void {
  const st = changed(s());
  st.here = system;
  skipReached(st);
}

/** A command changed the route: the recorded timeline no longer answers. */
function changed(st: State): State {
  st.timeline = null;
  return st;
}

/** The route commands' reply: the route's view. */
const viewed = (st: State): Done<RouteReply> => ({ ok: true, view: routeView(st) });

/** Every route command but reading the route (and clearing it) writes the next waypoint to the
 * clipboard, failing as the test's clipboard says. */
function copies(st: State): State {
  st.clipError = st.clipFails;
  return st;
}

const routeHandlers: Pick<
  CoreHandlers,
  "route" | "route_details" | "route_set" | "route_add" | "route_skip" | "route_back" | "route_clear" | "route_copy" | "route_resume"
> = {
  route: viewed,
  // The recorded timeline has its stops' details; another route's stops have none here.
  route_details: viewed,
  route_set: (st, args) => {
    copies(changed(st));
    const waypoints = cleanStops(typeof args.text === "string" ? parseList(args.text) : (args.waypoints ?? []));
    if (!waypoints.length) return { ok: false, error: "the route has no systems" };
    follow(st, args.name || "Route", args.source ?? "", waypoints, args.plot ?? undefined);
    return viewed(st);
  },
  route_add: (st, args) => {
    copies(changed(st));
    const system = (args.system ?? "").trim();
    if (!system) return { ok: false, error: "the stop has no system" };
    const stop: Waypoint = { system, ...(args.label?.trim() ? { label: args.label } : {}) };
    const r = st.route;
    if (ours(r)) addNext(st, r, stop);
    else follow(st, system, args.source ?? "", [stop]);
    return viewed(st);
  },
  route_skip: (st) => {
    const r = copies(changed(st)).route;
    if (r) r.next = Math.min(r.next + 1, r.waypoints.length);
    return viewed(st);
  },
  route_back: (st) => {
    const r = copies(changed(st)).route;
    if (r) r.next = Math.max(r.next - 1, 0);
    return viewed(st);
  },
  route_clear: (st) => {
    changed(st);
    st.clipError = null;
    st.route = null;
    st.parked = null;
    return viewed(st);
  },
  route_copy: (st) => viewed(copies(st)),
  route_resume: (st) => {
    copies(changed(st));
    if (!st.parked) return { ok: false, error: "there is no route to resume" };
    st.route = st.parked;
    st.parked = null;
    skipReached(st);
    return viewed(st);
  },
};

function s(): State {
  const sc = fakeScenario() ?? "empty";
  if (!state || state.scenario !== sc) state = fresh(sc);
  return state;
}

/** Start over (tests). */
export function resetFakeCore(): void {
  state = null;
}

function chosen(st: State): string | null {
  const want = st.mic.trim().toLowerCase();
  if (!want) return st.mics[0] ?? null;
  return st.mics.find((m) => m.toLowerCase() === want) ?? st.mics.find((m) => m.toLowerCase().includes(want)) ?? null;
}

const VERSION = "1.0.0-alpha.1";
const APP_DIR = "C:\\Users\\you\\AppData\\Local\\astromech\\data";

function status(st: State): Done<CoreStatus> {
  const setup = [...(st.ptt ? [] : ["push-to-talk not set"]), ...(st.models ? [] : ["speech models not downloaded"])];
  return {
    ok: true,
    version: VERSION,
    pid: 4242,
    game: "not_running",
    active: false,
    starting: false,
    voice: "voice not loaded",
    voice_paused: false,
    killed: false,
    autostart: true,
    crash_pending: false,
    status_line: "Waiting for Elite",
    app_dir: APP_DIR,
    log_dir: `${APP_DIR}\\logs`,
    config: `${APP_DIR}\\astromech.toml`,
    config_note: null,
    setup_needed: setup,
    ptt: st.ptt,
    kill_switch: st.kill,
    mic: st.mic,
    onboarded: st.onboarded,
    models_missing: !st.models,
    models_progress: st.modelsProgress,
    models_dir: `${APP_DIR}\\models`,
    // A core that hasn't checked for updates yet, raised no callouts and follows no journal yet.
    update: { status: "idle", installed: VERSION, latest: null, notes: "", error: null, last_check: null, ready: false, apply_error: null },
    toasts: [],
    history: null,
    mining: st.scenario === "empty" ? null : miningSample,
  };
}

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

function level(device: string, speech: boolean): Level {
  return speech
    ? { device, peak_dbfs: -24.3, noise_dbfs: -63.8, verdict: "ok" }
    : { device, peak_dbfs: -61.2, noise_dbfs: -64.0, verdict: "silent" };
}

function step(n: number): CalibrationStep {
  if (n === 0) return { step: 0, steps: 4, phrase: "", ms: 2000 };
  if (n <= PHRASES.length) return { step: n, steps: 4, phrase: PHRASES[n - 1] ?? null, ms: 2500 };
  return { step: n, steps: 4, phrase: null, ms: 0 };
}

/** The English Kokoro v1.0 voices, British first (core: speech::voice_choices). */
const VOICES = [
  ...["bf_alice", "bf_emma", "bf_isabella", "bf_lily", "bm_daniel", "bm_fable", "bm_george", "bm_lewis"],
  ...["af_alloy", "af_aoede", "af_bella", "af_heart", "af_jessica", "af_kore", "af_nicole", "af_nova", "af_river", "af_sarah", "af_sky"],
  ...["am_adam", "am_echo", "am_eric", "am_fenrir", "am_liam", "am_michael", "am_onyx", "am_puck", "am_santa"],
].map((name) => ({
  name,
  label: name.slice(3, 4).toUpperCase() + name.slice(4),
  accent: name.startsWith("b") ? "British" : "American",
  gender: name[1] === "f" ? "female" : "male",
}));

/** Voices previewed so far, in order (tests). */
export function fakePreviews(): string[] {
  return s().previews;
}

const NO_MIC = "no recording device found (plug in a microphone or headset)";

/** How the fake core answers core command `K`, given its state. */
type CoreHandler<K extends CoreCmd> = (st: State, args: Args<K>) => Reply<K> | Promise<Reply<K>>;
type CoreHandlers = { [K in CoreCmd]: CoreHandler<K> };
const ok = () => ({ ok: true }) as const;
const applies = () => ({ ok: true, applies_next_session: false }) as const;

/** Every core command the window sends, as the core answers it (or near enough). */
export const coreHandlers: CoreHandlers = {
  status,
  save_diagnostics: () => ({ ok: true, path: `${APP_DIR}\\diagnostics\\astromech-diagnostics-20261016-200000.zip` }),
  ack_crash: ok,
  check_update: ok,
  install_update: ok,
  autostart_on: ok,
  autostart_off: ok,
  pause_voice: ok,
  resume_voice: ok,
  kill_switch: ok,
  open_config: ok,
  // The core answers with how many materials the merged list has; the preview keeps one ship's.
  shopping_set: (_st, args) => {
    const change = JSON.parse(args) as { materials: unknown[] };
    return { ok: true, materials: change.materials.length };
  },
  ...routeHandlers,
  find_elite: (st) => ({
    ok: true,
    found: {
      bindings_dir: st.bindingsDir,
      bindings_found: st.elite,
      journal_dir: JOURNALS,
      journal_found: st.elite,
      install: st.elite ? "D:\\SteamLibrary\\steamapps\\common\\Elite Dangerous\\Products\\elite-dangerous-odyssey-64" : null,
      install_source: st.elite ? "Steam" : null,
      ok: st.elite,
    },
  }),
  set_folders: (st, args) => {
    if (args.bindings_dir) {
      st.bindingsDir = args.bindings_dir;
      // The preview pretends the folder the user typed exists.
      st.elite = true;
    }
    return applies();
  },
  set_ptt: (st, args) => {
    st.ptt = args ? `${args.device} ${args.key}` : null;
    return applies();
  },
  set_kill_switch: (st, args) => {
    st.kill = args ? `${args.device} ${args.key}` : null;
    return applies();
  },
  set_mic: (st, args) => {
    st.mic = args.mic ?? "";
    return applies();
  },
  mic_list: (st) => {
    const c = chosen(st);
    return {
      ok: true,
      names: st.mics,
      default: st.mics[0] ?? null,
      setting: st.mic,
      chosen: st.mic ? c : null,
      calibration: { device: c ?? (st.mic || "default"), min_level_dbfs: c ? (st.gates[c] ?? null) : null },
    };
  },
  mic_check: async (st) => {
    const c = chosen(st);
    if (!c) return { ok: false, error: NO_MIC };
    await sleep(150);
    return { ok: true, level: level(c, true) };
  },
  calibrate_start: (st) => {
    st.run = 0;
    return { ok: true, phrases: PHRASES, next: step(0) };
  },
  calibrate_record: async (st) => {
    if (st.run === null) return { ok: false, error: "calibration hasn't started" };
    const c = chosen(st);
    if (!c) return { ok: false, error: NO_MIC };
    if (st.run > PHRASES.length) return { ok: false, error: "calibration already has every clip" };
    await sleep(150);
    const lv = level(c, st.run > 0);
    st.run++;
    return { ok: true, level: lv, next: step(st.run) };
  },
  calibrate_finish: (st) => {
    const c = chosen(st);
    if (st.run === null || st.run <= PHRASES.length || !c) return { ok: false, error: `calibration needs silence and all ${PHRASES.length} phrases first` };
    st.gates[c] = -51;
    st.run = null;
    return { ok: true, device: c, noise_dbfs: -64, speech_peak_dbfs: -24.3, min_level_dbfs: -51, gain_db: 18.3 };
  },
  calibrate_cancel: (st) => {
    st.run = null;
    return { ok: true };
  },
  download_models: (st) => {
    st.modelsProgress = "Parakeet Downloading 0%";
    setTimeout(() => {
      st.models = true;
      st.modelsProgress = null;
    }, 1500);
    return { ok: true };
  },
  voice_settings: (st) => ({ ok: true, ...st.voice, voices: VOICES, speeds: [0.7, 1.3] }),
  set_voice: (st, args) => {
    if (args.speaker && !VOICES.some((v) => v.name === args.speaker)) return { ok: false, error: `unknown voice "${args.speaker}"` };
    st.voice = { speaker: args.speaker ?? st.voice.speaker, speed: args.speed ?? st.voice.speed, colour: args.colour ?? st.voice.colour };
    return applies();
  },
  window_settings: (st) => ({ ok: true, text_size: st.textSize }),
  set_window: (st, args) => {
    if (!["smaller", "normal", "larger"].includes(args.text_size)) return { ok: false, error: "text_size is smaller, normal or larger" };
    st.textSize = args.text_size;
    return applies();
  },
  voice_preview: (st) => {
    if (!st.models) return { ok: false, error: "download the speech models first (Speech models, above)" };
    st.previews.push(st.voice.speaker);
    return { ok: true, line: "Landing gear deployed." };
  },
  callouts: (st) => calloutsReply(st),
  set_callout: (st, c) => {
    if (!CALLOUTS.some((x) => x.id === c.id)) return { ok: false, error: `unknown callout "${c.id}"` };
    if (typeof c.cooldown_s === "number" && !(c.cooldown_s >= 0 && c.cooldown_s <= 86400)) {
      return { ok: false, error: "cooldown_s must be between 0 and 86400 seconds" };
    }
    const cur = (st.callouts[c.id] ??= {});
    if (typeof c.enabled === "boolean") cur.enabled = c.enabled;
    if (c.cooldown_s === null) delete cur.cooldown_s;
    else if (typeof c.cooldown_s === "number") cur.cooldown_s = c.cooldown_s;
    return applies();
  },
  prospector_thresholds: (st) => prospectorReply(st),
  prospector_set: (st, args) => {
    const c = JSON.parse(args) as { ore: string; threshold?: number | null; reset?: boolean };
    if (!(c.ore in PROSPECTOR_DEFAULTS)) return { ok: false, error: `unknown ore "${c.ore}"` };
    st.prospector[c.ore] = c.reset ? (PROSPECTOR_DEFAULTS[c.ore] ?? null) : (c.threshold ?? null);
    return prospectorReply(st);
  },
  eddn_log: (st) => {
    // Sharing was on earlier (the sample's history); the switch is this PC's setting.
    if (st.scenario === "empty") return { ok: true, enabled: st.eddn, test: false, pending: 0, last_sent: null, last_error: null, log: [], skipped: [] };
    return { ok: true, ...eddnSample, enabled: st.eddn, pending: st.eddn ? eddnSample.pending : 0 };
  },
  set_eddn: (st, args) => {
    st.eddn = args.enabled === true;
    return applies();
  },
  edsm_status: (st) => {
    const e = st.edsm;
    const uploading = e.enabled && e.key && e.commander !== "";
    return { ok: true, enabled: e.enabled, commander: e.commander, has_key: e.key, key_error: null, uploading,
      pending: 0, last_sent: null, last_error: null, log: [], skipped: [] };
  },
  set_edsm: (st, ch) => {
    if (ch.enabled != null) st.edsm.enabled = ch.enabled;
    if (ch.commander != null) st.edsm.commander = ch.commander.trim();
    if (ch.api_key != null) st.edsm.key = ch.api_key.trim() !== "";
    return applies();
  },
  edsm_flight_log: (st) => {
    if (!st.edsm.key || !st.edsm.commander) return { ok: false, error: "set your EDSM commander name and API key first" };
    return { ok: true, visits: [
      { system: "Sol", date: "2026-10-16 20:00:00", first_discover: false, ship_id: 7 },
      { system: "Achenar", date: "2026-10-16 19:10:00", first_discover: true, ship_id: 7 },
    ] };
  },
  onboarding_done: (st) => {
    st.onboarded = true;
    return { ok: true };
  },
  // Only a session (Elite running) has a system; `ready` is in one.
  exobiology: (st) => (st.scenario === "ready" ? { ...biologySample } : { ok: false, error: "Elite isn't running." }),
  // The status bar's fuel and cargo: the sample ship's in a session, nothing known outside one.
  ship_status: (st) => (st.scenario === "ready" ? { ...shipSample } : { ok: true, ship_id: null, ship_name: null, max_jump_range: null, fuel: null, fuel_capacity: null, cargo: null, cargo_capacity: null }),
  // The voice tester answers from sample results in every scenario (see NO_CORE).
  voice_test: (_st, args) => ({ ok: true, result: sampleRun(args) }),
  voice_test_poll: () => ({ ok: true, listening: false, last: null }),
  voice_test_stop: ok,
  // Elite isn't running here (the status says so), so there's nothing to press.
  voice_test_live: () => ({ ok: false, error: "Elite isn't running, so there's nothing to send keys to." }),
  learned_phrases: () => ({ ok: true, phrases: learned }),
  learned_forget: (_st, args) => {
    learned = learned.filter((p) => p.heard !== args);
    return { ok: true, phrases: learned };
  },
};

async function answer<K extends CoreCmd>(cmd: K, args: Args<K>): Promise<Reply<K>> {
  await previewGate(cmd);
  const handler: CoreHandler<K> = coreHandlers[cmd];
  return handler(s(), args);
}

/** A core command, answered by the fake core in the URL's scenario (any scenario: tests). */
export function fakeCall<K extends CoreCmd>(cmd: K, ...[args]: ArgList<K>): Promise<Reply<K>> {
  return answer(cmd, args as Args<K>);
}

/** What the preview answers with no core (no `?core=`): samples for some pages; the rest fail as
 * the window's calls do with no core. */
const NO_CORE: Partial<Pick<Handlers, CoreCmd>> = {
  eddn_log: async () => {
    await previewGate("eddn_log");
    return { ok: true, ...eddnSample };
  },
  edsm_status: () => ({ ok: true, ...EDSM_SAMPLE }),
  exobiology: async () => {
    await previewGate("exobiology");
    return labelled(biologySample, "Sample system (synthetic)");
  },
  voice_test: (args) => fakeCall("voice_test", args),
  voice_test_poll: () => fakeCall("voice_test_poll"),
  voice_test_stop: () => fakeCall("voice_test_stop"),
  learned_phrases: () => fakeCall("learned_phrases"),
  learned_forget: (args) => fakeCall("learned_forget", args),
};

/** A core command as the browser preview answers it: the fake core in the URL's scenario, or with
 * no scenario the samples above. */
export function coreReply<K extends CoreCmd>(cmd: K, args: Args<K>): Promise<Reply<K>> {
  if (fakeScenario()) return answer(cmd, args);
  const f: Handler<K> | undefined = NO_CORE[cmd];
  return f ? Promise.resolve(f(args)) : Promise.reject("not running inside the app");
}

/** The core's commands as the preview's transport answers them (transport.ts). */
export function coreCommands(): Pick<Handlers, CoreCmd> {
  const viaCore = <K extends CoreCmd>(cmd: K): Handler<K> => (args) => coreReply(cmd, args);
  // Object.fromEntries loses each key's own type; every entry is viaCore's, typed above.
  return Object.fromEntries((Object.keys(coreHandlers) as CoreCmd[]).map((cmd) => [cmd, viaCore(cmd)])) as unknown as Pick<Handlers, CoreCmd>;
}

// ---- The voice tester's samples.
const voiceSamples = recorded<Record<string, DryRun>>(voiceFixture);
const norm = (t: string) => t.toLowerCase().replace(/[^a-z ]/g, "").trim();

/** The sample phrases match, anything else is "Say again". */
function sampleRun(text: string): DryRun {
  const hit = voiceSamples[norm(text)];
  if (hit) return { ...hit, transcript: text };
  return { ...voiceSamples["banana bread"]!, transcript: text };
}

let learned: LearnedPhrase[] = [
  { heard: "You're down.", command: "landing_gear.down", does: "gear down" },
  { heard: "lamps", command: "lights.on", does: "lights on" },
];

const EDSM_SAMPLE: EdsmStatus = {
  enabled: true,
  commander: "Jameson",
  has_key: true,
  key_error: null,
  uploading: true,
  pending: 0,
  last_sent: { at: 1792166460, schema_ref: "journal", outcome: "sent", detail: "12 events" },
  last_error: null,
  log: [{ at: 1792166460, schema_ref: "journal", outcome: "sent", detail: "12 events" }],
  skipped: [],
};

