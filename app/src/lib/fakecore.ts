/**
 * Recorded cores for the browser preview and component tests (no Tauri, no Windows): what the
 * core's control API answers on a PC in one of three states, picked with `?core=<scenario>`.
 * - `empty`: Elite never ran here, no sticks, no microphone, nothing set up;
 * - `new`: a fresh install on a PC with Elite, two sticks and two microphones;
 * - `ready`: the same PC after onboarding.
 * Commands mutate the in-memory state the way the core would. Synthetic data only.
 */
import type { CoreCmd, CoreStatus, Reply } from "./core";

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
}

let state: State | null = null;

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
  };
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

const NO_MIC = "no recording device found (plug in a microphone or headset)";

export async function fakeCall(cmd: CoreCmd, args?: unknown): Promise<Reply> {
  const st = s();
  const a = (args ?? {}) as Record<string, string | null>;
  switch (cmd) {
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
    case "onboarding_done":
      st.onboarded = true;
      return { ok: true };
    default:
      return { ok: true };
  }
}
