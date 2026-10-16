/**
 * Powerplay from the journal (the shell replays it): the commander's standing and the current
 * system's Powerplay state.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import sample from "./fixtures/powerplay.json";

export interface NextRank {
  rank: number;
  merits: number;
  to_go: number;
  reward: string;
  note: string | null;
}

export interface Standing {
  power: string | null;
  rank: number | null;
  merits: number | null;
  time_pledged_secs: number | null;
  session_merits: number;
  week_merits: number;
  /** When this weekly cycle ends (RFC 3339). */
  week_ends: string;
  next: NextRank | null;
}

/** What a system offers the pledged power. */
export type Work = "reinforce" | "undermine" | "acquire";

export interface SystemPower {
  system: string;
  controlling_power: string | null;
  /** As the journal writes it: Exploited, Fortified, Stronghold, Unoccupied. */
  state: string | null;
  /** Progress through the state's band, a fraction (may be below 0 or above 1). */
  control_progress: number | null;
  reinforcement: number | null;
  undermining: number | null;
  /** Powers other than the controller active here. */
  contesting: string[];
  /** Unoccupied systems: acquisition progress, highest first. */
  acquisition: { power: string; progress: number }[];
  work: Work | null;
  summary: string;
}

export interface Overview {
  standing: Standing;
  /** The current system (last jump or location). */
  here: string | null;
  /** Its Powerplay state; null when it has none. */
  system: SystemPower | null;
}

export const WORK_LABEL: Record<Work, string> = {
  reinforce: "Your power's system: reinforce",
  undermine: "Undermining target",
  acquire: "Acquisition target",
};

/** "28%". */
export function pct(f: number | null | undefined): string {
  return f == null ? "–" : `${Math.round(f * 100)}%`;
}

export async function loadOverview(): Promise<Overview> {
  if (!inTauri) return sample as unknown as Overview;
  return await invoke<Overview>("powerplay_overview");
}

/** "12,345". */
export function num(n: number | null | undefined): string {
  return n == null ? "–" : n.toLocaleString("en-GB");
}

/** "212 days", "5 h". */
export function pledged(secs: number | null): string {
  if (secs == null) return "–";
  const d = Math.floor(secs / 86400);
  return d >= 1 ? `${d} day${d === 1 ? "" : "s"}` : `${Math.floor(secs / 3600)} h`;
}

/** "3 d 4 h" until `iso`. */
export function until(iso: string, now = Date.now()): string {
  const s = Math.max(0, (Date.parse(iso) - now) / 1000);
  const d = Math.floor(s / 86400);
  const h = Math.floor((s % 86400) / 3600);
  return d ? `${d} d ${h} h` : `${h} h ${Math.floor((s % 3600) / 60)} min`;
}
