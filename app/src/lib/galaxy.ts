/**
 * Live "nearest systems" lookups (Spansh faction states) with jump counts from the local star
 * map. The shell does the network and the maths.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import { previewGate } from "./fakecore";
import sample from "./fixtures/nearby.json";
import hgeSample from "./fixtures/nearby-hge.json";
import type { Filters, Origin } from "./search";

export interface Faction {
  name: string;
  allegiance: string;
  state: string | null;
  active_states: string[];
  influence: number;
}

export interface NearbyHit {
  name: string;
  id64: number;
  ly: number;
  jumps: number;
  exact: boolean;
  population: number;
  updated_at: string | null;
  factions: Faction[];
  /** The factions and `updated_at` come from the commander's own arrival (journal), newer than Spansh's. */
  seen_on_visit?: boolean;
  /** High Grade Emissions lookups: population and distance score, 0 to about 150 (100 = a billion people here). */
  score?: number;
}

export interface Nearby {
  query: { id: string; label: string };
  here: { system: string; pos: [number, number, number] };
  range: number;
  fetched: string;
  hits: NearbyHit[];
  warning?: string;
}

/** Systems matching `query` and `filters` near `origin` (a typed system name), or near the commander. */
export async function nearby(query: string, origin: Origin = null, filters: Filters | null = null): Promise<Nearby> {
  if (!inTauri) {
    await previewGate("galaxy_nearby");
    const s = (query === "famine" ? sample : hgeSample) as unknown as Nearby;
    const days = filters?.max_age_days;
    const hits = days == null ? s.hits : s.hits.filter((h) => h.updated_at && Date.parse(s.fetched) - Date.parse(h.updated_at) <= days * 86400000);
    return { ...s, hits, query: { id: query, label: s.query.label }, here: origin ? { ...s.here, system: origin } : s.here };
  }
  return await invoke<Nearby>("galaxy_nearby", { query, origin, filters });
}

/** "2.0B", "85M", "35k", "uninhabited". */
export function population(n: number): string {
  if (n >= 1e9) return `${(n / 1e9).toFixed(1)}B`;
  if (n >= 1e6) return `${Math.round(n / 1e6)}M`;
  if (n >= 1e3) return `${Math.round(n / 1e3)}k`;
  return n > 0 ? String(n) : "uninhabited";
}

/** "3 h ago", "2 d ago". */
export function ago(iso: string | null, now = Date.now()): string {
  if (!iso) return "unknown";
  const s = Math.max(0, (now - Date.parse(iso)) / 1000);
  if (s < 90) return "just now";
  if (s < 5400) return `${Math.round(s / 60)} min ago`;
  if (s < 129600) return `${Math.round(s / 3600)} h ago`;
  return `${Math.round(s / 86400)} d ago`;
}

/** "3 jumps" (exact) or "~3 jumps" (straight-line estimate). */
export function jumps(n: number, exact: boolean): string {
  return `${exact ? "" : "~"}${n} jump${n === 1 ? "" : "s"}`;
}
