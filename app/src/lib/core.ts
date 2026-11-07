/**
 * The core's status and commands, through the shell's Tauri commands (which talk to the core's
 * local control API), over the transport (transport.ts). In the browser preview everything
 * reports "not connected", unless `?core=new` or `?core=ready` picks a recorded core
 * (app/src/preview/fakecore.ts).
 */
import type { MiningRun } from "./mining";
import { call, subscribe, type CoreCmd } from "./transport";

export type { CoreCmd } from "./transport";

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
  /** The last few callout toasts, oldest first. */
  toasts?: Toast[];
  /** The journal history the core keeps (backfill, then new lines). */
  history?: HistoryProgress | null;
  /** The current mining run; null while no session is running. */
  mining?: MiningRun | null;
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

/** One callout shown as a toast. `id` increases by one per toast within a core session. */
export interface Toast {
  id: number;
  rule: string;
  text: string;
  time: string;
}

/**
 * Toasts newer than `lastSeen`, and the new `lastSeen`. The first poll (`lastSeen` null) shows
 * nothing, so opening the window doesn't replay old callouts. Ids restart with a new core
 * session; a newest id below `lastSeen` means that happened, and everything listed is new.
 */
export function freshToasts(list: Toast[], lastSeen: number | null): { show: Toast[]; last: number } {
  const newest = list.at(-1)?.id ?? 0;
  if (lastSeen === null) return { show: [], last: newest };
  const from = newest < lastSeen ? 0 : lastSeen;
  return { show: list.filter((t) => t.id > from), last: newest };
}

export async function coreStatus(): Promise<CoreStatus | null> {
  try {
    return await call<CoreStatus | null>("core_status");
  } catch {
    return null;
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type Reply = { ok: boolean; error?: string; [k: string]: any };

/**
 * A string is the voice tester's phrase ("voice_test"), command id ("voice_test_live") or a
 * prospector threshold change as JSON ("prospector_set"); an object is a setup command's
 * arguments (e.g. `set_ptt` with `{device, key}`).
 */
export function coreCall(cmd: CoreCmd, arg?: string | Record<string, unknown> | null): Promise<Reply> {
  return call<Reply>(cmd, arg ?? null);
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
  return await call<{ version: string }>("shell_info");
}

export async function openUrl(url: string): Promise<void> {
  await call("open_external", { target: url });
}

/** The tray (or a second start) asks for a page: the shell emits "route". */
export function onRoute(cb: (route: string) => void): () => void {
  return subscribe<string>("route", cb);
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
