/**
 * The logbook: where the commander has been, from the journal history the core keeps
 * (crates/astromech-history, read by the shell's `logbook` command).
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import sample from "./fixtures/logbook.json";

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
  jump_dist: number | null;
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
