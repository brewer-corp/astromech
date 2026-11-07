/**
 * Exobiology: the current system's biology (FSS counts, DSS genus lists, the species predicted
 * from each body's scan, ranked by value), what other players reported for it (Spansh), and the
 * sampling in progress. From the core's `exobiology` command (crates/astromech-core/src/biology.rs),
 * live while Elite runs. The browser preview shows a synthetic sample.
 */
import type { Biology } from "./api/types/Biology";
import { coreCall } from "./core";
import type { Sampled } from "./transport";

export type { BodyBio } from "./api/types/BodyBio";
export type { Candidate } from "./api/types/Candidate";
export type { Clearance } from "./api/types/Clearance";
export type { GenusBio } from "./api/types/GenusBio";
export type { KnownView as KnownBody } from "./api/types/KnownView";
export type { SamplingView as Sampling } from "./api/types/SamplingView";
/** The system's biology; null `known` until the lookup answers. */
export type BiologyView = Sampled<Biology>;

export async function loadBiology(): Promise<BiologyView> {
  const r = await coreCall("exobiology");
  if (!r.ok) throw new Error(r.error ?? "the core didn't answer");
  return r;
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
