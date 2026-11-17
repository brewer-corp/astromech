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

/** A species on a card: its name, its value, and whether other players reported it there. */
export type CardSpecies = { name: string; value: number | null; reported: boolean };

/** A genus on a body card. */
export type CardGenus = {
  genus: string;
  /** From the DSS or a sample (or reported by others); false: predicted from the scan. */
  known: boolean;
  /** Only other players reported it; the commander has nothing on it. */
  reported: boolean;
  /** Other players reported it (as well as the commander, or alone). */
  others: boolean;
  species: CardSpecies[];
  /** What it could be worth, lowest to highest; null for a reported genus with no species. */
  value: [number, number] | null;
  analysed: boolean;
};

/** One body with biology: the commander's, with what others reported merged in, or others' alone. */
export type BodyCard = {
  /** The full body name: the card's key. */
  name: string;
  body_id: number | null;
  signals: number | null;
  /** The commander's range; for a body only others reported, theirs. */
  value: [number, number];
  /** What others reported it is worth, where they reported the body and it differs from `value`. */
  reported_value: [number, number] | null;
  genera: CardGenus[];
  /** Only other players reported the body. */
  reported: boolean;
};

type Body = import("./api/types/BodyBio").BodyBio;
type Known = import("./api/types/KnownView").KnownView;

/** The same body: by id when both have one, else by full name. A null id never matches another. */
const same = (b: Body, k: Known) => (b.body_id !== null && k.body_id !== null ? b.body_id === k.body_id : b.name === k.name);

/**
 * Whether a species is of a genus: "Stratum Tectonicas" is a Stratum, "Roseum Brain Tree" a Brain
 * Tree, "Amphora Plant" an Amphora Plant (the catalogue names some species after their genus).
 */
export const ofGenus = (species: string, genus: string) => species === genus || species.startsWith(`${genus} `) || species.endsWith(` ${genus}`);

/** A reported species' genus, from the names the body lists (the core lists every species' genus). */
const genusOf = (species: string, genera: string[]) => genera.find((g) => ofGenus(species, g)) ?? species.split(" ")[0] ?? species;

const span = (vs: number[]): [number, number] | null => (vs.length ? [Math.min(...vs), Math.max(...vs)] : null);

/** A reported body's genera, each with the species reported for it. */
function reportedGenera(k: Known): CardGenus[] {
  const names = [...k.genera];
  for (const s of k.species) if (!names.includes(genusOf(s.name, names))) names.push(genusOf(s.name, names));
  return names.map((genus) => {
    const species = k.species.filter((s) => genusOf(s.name, names) === genus).map((s) => ({ name: s.name, value: s.value, reported: true }));
    return { genus, known: true, reported: true, others: true, species, value: span(species.map((s) => s.value!)), analysed: false };
  });
}

/**
 * The page's cards: the commander's bodies in the reply's order (most valuable first), each with
 * what other players reported for it added (a genus or species only they reported, and their
 * value range); then the bodies only others reported, in their order. `known` null (not asked,
 * or the lookup failed) gives the commander's cards alone.
 */
export function bodyCards(bodies: Body[], known: Known[] | null): BodyCard[] {
  const reports = known ?? [];
  const used = new Set<Known>();
  const mine = bodies.map((b): BodyCard => {
    const k = reports.find((r) => !used.has(r) && same(b, r));
    if (k) used.add(k);
    const genera: CardGenus[] = b.genera.map((g) => ({
      genus: g.genus,
      known: g.known,
      reported: false,
      others: false,
      species: g.species.map((c) => ({ name: c.species, value: c.value, reported: false })),
      value: g.value,
      analysed: g.analysed,
    }));
    if (k) {
      for (const r of reportedGenera(k)) {
        const g = genera.find((x) => x.genus === r.genus);
        if (!g) {
          genera.push(r);
          continue;
        }
        g.others = true;
        for (const s of r.species) {
          const mine = g.species.find((x) => x.name === s.name);
          if (mine) {
            mine.reported = true;
            mine.value = s.value;
          } else g.species.push(s);
        }
      }
    }
    const differs = k && (k.value[0] !== b.value[0] || k.value[1] !== b.value[1]);
    return { name: b.name, body_id: b.body_id, signals: b.signals ?? k?.signals ?? null, value: b.value, reported_value: differs ? k.value : null, genera, reported: false };
  });
  const theirs = reports
    .filter((k) => !used.has(k))
    .map((k): BodyCard => ({ name: k.name, body_id: k.body_id, signals: k.signals, value: k.value, reported_value: null, genera: reportedGenera(k), reported: true }));
  return [...mine, ...theirs];
}
