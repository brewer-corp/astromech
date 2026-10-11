/**
 * The core's status and commands, through the shell's Tauri commands (which talk to the core's
 * local control API). Outside Tauri (vite dev, mockups) everything reports "not connected".
 */
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

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

export async function coreCall(cmd: string): Promise<Record<string, unknown>> {
  if (!inTauri) return { ok: false, error: "not running inside the app" };
  return await invoke<Record<string, unknown>>("core_call", { cmd });
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
  if (!s) return "Core not running";
  if (s.starting) return "Starting voice…";
  const game = s.game === "running" ? "Elite running" : s.game === "launcher" ? "Launcher open" : "Waiting for Elite";
  if (!s.active) return game;
  const voice = s.voice_paused ? "Voice paused" : s.voice === "voice ready" ? "Voice ready" : "Voice off";
  return `${game} · ${voice}${s.killed ? " · Kill switch on" : ""}`;
}
