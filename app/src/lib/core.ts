/**
 * The core's status and commands, through the shell's Tauri commands (which talk to the core's
 * local control API). Outside Tauri (vite dev, mockups) everything reports "not connected",
 * unless `?core=fresh` or `?core=ready` picks a recorded core (fakecore.ts) for the preview.
 */
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { fakeCall, fakeScenario, fakeStatus } from "./fakecore";

export interface UpdateStatus {
  status: "idle" | "checking" | "available" | "downloading" | "ready" | "applying" | "error";
  installed: string;
  latest: string | null;
  notes: string;
  error: string | null;
  last_check: string | null;
  ready: boolean;
  /** Why the previous install failed, if it did. */
  apply_error?: string | null;
}

export interface CoreStatus {
  ok: boolean;
  version: string;
  pid: number;
  game: "not_running" | "launcher" | "running";
  active: boolean;
  starting: boolean;
  voice: string;
  voice_paused: boolean;
  killed: boolean;
  autostart: boolean;
  crash_pending: boolean;
  status_line: string;
  app_dir: string;
  log_dir: string;
  config: string;
  config_note: string | null;
  /** What still has to be set up before voice works, e.g. "push-to-talk not set". */
  setup_needed?: string[];
  /** Push-to-talk as "<device> <key>", or null. */
  ptt?: string | null;
  /** Kill switch as "<device> <key>", or null. */
  kill_switch?: string | null;
  /** The `[voice] mic` setting: a device name or part of one; empty is the Windows default. */
  mic?: string;
  /** Onboarding finished or skipped (absent from cores older than onboarding). */
  onboarded?: boolean;
  models_missing?: boolean;
  /** Download progress while speech models download. */
  models_progress?: string | null;
  models_dir?: string;
  update?: UpdateStatus;
  /** The journal history the core keeps (backfill, then new lines). */
  history?: HistoryProgress | null;
}

export interface HistoryProgress {
  running: boolean;
  /** The first import since the core started has finished. */
  ready: boolean;
  files: number;
  lines: number;
  bytes: number;
  bytes_total: number;
  events: number;
  error: string | null;
}

export const inTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export async function coreStatus(): Promise<CoreStatus | null> {
  if (!inTauri) return fakeScenario() ? fakeStatus() : null;
  try {
    return await invoke<CoreStatus | null>("core_status");
  } catch {
    return null;
  }
}

/** Commands the window may send; the shell's `core_call` allows exactly these. */
export type CoreCmd =
  | "save_diagnostics"
  | "ack_crash"
  | "check_update"
  | "install_update"
  | "autostart_on"
  | "autostart_off"
  | "pause_voice"
  | "resume_voice"
  | "kill_switch"
  | "download_models"
  | "open_config"
  | "voice_test"
  | "voice_test_poll"
  | "voice_test_stop"
  | "voice_test_live"
  // Onboarding and Settings → Microphone (core: setup.rs, calibration.rs).
  | "find_elite"
  | "set_ptt"
  | "set_kill_switch"
  | "set_mic"
  | "set_folders"
  | "mic_list"
  | "mic_check"
  | "calibrate_start"
  | "calibrate_record"
  | "calibrate_finish"
  | "calibrate_cancel"
  | "onboarding_done"
  // Settings → Voice responses.
  | "set_voice"
  | "voice_settings"
  | "voice_preview";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type Reply = { ok: boolean; error?: string; [k: string]: any };

/**
 * A string is the voice tester's phrase ("voice_test") or command id ("voice_test_live"); an
 * object is a setup command's arguments (e.g. `set_ptt` with `{device, key}`).
 */
export async function coreCall(cmd: CoreCmd, arg?: string | Record<string, unknown> | null): Promise<Reply> {
  const text = typeof arg === "string" ? arg : undefined;
  const args = typeof arg === "string" ? null : (arg ?? null);
  if (!inTauri) return fakeScenario() ? fakeCall(cmd, args) : { ok: false, error: "not running inside the app" };
  return await invoke<Reply>("core_call", { cmd, text, args });
}

/** A button from the config as the status shows it ("231D3201 Joy_4"). */
export function parseButton(s: string | null | undefined): { device: string; key: string } | null {
  const [device, key] = (s ?? "").split(" ");
  return device && key ? { device, key } : null;
}

/** Where Elite keeps what astromech reads (core setup.rs `Found`). */
export interface Found {
  bindings_dir: string | null;
  bindings_found: boolean;
  journal_dir: string | null;
  journal_found: boolean;
  install: string | null;
  install_source: string | null;
  ok: boolean;
}

export interface Level { device: string; peak_dbfs: number; noise_dbfs: number; verdict: "ok" | "quiet" | "silent" }

export type Colour = "natural" | "cockpit" | "harsh";
export interface VoiceChoice { name: string; label: string; accent: "British" | "American"; gender: "female" | "male" }
export interface VoiceSettings { speaker: string; speed: number; colour: Colour; voices: VoiceChoice[]; speeds: [number, number] }

export interface MicList {
  names: string[];
  default: string | null;
  setting: string;
  chosen: string | null;
  calibration: { device: string; min_level_dbfs: number | null };
}

export async function shellInfo(): Promise<{ version: string }> {
  if (!inTauri) return { version: "dev" };
  return await invoke<{ version: string }>("shell_info");
}

export async function openUrl(url: string): Promise<void> {
  if (!inTauri) {
    window.open(url, "_blank");
    return;
  }
  await invoke("open_external", { target: url });
}

/** The tray (or a second start) asks for a page: the shell emits "route". */
export function onRoute(cb: (route: string) => void): void {
  if (!inTauri) return;
  void listen<string>("route", (e) => cb(e.payload));
}

/** One-line summary for the header. */
export function headerState(s: CoreStatus | null): string {
  if (!s) return "Not running";
  if (s.starting) return "Starting voice…";
  const game = s.game === "running" ? "Elite running" : s.game === "launcher" ? "Launcher open" : "Waiting for Elite";
  if (!s.active) return game;
  const voice = s.voice_paused ? "Voice paused" : s.voice === "voice ready" ? "Voice ready" : "Voice off";
  return `${game} · ${voice}${s.killed ? " · Kill switch on" : ""}`;
}
