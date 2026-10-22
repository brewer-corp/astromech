/**
 * Colonisation candidates near a system (astromech_galaxy::colonies): unpopulated systems nobody
 * has claimed within 15 ly of a populated one, where a claim can be made, with what their
 * bodies offer. The shell asks Spansh's systems and bodies searches. The browser preview
 * answers from a recorded search around Greae Phoea EM-K d9-675 (fixtures/colonies.json).
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import { previewLookup } from "./fakecore";
import sample from "./fixtures/colonies.json";
import type { Filters, Origin, Results } from "./search";

/** What the commander wants in the system (astromech_galaxy::Interest). */
export type Interest = "any" | "rings" | "earth_like" | "landable";

export const INTERESTS: { id: Interest; label: string }[] = [
  { id: "any", label: "Any" },
  { id: "rings", label: "Rings" },
  { id: "earth_like", label: "Earth-like world" },
  { id: "landable", label: "Landable" },
];

/** The radius used when none is picked (astromech_galaxy::colonies::DEFAULT_RADIUS_LY). */
export const DEFAULT_RADIUS_LY = 30;

/** The id the search kit remembers this lookup's filters under. */
export const LOOKUP = "colonisation.candidates";

export interface Bodies {
  /** Bodies Spansh knows (0: not scanned). */
  known: number;
  main_star: string | null;
  earth_like: number;
  water_worlds: number;
  ammonia_worlds: number;
  ringed: number;
  landable: number;
  terraformable: number;
}

export interface CandidateHit {
  name: string;
  id64: number;
  ly: number;
  jumps: number;
  exact: boolean;
  /** The populated system that puts it in range; no name when only Spansh's distance is known. */
  populated: { name: string | null; id64: number | null; ly: number };
  bodies: Bodies;
  updated_at: string | null;
  detail: string;
}

export interface Candidates {
  interest: Interest;
  here: { system: string; pos: [number, number, number] };
  range: number;
  radius_ly: number;
  fetched: string;
  hits: CandidateHit[];
  /** Unclaimed systems that couldn't be checked against every populated system nearby. */
  unchecked: number;
  warning?: string;
}

const samples = sample as unknown as Record<Interest, Candidates>;

/** Candidates with a body of `interest` near `origin` (a typed system name), or near the commander. */
export async function candidates(interest: Interest, origin: Origin = null, filters: Filters | null = null): Promise<Candidates> {
  if (!inTauri) {
    return previewLookup("colony_candidates", origin, () => {
      const s = samples[interest];
      return origin ? { ...s, here: { ...s.here, system: origin } } : s;
    });
  }
  return await invoke<Candidates>("colony_candidates", { interest, origin, filters });
}

/** The search kit's rows for a candidates answer. */
export function candidateResults(r: Candidates): Results {
  return {
    origin: r.here.system,
    range: r.range,
    source: "Spansh",
    fetched: r.fetched,
    warning: r.warning,
    rows: r.hits.map((h) => ({
      key: h.id64,
      system: h.name,
      ly: h.ly,
      jumps: h.jumps,
      exact: h.exact,
      updated_at: h.updated_at,
      detail: h.detail,
    })),
  };
}
