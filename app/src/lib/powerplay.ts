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
