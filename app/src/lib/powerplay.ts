/**
 * Powerplay from the journal (the shell replays it): the commander's standing and the current
 * system's Powerplay state; and the pledged power's nearby targets (Spansh, through the shell).
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import { previewData } from "./fakecore";
import sample from "./fixtures/powerplay.json";
import sampleTargets from "./fixtures/powerplay-targets.json";
import type { Filters, Origin } from "./search";

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

/** Something that earns the pledged power merits here (astromech_powerplay::Suggestion). */
export interface Suggestion {
  /** Stable id from the rules table, e.g. "bounty-hunting". */
  activity: string;
  name: string;
  how: string;
  /** On foot (Odyssey). */
  odyssey: boolean;
  /** The pledged power's ethos favours it: bonus merits. */
  favoured: boolean;
  /** The controlling power is weak to it (undermining). */
  weak_spot: boolean;
  /** Why the rule isn't verified in game, if it isn't. */
  uncertain: string | null;
}

/** The activity rules table the suggestions come from. */
export interface RulesVersion {
  version: number;
  /** YYYY-MM-DD. */
  fetched: string;
}

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
  /** What earns the pledged power merits here; empty when not pledged or nothing does. */
  suggestions: Suggestion[];
  activity_rules: RulesVersion;
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
  if (!inTauri) {
    const o = sample as unknown as Overview;
    const unpledged: Overview = { standing: { power: null, rank: null, merits: null, time_pledged_secs: null, session_merits: 0, week_merits: 0, week_ends: o.standing.week_ends, next: null }, here: null, system: null };
    return previewData("powerplay_overview", () => o, () => unpledged);
  }
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

/** Kinds of nearby target (astromech_galaxy::Target). */
export type Target = "acquire" | "reinforce" | "undermine";

export const TARGETS: { id: Target; title: string; busy: string }[] = [
  { id: "acquire", title: "Acquire", busy: "Looking for unoccupied systems your power can take…" },
  { id: "reinforce", title: "Reinforce", busy: "Looking for your power's systems…" },
  { id: "undermine", title: "Undermine", busy: "Looking for other powers' systems…" },
];

export interface TargetHit {
  name: string;
  id64: number;
  ly: number;
  jumps: number;
  exact: boolean;
  controlling_power: string | null;
  state: string | null;
  control_progress: number | null;
  reinforcement: number | null;
  undermining: number | null;
  /** Unoccupied systems: the pledged power's acquisition progress. */
  acquisition: number | null;
  others: string[];
  updated_at: string | null;
}

export interface Targets {
  target: Target;
  power: string;
  here: { system: string; pos: [number, number, number] };
  range: number;
  within_ly: number;
  fetched: string;
  hits: TargetHit[];
  warning?: string;
}

/** The pledged power's `target` systems within `withinLy` of `origin` (or the commander). */
export async function loadTargets(target: Target, origin: Origin, filters: Filters | null, withinLy: number): Promise<Targets> {
  if (!inTauri) {
    const s = (sampleTargets as unknown as Record<Target, Targets>)[target];
    const shown = (hits: TargetHit[]) => ({ ...s, hits, within_ly: withinLy, here: origin ? { ...s.here, system: origin } : s.here });
    return previewData("powerplay_targets", () => shown(s.hits.filter((h) => h.ly <= withinLy)), () => shown([]));
  }
  return await invoke<Targets>("powerplay_targets", { target, origin, filters, withinLy });
}

/** The row detail: state and progress, and which other powers are there besides `power`. */
export function targetDetail(h: TargetHit, power: string): string {
  const parts: string[] = [];
  if (h.controlling_power) {
    parts.push(`${h.controlling_power} ${h.state?.toLowerCase() ?? ""} ${pct(h.control_progress)}`.replace(/\s+/g, " ").trim());
    if (h.reinforcement != null && h.undermining != null) parts.push(`reinforcement ${num(h.reinforcement)} / undermining ${num(h.undermining)}`);
  } else {
    parts.push(`Unoccupied, acquisition ${pct(h.acquisition)}`);
  }
  const others = h.others.filter((p) => p !== power);
  if (others.length) parts.push(`also ${others.join(", ")}`);
  return parts.join(" · ");
}
