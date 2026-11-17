/**
 * The logbook: where the commander has been, from the journal history the core keeps
 * (crates/astromech-history, read by the core's `logbook` command).
 */
import type { Logbook as Book } from "./api/types/Logbook";
import type { Session as Played } from "./api/types/Session";
import type { Payment } from "./api/types/Payment";
import type { Visit } from "./api/types/Visit";
import { call, type Sampled } from "./transport";

export type { Commander } from "./api/types/Commander";
export type { Payment } from "./api/types/Payment";
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

/** What a session's earnings count, in the Tip of the figure that shows them (Session, Now). */
export const EARNED =
  "Credits paid for what you did: trade sales, data, vouchers handed in, mission rewards, rescue, community goals, Odyssey goods and Powerplay salary. Selling ships, modules or drones isn't counted.";

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

/** One day of the logbook's timeline: its date (UTC) and its arrivals and dockings, as given. */
export interface Day {
  /** "2026-09-03", the UTC date of the journal's timestamps. */
  date: string;
  /** FSD jumps that day; carrier jumps and logins (`Location`) are not the commander's jumps. */
  jumps: number;
  dockings: number;
  visits: Visit[];
}

/** The timeline grouped by its UTC day, in the order given (the core sends newest first). */
export function byDay(timeline: Visit[]): Day[] {
  const days: Day[] = [];
  for (const v of timeline) {
    const date = v.ts.slice(0, 10);
    let day = days.at(-1);
    if (day?.date !== date) days.push((day = { date, jumps: 0, dockings: 0, visits: [] }));
    day.visits.push(v);
    if (v.kind === "station") day.dockings++;
    else if (v.via === "FSDJump") day.jumps++;
  }
  return days;
}

/** "10:25" from a journal timestamp. */
export function clock(ts: string): string {
  return ts.slice(11, 16);
}

/** A mark on the session's band: one kind's payments in one minute, the precision its Tip shows. */
export interface Mark {
  /** The first payment's time. */
  ts: string;
  payments: Payment[];
  amount: number;
}

/**
 * The session's band: a row per kind in `kinds`' order (the totals'), each with its marks in
 * time order. Payments of a kind in the same minute share a mark, since they would draw on the
 * same spot and the later would hide the earlier (selling several goods at once pays together).
 */
export function bandRows(kinds: string[], timeline: Payment[]): { kind: string; marks: Mark[] }[] {
  return kinds.map((kind) => {
    const marks: Mark[] = [];
    for (const p of timeline) {
      if (p.kind !== kind) continue;
      const last = marks.at(-1);
      if (last && last.ts.slice(0, 16) === p.ts.slice(0, 16)) {
        last.payments.push(p);
        last.amount += p.amount;
      } else marks.push({ ts: p.ts, payments: [p], amount: p.amount });
    }
    return { kind, marks };
  });
}
