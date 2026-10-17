/**
 * Recorded cores for the browser preview and component tests (no Tauri, no Windows): what the
 * core's control API answers on a PC in one of three states, picked with `?core=<scenario>`.
 * - `empty`: Elite never ran here, no sticks, no microphone, nothing set up;
 * - `new`: a fresh install on a PC with Elite, two sticks and two microphones;
 * - `ready`: the same PC after onboarding.
 * Commands mutate the in-memory state the way the core would. Synthetic data only.
 */
import type { CoreCmd, CoreStatus, Reply } from "./core";
import type { Route, RouteView, Waypoint } from "./route";
import eddnSample from "./fixtures/eddn-log.json";
import biologySample from "./fixtures/exobiology.json";

export type Scenario = "empty" | "new" | "ready";

const BINDINGS = "C:\\Users\\you\\AppData\\Local\\Frontier Developments\\Elite Dangerous\\Options\\Bindings";
const JOURNALS = "C:\\Users\\you\\Saved Games\\Frontier Developments\\Elite Dangerous";
const MICS = ["Microphone (Example USB Mic)", "Headset Microphone (Example VR Headset)"];
const PHRASES = ["gear down", "deploy hardpoints", "request docking"];

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
  voice: { speaker: string; speed: number; colour: string };
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
  here: string | null;
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

function calloutsReply(st: State): Reply {
  return {
    ok: true,
    callouts: CALLOUTS.map((c) => ({
      ...c,
      enabled: st.callouts[c.id]?.enabled ?? c.default_enabled,
      cooldown_s: st.callouts[c.id]?.cooldown_s ?? c.default_cooldown_s,
    })),
  };
}

function prospectorReply(st: State): Reply {
  const ores = Object.keys(PROSPECTOR_DEFAULTS).map((ore) => ({
    ore,
    name: PROSPECTOR_NAMES[ore],
    default: PROSPECTOR_DEFAULTS[ore],
    threshold: st.prospector[ore],
  }));
  return { ok: true, ores };
}

export function fakeScenario(): Scenario | null {
  if (typeof location === "undefined") return null;
  const s = new URLSearchParams(location.search).get("core");
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
    previews: [],
    prospector: { ...PROSPECTOR_DEFAULTS },
    callouts: {},
    eddn: false,
    edsm: { enabled: false, commander: "", key: false },
    route: null,
    here: done ? "Sol" : null,
  };
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
    } else out.push({ system, ...(w.label ? { label: w.label } : {}) });
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

/** The preview's clipboard always takes the text. */
function routeView(st: State): RouteView {
  const r = st.route;
  return {
    route: r ? structuredClone(r) : null,
    here: st.here,
    next: r?.waypoints[r.next] ?? null,
    remaining: r ? r.waypoints.length - r.next : 0,
    clipboard_error: null,
  };
}

function routeCall(st: State, cmd: CoreCmd, args: Record<string, unknown>): Reply {
  const r = st.route;
  switch (cmd) {
    case "route_set": {
      const waypoints = cleanStops(typeof args.text === "string" ? parseList(args.text) : ((args.waypoints as Waypoint[] | undefined) ?? []));
      if (!waypoints.length) return { ok: false, error: "the route has no systems" };
      const first = waypoints[0];
      const next = st.here && first && sameSystem(first.system, st.here) ? 1 : 0;
      st.route = { name: String(args.name || "Route"), source: String(args.source ?? ""), waypoints, next };
      break;
    }
    case "route_skip":
      if (r) r.next = Math.min(r.next + 1, r.waypoints.length);
      break;
    case "route_back":
      if (r) r.next = Math.max(r.next - 1, 0);
      break;
    case "route_clear":
      st.route = null;
      break;
  }
  return { ok: true, view: routeView(st) };
}

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

export function fakeStatus(): CoreStatus {
  const st = s();
  const setup = [...(st.ptt ? [] : ["push-to-talk not set"]), ...(st.models ? [] : ["speech models not downloaded"])];
  return {
    ok: true,
    version: "0.1.5",
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
    app_dir: "C:\\Users\\you\\AppData\\Local\\astromech\\data",
    log_dir: "C:\\Users\\you\\AppData\\Local\\astromech\\data\\logs",
    config: "C:\\Users\\you\\AppData\\Local\\astromech\\data\\astromech.toml",
    config_note: null,
    setup_needed: setup,
    ptt: st.ptt,
    kill_switch: st.kill,
    mic: st.mic,
    onboarded: st.onboarded,
    models_missing: !st.models,
    models_progress: st.modelsProgress,
    models_dir: "C:\\Users\\you\\AppData\\Local\\astromech\\data\\models",
  };
}

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

function level(device: string, speech: boolean) {
  return speech
    ? { device, peak_dbfs: -24.3, noise_dbfs: -63.8, verdict: "ok" }
    : { device, peak_dbfs: -61.2, noise_dbfs: -64.0, verdict: "silent" };
}

function step(n: number) {
  if (n === 0) return { step: 0, steps: 4, phrase: "", ms: 2000 };
  if (n <= PHRASES.length) return { step: n, steps: 4, phrase: PHRASES[n - 1], ms: 2500 };
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

export async function fakeCall(cmd: CoreCmd, args?: unknown): Promise<Reply> {
  const st = s();
  const a = (args ?? {}) as Record<string, string | null>;
  switch (cmd) {
    case "route":
    case "route_set":
    case "route_skip":
    case "route_back":
    case "route_clear":
    case "route_copy":
      return routeCall(st, cmd, (args ?? {}) as Record<string, unknown>);
    case "find_elite":
      return {
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
      };
    case "set_folders":
      if (a.bindings_dir) {
        st.bindingsDir = a.bindings_dir;
        // The preview pretends the folder the user typed exists.
        st.elite = true;
      }
      return { ok: true, applies_next_session: false };
    case "set_ptt":
    case "set_kill_switch": {
      const b = args ? `${a.device} ${a.key}` : null;
      if (cmd === "set_ptt") st.ptt = b;
      else st.kill = b;
      return { ok: true, applies_next_session: false };
    }
    case "set_mic":
      st.mic = a.mic ?? "";
      return { ok: true, applies_next_session: false };
    case "mic_list": {
      const c = chosen(st);
      return {
        ok: true,
        names: st.mics,
        default: st.mics[0] ?? null,
        setting: st.mic,
        chosen: st.mic ? c : null,
        calibration: { device: c ?? (st.mic || "default"), min_level_dbfs: c ? (st.gates[c] ?? null) : null },
      };
    }
    case "mic_check": {
      const c = chosen(st);
      if (!c) return { ok: false, error: NO_MIC };
      await sleep(150);
      return { ok: true, level: level(c, true) };
    }
    case "calibrate_start":
      st.run = 0;
      return { ok: true, phrases: PHRASES, next: step(0) };
    case "calibrate_record": {
      if (st.run === null) return { ok: false, error: "calibration hasn't started" };
      const c = chosen(st);
      if (!c) return { ok: false, error: NO_MIC };
      if (st.run > PHRASES.length) return { ok: false, error: "calibration already has every clip" };
      await sleep(150);
      const lv = level(c, st.run > 0);
      st.run++;
      return { ok: true, level: lv, next: step(st.run) };
    }
    case "calibrate_finish": {
      const c = chosen(st);
      if (st.run === null || st.run <= PHRASES.length || !c) return { ok: false, error: `calibration needs silence and all ${PHRASES.length} phrases first` };
      st.gates[c] = -51;
      st.run = null;
      return { ok: true, device: c, noise_dbfs: -64, speech_peak_dbfs: -24.3, min_level_dbfs: -51, gain_db: 18.3 };
    }
    case "calibrate_cancel":
      st.run = null;
      return { ok: true };
    case "download_models":
      st.modelsProgress = "Parakeet Downloading 0%";
      setTimeout(() => {
        st.models = true;
        st.modelsProgress = null;
      }, 1500);
      return { ok: true };
    case "voice_settings":
      return { ok: true, ...st.voice, voices: VOICES, speeds: [0.7, 1.3] };
    case "set_voice": {
      if (a.speaker && !VOICES.some((v) => v.name === a.speaker)) return { ok: false, error: `unknown voice "${a.speaker}"` };
      const b = (args ?? {}) as Partial<State["voice"]>;
      st.voice = { ...st.voice, ...Object.fromEntries(Object.entries(b).filter(([, v]) => v != null)) };
      return { ok: true, applies_next_session: false };
    }
    case "voice_preview":
      if (!st.models) return { ok: false, error: "download the speech models first (Speech models, above)" };
      st.previews.push(st.voice.speaker);
      return { ok: true, line: "Landing gear deployed." };
    case "callouts":
      return calloutsReply(st);
    case "set_callout": {
      const c = (args ?? {}) as { id?: string; enabled?: boolean; cooldown_s?: number | null };
      if (!CALLOUTS.some((x) => x.id === c.id)) return { ok: false, error: `unknown callout "${c.id}"` };
      if (typeof c.cooldown_s === "number" && !(c.cooldown_s >= 0 && c.cooldown_s <= 86400)) {
        return { ok: false, error: "cooldown_s must be between 0 and 86400 seconds" };
      }
      const cur = (st.callouts[c.id as string] ??= {});
      if (typeof c.enabled === "boolean") cur.enabled = c.enabled;
      if (c.cooldown_s === null) delete cur.cooldown_s;
      else if (typeof c.cooldown_s === "number") cur.cooldown_s = c.cooldown_s;
      return { ok: true, applies_next_session: false };
    }
    case "prospector_thresholds":
      return prospectorReply(st);
    case "prospector_set": {
      const c = JSON.parse(typeof args === "string" ? args : "{}") as { ore: string; threshold?: number | null; reset?: boolean };
      if (!(c.ore in PROSPECTOR_DEFAULTS)) return { ok: false, error: `unknown ore "${c.ore}"` };
      st.prospector[c.ore] = c.reset ? (PROSPECTOR_DEFAULTS[c.ore] ?? null) : (c.threshold ?? null);
      return prospectorReply(st);
    }
    case "eddn_log":
      // Sharing was on earlier (the sample's history); the switch is this PC's setting.
      return { ok: true, ...eddnSample, enabled: st.eddn, pending: st.eddn ? eddnSample.pending : 0 };
    case "set_eddn":
      st.eddn = (args as { enabled?: boolean } | null)?.enabled === true;
      return { ok: true, applies_next_session: false };
    case "edsm_status": {
      const e = st.edsm;
      const uploading = e.enabled && e.key && e.commander !== "";
      return { ok: true, enabled: e.enabled, commander: e.commander, has_key: e.key, key_error: null, uploading,
        pending: 0, last_sent: null, last_error: null, log: [], skipped: [] };
    }
    case "set_edsm": {
      const ch = (args ?? {}) as { enabled?: boolean; commander?: string; api_key?: string };
      if (ch.enabled != null) st.edsm.enabled = ch.enabled;
      if (ch.commander != null) st.edsm.commander = ch.commander.trim();
      if (ch.api_key != null) st.edsm.key = ch.api_key.trim() !== "";
      return { ok: true, applies_next_session: false };
    }
    case "edsm_flight_log":
      if (!st.edsm.key || !st.edsm.commander) return { ok: false, error: "set your EDSM commander name and API key first" };
      return { ok: true, visits: [
        { system: "Sol", date: "2026-10-16 20:00:00", first_discover: false, ship_id: 7 },
        { system: "Achenar", date: "2026-10-16 19:10:00", first_discover: true, ship_id: 7 },
      ] };
    case "onboarding_done":
      st.onboarded = true;
      return { ok: true };
    case "exobiology":
      // Only a session (Elite running) has a system; `ready` is in one.
      return st.scenario === "ready" ? { ...biologySample } : { ok: false, error: "Elite isn't running." };
    default:
      return { ok: true };
  }
}
