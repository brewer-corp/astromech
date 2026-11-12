/**
 * The core's status and commands, over the transport (transport.ts): in the app, the shell's one
 * proxy to the core's local control API. In the browser preview everything
 * reports "not connected", unless `?core=new` or `?core=ready` picks a recorded core
 * (app/src/preview/fakecore.ts).
 */
import type { CoreStatus } from "./api/types/CoreStatus";
import type { Failed } from "./api/types/Failed";
import type { ShipStatus } from "./api/types/ShipStatus";
import type { Toast } from "./api/types/Toast";
import { call, subscribe, type ArgList, type Args, type Reply, type WindowCommand } from "./transport";

export type { Colour } from "./api/types/Colour";
export type { CoreStatus } from "./api/types/CoreStatus";
export type { Found } from "./api/types/Found";
export type { LevelCheck as Level } from "./api/types/LevelCheck";
export type { MicList } from "./api/types/MicList";
export type { ShipStatus } from "./api/types/ShipStatus";
export type { Progress as HistoryProgress } from "./api/types/Progress";
export type { Snapshot as UpdateStatus } from "./api/types/Snapshot";
export type { Toast } from "./api/types/Toast";
export type { VoiceChoice } from "./api/types/VoiceChoice";
export type { VoiceSettings } from "./api/types/VoiceSettings";

/**
 * The core's own commands (astromech-core's `Core::commands`): those whose replies carry `ok` and
 * report most failures inside (`{ ok: false, error }`, astromech_api::reply::Answer). `coreCall`
 * gives a failed call as the same reply.
 */
export type CoreCmd = { [K in WindowCommand]: Reply<K> extends { ok: boolean } ? K : never }[WindowCommand];
/** The core commands that take no args (`coreCall(cmd)` for any of them). */
export type BareCoreCmd = { [K in CoreCmd]: [Args<K>] extends [null] ? K : never }[CoreCmd];

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

/** The core's status, or null when the call fails: the core isn't running (or didn't answer). */
export async function coreStatus(): Promise<Reply<"status"> | null> {
  try {
    return await call("status");
  } catch {
    return null;
  }
}

/** The ship's fuel and cargo for the status bar, or null when the call fails. Every figure is
 * null while Elite isn't running. */
export async function shipStatus(): Promise<ShipStatus | null> {
  try {
    return await call("ship_status");
  } catch {
    return null;
  }
}

/**
 * Send a core command; a failed call gives `{ ok: false, error }` like the core's own failures.
 * A string argument is the voice tester's phrase ("voice_test"), command id ("voice_test_live")
 * or a prospector threshold change as JSON ("prospector_set").
 */
export async function coreCall<K extends CoreCmd>(cmd: K, ...args: ArgList<K>): Promise<Reply<K> | Failed> {
  try {
    return await call(cmd, ...args);
  } catch (e) {
    return { ok: false, error: String(e) };
  }
}

/** A button from the config as the status shows it ("231D3201 Joy_4"). */
export function parseButton(s: string | null | undefined): { device: string; key: string } | null {
  const [device, key] = (s ?? "").split(" ");
  return device && key ? { device, key } : null;
}

/** This window's own version, built into the bundle (vite.config.ts, from app/package.json, which
 * scripts/set-version.py stamps with the release's), to set against the core's. */
export const WINDOW_VERSION: string = __APP_VERSION__;

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
