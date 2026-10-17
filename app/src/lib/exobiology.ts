/**
 * Exobiology: the current system's biology (FSS counts, DSS genus lists, the species predicted
 * from each body's scan, ranked by value), what other players reported for it (Spansh), and the
 * sampling in progress. From the core's `exobiology` command (crates/astromech-core/src/biology.rs),
 * live while Elite runs. Outside the app it is a synthetic sample.
 */
import { coreCall, inTauri } from "./core";
import { fakeScenario, previewGate } from "./fakecore";
import sample from "./fixtures/exobiology.json";

export interface Candidate {
  genus: string;
  species: string;
  value: number;
  fit: number;
}

export interface GenusBio {
  genus: string;
  /** The DSS or a sample says it's there; false: predicted from the body's conditions. */
  known: boolean;
  species: Candidate[];
  value: [number, number];
  colony_range_m: number | null;
}

export interface BodyBio {
  body_id: number;
  name: string;
  signals: number | null;
  /** The genus list came from the DSS. */
  mapped: boolean;
  genera: GenusBio[];
  value: [number, number];
}

export interface KnownBody {
  name: string;
  body_id: number | null;
  signals: number;
  genera: string[];
  species: { name: string; value: number }[];
  value: [number, number];
}

export interface Clearance {
  next: number;
  nearest_m: number;
  bearing_deg: number;
  colony_range_m: number;
  far_enough: boolean;
}

export interface Sampling {
  genus: string;
  species: string;
  count: number;
  complete: boolean;
  colony_range_m: number | null;
  body_id: number | null;
  clearance: Clearance | null;
}

export interface BiologyView {
  system: string | null;
  bodies: BodyBio[];
  /** Reported by other players; null until the lookup answers. */
  known: KnownBody[] | null;
  sampling: Sampling | null;
  /** Set for the sample shown outside the app. */
  source?: string;
}

export async function loadBiology(): Promise<BiologyView> {
  if (!inTauri && !fakeScenario()) {
    await previewGate("exobiology");
    return { ...(sample as unknown as BiologyView), source: "Sample system (synthetic)" };
  }
  const r = await coreCall("exobiology");
  if (!r.ok) throw new Error(r.error ?? "the core didn't answer");
  return r as unknown as BiologyView;
}

/** "19 M", "1.4 M": credits in millions. */
export function millions(v: number): string {
  const m = v / 1e6;
  return m >= 9.95 ? `${Math.round(m)} M` : `${m.toFixed(1).replace(/\.0$/, "")} M`;
}

/** "19 M" or "1.4–19 M". */
export function valueRange([lo, hi]: [number, number]): string {
  return lo === hi ? millions(hi) : `${millions(lo).replace(" M", "")}–${millions(hi)}`;
}

/** "A 2" for "Synuefe BY-F d12-39 A 2" in that system. */
export function shortName(body: string, system: string | null): string {
  if (system && body.startsWith(system + " ")) return body.slice(system.length + 1).trim() || body;
  return body;
}
