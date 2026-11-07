/**
 * Colonisation candidates near a system (astromech_galaxy::colonies): unpopulated systems nobody
 * has claimed within 15 ly of a populated one, where a claim can be made, with what their
 * bodies offer. The core asks Spansh's systems and bodies searches. The browser preview
 * answers from a recorded search around Greae Phoea EM-K d9-675 (app/src/preview/fixtures/colonies.json).
 */
import type { Filters, Origin, Results } from "./search";
import type { Candidates } from "./api/types/Candidates";
import type { Interest } from "./api/types/Interest";
import { call } from "./transport";

export type { Bodies } from "./api/types/Bodies";
export type { CandidateHit } from "./api/types/CandidateHit";
export type { Candidates } from "./api/types/Candidates";
export type { Interest } from "./api/types/Interest";

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

/** Candidates with a body of `interest` near `origin` (a typed system name), or near the commander. */
export async function candidates(interest: Interest, origin: Origin = null, filters: Filters | null = null): Promise<Candidates> {
  return await call("colony_candidates", { interest, origin, filters });
}

/** The search kit's rows for a candidates answer. */
export function candidateResults(r: Candidates): Results {
  return {
    origin: r.here.system,
    range: r.range,
    source: "Spansh",
    fetched: r.fetched,
    warning: r.warning,
    place: "system",
    columns: [{ key: "detail", head: "Bodies", text: true }],
    rows: r.hits.map((h) => ({
      key: h.id64,
      system: h.name,
      ly: h.ly,
      jumps: h.jumps,
      exact: h.exact,
      updated_at: h.updated_at,
      facts: { detail: h.detail },
    })),
  };
}
