/**
 * Powerplay from the journal (the core replays it): the commander's standing and the current
 * system's Powerplay state; and the pledged power's nearby targets (Spansh, asked by the core).
 */
import type { Filters, Origin } from "./search";
import type { Overview } from "./api/types/Overview";
import type { PowerplayTarget } from "./api/types/PowerplayTarget";
import type { TargetHit } from "./api/types/TargetHit";
import type { Targets } from "./api/types/Targets";
import type { Work } from "./api/types/Work";
import { call } from "./transport";

export type { NextRank } from "./api/types/NextRank";
export type { StandingView as Standing } from "./api/types/StandingView";
/** What a system offers the pledged power. */
export type { Work } from "./api/types/Work";
export type { Suggestion } from "./api/types/Suggestion";
export type { RulesVersion } from "./api/types/RulesVersion";
export type { PowerplaySystemView as SystemPower } from "./api/types/PowerplaySystemView";
export type { Overview } from "./api/types/Overview";
export type { TargetHit } from "./api/types/TargetHit";
export type { Targets } from "./api/types/Targets";

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
  return await call("powerplay_overview");
}

/** "12,345". */
export function num(n: number | null | undefined): string {
  return n == null ? "–" : n.toLocaleString("en-GB");
}

/** How long the commander has been pledged: "37 weeks" from two weeks on, else "9 days" or "5 h". */
export function pledged(secs: number | null): string {
  if (secs == null) return "–";
  const d = Math.floor(secs / 86400);
  if (d >= 14) return `${Math.floor(d / 7)} weeks`;
  return d >= 1 ? `${d} day${d === 1 ? "" : "s"}` : `${Math.floor(secs / 3600)} h`;
}

/** The time from `now` to `iso` in the cycle's words: "2 d 14 h", "5 h 12 min", "12 min"; "0 min" once past. */
export function timeLeft(iso: string, now = Date.now()): string {
  const m = Math.max(0, Math.floor((Date.parse(iso) - now) / 60000));
  const d = Math.floor(m / 1440);
  const h = Math.floor((m % 1440) / 60);
  if (d) return `${d} d ${h} h`;
  return h ? `${h} h ${m % 60} min` : `${m} min`;
}

/** When `iso` falls, in UTC as the game's cycle is told: "Thursday 07:00 UTC". */
export function endsAt(iso: string): string {
  const t = new Date(iso);
  const day = t.toLocaleDateString("en-GB", { weekday: "long", timeZone: "UTC" });
  const hm = t.toLocaleTimeString("en-GB", { hour: "2-digit", minute: "2-digit", hourCycle: "h23", timeZone: "UTC" });
  return `${day} ${hm} UTC`;
}

/** Kinds of nearby target (astromech_galaxy::Target). */
export type Target = PowerplayTarget;

export const TARGETS: { id: Target; title: string; busy: string }[] = [
  { id: "acquire", title: "Acquire", busy: "Looking for unoccupied systems your power can take…" },
  { id: "reinforce", title: "Reinforce", busy: "Looking for your power's systems…" },
  { id: "undermine", title: "Undermine", busy: "Looking for other powers' systems…" },
];

/** The pledged power's `target` systems within `withinLy` of `origin` (or the commander). */
export async function loadTargets(target: Target, origin: Origin, filters: Filters | null, withinLy: number): Promise<Targets> {
  return await call("powerplay_targets", { target, origin, filters, within_ly: withinLy });
}

/**
 * The row's second line: state and progress, and how many other powers are active there besides
 * `power`, so the line has a bounded length whatever the system (their names are `targetOthers`,
 * for its Tip). The controlling power is named unless it is `power`. This cycle's reinforcement
 * and undermining are figures of their own (`TargetHit`).
 */
export function targetDetail(h: TargetHit, power: string): string {
  const parts: string[] = [];
  if (h.controlling_power) {
    const who = h.controlling_power === power ? "" : h.controlling_power;
    const state = h.state ? (who ? h.state.toLowerCase() : h.state) : "";
    parts.push(`${who} ${state} ${pct(h.control_progress)}`.replace(/\s+/g, " ").trim());
  } else {
    parts.push(`Unoccupied, acquisition ${pct(h.acquisition)}`);
  }
  const n = targetOthers(h, power).length;
  if (n) parts.push(`${n} other power${n === 1 ? "" : "s"}`);
  return parts.join(" · ");
}

/** The other powers active in a target system besides `power`. */
export function targetOthers(h: TargetHit, power: string): string[] {
  return h.others.filter((p) => p !== power && p !== h.controlling_power);
}
