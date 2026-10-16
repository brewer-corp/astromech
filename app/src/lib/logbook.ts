/**
 * The logbook: where the commander has been, from the journal history the core keeps
 * (crates/astromech-history, read by the shell's `logbook` command).
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import sample from "./fixtures/logbook.json";
import sampleSession from "./fixtures/session.json";

export interface Commander {
  cmdr: string;
  name: string;
  legacy: boolean;
  first_seen: string;
  last_seen: string;
}

export interface Place {
  kind: "system" | "station";
  star_system: string;
  system_address: number | null;
  station_name: string | null;
  market_id: number | null;
  visits: number;
  first: string;
  last: string;
}

/** One arrival (`system`) or docking (`station`). */
export interface Visit {
  ts: string;
  kind: "system" | "station";
  star_system: string;
  system_address: number | null;
  station_name: string | null;
  market_id: number | null;
  /** Hyperspace jump distance (FSD jumps only). */
  jump_dist: number | null;
  via: "FSDJump" | "CarrierJump" | "Location" | "Docked" | string;
}

export interface Logbook {
  commander: Commander | null;
  commanders: Commander[];
  totals: { systems: number; stations: number; system_visits: number; station_visits: number };
  query: string;
  places: Place[];
  matches: number;
  timeline: Visit[];
  /** Where it came from, when not the live history. */
  source?: string;
}

const matches = (q: string) => (v: { star_system: string; station_name: string | null }) =>
  !q || v.star_system.toLowerCase().includes(q) || (v.station_name ?? "").toLowerCase().includes(q);

/** The sample history searched the way the core searches (outside the app). */
export function searchSample(query: string): Logbook {
  const book = sample as Logbook;
  const q = query.trim().toLowerCase();
  const places = book.places.filter(matches(q));
  return { ...book, query: query.trim(), places, matches: places.length, timeline: book.timeline.filter(matches(q)), source: "Sample journal history" };
}

export async function loadLogbook(query: string, cmdr?: string): Promise<Logbook> {
  if (!inTauri) return searchSample(query);
  return await invoke<Logbook>("logbook", { query, cmdr: cmdr ?? null });
}

/** "2026-09-03 10:25" from a journal timestamp (UTC, as the game writes it). */
export function when(ts: string): string {
  return ts.replace("T", " ").replace(/:\d\dZ$/, "");
}

export interface Session {
  commander: Commander;
  start: string;
  end: string;
  duration_secs: number;
  /** No later session has started: this may be the one being played. */
  latest: boolean;
  /** Credits by activity, largest first. */
  earnings: { kind: string; amount: number; count: number }[];
  total: number;
  trade_profit: number;
  per_hour: number | null;
  jumps: number;
  distance_ly: number;
  carrier_jumps: number;
  bounties_awarded: number;
  bounty_value: number;
  data_sold: number;
  source?: string;
}

/** The latest session (the sample's first session outside the app). */
export async function loadSession(): Promise<Session | null> {
  if (!inTauri) return { ...(sampleSession as Session), source: "Sample journal history" };
  return await invoke<Session | null>("session_summary");
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
