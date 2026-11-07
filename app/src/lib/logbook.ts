/**
 * The logbook: where the commander has been, from the journal history the core keeps
 * (crates/astromech-history, read by the core's `logbook` command).
 */
import type { Logbook as Book } from "./api/types/Logbook";
import type { Session as Played } from "./api/types/Session";
import { call, type Sampled } from "./transport";

export type { Commander } from "./api/types/Commander";
export type { Place } from "./api/types/Place";
export type { Visit } from "./api/types/Visit";
export type Logbook = Sampled<Book>;
/** A play session; `latest`: no later session has started, so this may be the one being played. */
export type Session = Sampled<Played>;

export async function loadLogbook(query: string, cmdr?: string): Promise<Logbook> {
  return await call("logbook", { query, cmdr: cmdr ?? null });
}

/** "2026-09-03 10:25" from a journal timestamp (UTC, as the game writes it). */
export function when(ts: string): string {
  return ts.replace("T", " ").replace(/:\d\dZ$/, "");
}

/** The latest session. */
export async function loadSession(): Promise<Session | null> {
  return await call("session_summary");
}

/** How the earnings kinds read. */
export const EARNING_NAMES: Record<string, string> = {
  trade: "Trade",
  exploration: "Exploration data",
  organic: "Organic data",
  bounty: "Bounty vouchers",
  combat_bond: "Combat bonds",
  voucher: "Other vouchers",
  mission: "Missions",
  search_and_rescue: "Search and rescue",
  community_goal: "Community goals",
  micro_resources: "Odyssey goods",
  powerplay: "Powerplay salary",
};

/** "1 h 05 min" */
export function duration(secs: number): string {
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  return h ? `${h} h ${String(m).padStart(2, "0")} min` : `${m} min`;
}
