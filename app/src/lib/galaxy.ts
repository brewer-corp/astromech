/**
 * Live "nearest systems" lookups (Spansh faction states) with jump counts from the local star
 * map. The core does the network and the maths.
 */
import type { Nearby } from "./api/types/Nearby";
import type { NearbyHit } from "./api/types/NearbyHit";
import { call } from "./transport";
import { figure, type Filters, type Origin, type Results } from "./search";

export type { Faction } from "./api/types/Faction";
export type { NearbyHit } from "./api/types/NearbyHit";
export type { Nearby } from "./api/types/Nearby";

/**
 * The faction-state lookups the core offers (astromech_galaxy::remote::QUERIES, same ids): the
 * High Grade Emissions pools, then wake scanning (Famine). `name` is the Faction states finder's
 * pick, `about` the line under it.
 */
export const FACTION_STATES = [
  { id: "fed_composites", name: "Federation faction", about: "High Grade Emissions for Federation composites, in systems over a billion people." },
  { id: "empire_shielding", name: "Empire faction", about: "High Grade Emissions for Imperial Shielding, in systems over a billion people." },
  { id: "proto", name: "Boom (Independent or Alliance)", about: "High Grade Emissions for Proto materials." },
  { id: "military", name: "War or Civil War", about: "High Grade Emissions for Military materials, any allegiance." },
  { id: "improvised", name: "Civil Unrest (Independent or Alliance)", about: "High Grade Emissions for Improvised Components." },
  { id: "pharma", name: "Outbreak (Independent or Alliance)", about: "High Grade Emissions for Pharmaceutical Isolators." },
  { id: "famine", name: "Famine", about: "Distribution centres, for wake scanning." },
] as const;

/** Systems matching `query` and `filters` near `origin` (a typed system name), or near the commander. */
export async function nearby(query: string, origin: Origin = null, filters: Filters | null = null): Promise<Nearby> {
  return await call("galaxy_nearby", { query, origin, filters });
}

/** The matching faction, its allegiance and states, and whether the commander's own visit said so. */
function factionFact(h: NearbyHit): string {
  const f = h.factions[0];
  const st = f ? (f.active_states.length ? f.active_states.join(", ") : (f.state ?? "")) : "";
  return [f ? `${f.name} (${f.allegiance}${st ? `, ${st}` : ""})` : "", h.seen_on_visit ? "seen on your visit" : ""].filter(Boolean).join(" · ");
}

/** The search kit's rows for a faction-state answer: HGE lookups ranked by the score, with the
 * population it weighs; faction states by jumps. */
export function nearbyResults(r: Nearby): Results {
  const scored = r.hits.some((h) => h.score != null);
  return {
    origin: r.here.system,
    range: r.range,
    source: scored ? "Spansh, ranked by population and distance" : "Spansh",
    fetched: r.fetched,
    warning: r.warning,
    label: r.query.label,
    place: "system",
    ...(scored ? { main: { head: "Score" } } : {}),
    columns: [...(scored ? [{ key: "pop", head: "Population", width: 96 }] : []), { key: "faction", head: "Faction", text: true }],
    rows: r.hits.map((h) => ({
      key: h.id64,
      system: h.name,
      ly: h.ly,
      jumps: h.jumps,
      exact: h.exact,
      updated_at: h.updated_at,
      ...(h.score == null ? {} : { main: figure(h.score) }),
      facts: { pop: population(h.population), faction: factionFact(h) },
    })),
  };
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
