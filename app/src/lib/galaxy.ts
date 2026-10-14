/**
 * Live "nearest systems" lookups (Spansh faction states) with jump counts from the local star
 * map. The shell does the network and the maths.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import sample from "./fixtures/nearby.json";

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
}

export interface Nearby {
  query: { id: string; label: string };
  here: { system: string; pos: [number, number, number] };
  range: number;
  fetched: string;
  hits: NearbyHit[];
  warning?: string;
}

export async function nearby(query: string): Promise<Nearby> {
  if (!inTauri) return { ...(sample as unknown as Nearby), query: { id: query, label: (sample as unknown as Nearby).query.label } };
  return await invoke<Nearby>("galaxy_nearby", { query });
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
