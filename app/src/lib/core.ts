/**
 * The core's status and commands, through the shell's Tauri commands (which talk to the core's
 * local control API). Outside Tauri (vite dev, mockups) everything reports "not connected".
 */
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

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
  models_missing?: boolean;
  /** Download progress while speech models download. */
  models_progress?: string | null;
  models_dir?: string;
  update?: UpdateStatus;
  /** The last few callout toasts, oldest first. */
  toasts?: Toast[];
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

export const inTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export async function coreStatus(): Promise<CoreStatus | null> {
  if (!inTauri) return null;
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
  | "voice_test_live";

/** `text` is the voice tester's phrase ("voice_test") or command id ("voice_test_live"). */
export async function coreCall(cmd: CoreCmd, text?: string): Promise<Record<string, unknown>> {
  if (!inTauri) return { ok: false, error: "not running inside the app" };
  return await invoke<Record<string, unknown>>("core_call", { cmd, text });
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
