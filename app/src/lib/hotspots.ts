/**
 * The Mining hotspots finder's lookup (astromech_market::mining): rings with a hotspot of a
 * mineral, of a ring type and with a reserve level, within the finder's radius of the commander,
 * the route's next stop or a typed system, most hotspots first. The shell asks Spansh's bodies
 * search (`mining_hotspots`, highest hotspot counts first, at most four requests); the browser
 * preview answers from a recorded Painite search within 100 ly of Sol (fixtures/hotspots.json,
 * the shell's own answer, 30 rings). Adapted into the search kit's rows by {@link hotspotResults}.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import { previewLookup } from "./fakecore";
import sample from "./fixtures/hotspots.json";
import { figure, type Filters, type Origin, type Results } from "./search";

/** The ores the hotspot lookups offer, most valuable first (astromech_journal::ore::VALUE_ORDER),
 * by journal symbol with the name the rings list them under. */
export const ORES = [
  { symbol: "opal", name: "Void Opal" },
  { symbol: "lowtemperaturediamond", name: "Low Temperature Diamonds" },
  { symbol: "painite", name: "Painite" },
  { symbol: "platinum", name: "Platinum" },
  { symbol: "musgravite", name: "Musgravite" },
  { symbol: "grandidierite", name: "Grandidierite" },
  { symbol: "alexandrite", name: "Alexandrite" },
  { symbol: "benitoite", name: "Benitoite" },
  { symbol: "monazite", name: "Monazite" },
  { symbol: "rhodplumsite", name: "Rhodplumsite" },
  { symbol: "serendibite", name: "Serendibite" },
  { symbol: "osmium", name: "Osmium" },
  { symbol: "palladium", name: "Palladium" },
  { symbol: "bromellite", name: "Bromellite" },
  { symbol: "tritium", name: "Tritium" },
] as const;

export type Ore = (typeof ORES)[number]["symbol"];

/** Ring types as Spansh names them. */
export const RING_TYPES = ["Metallic", "Metal Rich", "Rocky", "Icy"] as const;
export type RingType = (typeof RING_TYPES)[number];

/** The least reserve level (astromech_market::mining::Reserve): "major" is Major or Pristine. */
export type Reserve = "pristine" | "major";

/** What a hotspot search asks for (astromech_market::mining::Want); null: any. */
export interface Want {
  ore: Ore;
  ring_type: RingType | null;
  reserve: Reserve | null;
}

export interface Signal {
  name: string;
  count: number;
}

/** One ring (astromech_market::mining::Hotspot). */
export interface Hotspot {
  system: string;
  body: string;
  /** "Barnard's Star 5 A Ring". */
  ring: string;
  ring_type: string;
  /** Hotspots of the ore in the ring. */
  count: number;
  /** The ring's hotspots of other ores, most first. */
  also: Signal[];
  /** "Pristine", "Major", "Common", "Low" or "Depleted". */
  reserve: string | null;
  ly: number;
  jumps: number;
  exact: boolean;
  arrival_ls: number | null;
  updated_at: string | null;
}

/** One search's answer (astromech_market::mining::Hotspots). */
export interface Hotspots {
  want: Want;
  /** The ore's name: "Painite". */
  name: string;
  here: { system: string };
  range: number;
  radius_ly: number | null;
  /** Bodies within the radius with the mineral in a ring and the reserve asked for (Spansh's count, which stops at 10000). */
  bodies: number;
  /** The requests ran out before every ring could be ranked: `rings` are every ring with at least this many hotspots. */
  least: number | null;
  /** The requests ran out before any count was known in full: `rings` are the most hotspots within this many ly only. */
  reach_ly: number | null;
  rings: Hotspot[];
  /** When Spansh was asked, in unix seconds. */
  fetched_at: number;
}

/** The shell's answer size. */
const LIMIT = 10;
const RESERVE_LEVELS: Record<Reserve, string[]> = { pristine: ["Pristine"], major: ["Pristine", "Major"] };

export function oreNamed(symbol: string): (typeof ORES)[number] | undefined {
  return ORES.find((o) => o.symbol === symbol);
}

/** The browser preview: the recorded Painite answer, with the ring type and reserve applied to
 * its rings (other minerals get the same rings). */
function preview(want: Want, origin: Origin): Hotspots {
  const s = sample as unknown as Hotspots;
  const levels = want.reserve ? RESERVE_LEVELS[want.reserve] : null;
  const rings = s.rings
    .filter((r) => (!want.ring_type || r.ring_type === want.ring_type) && (!levels || (r.reserve != null && levels.includes(r.reserve))))
    .slice(0, LIMIT);
  return { ...s, want, name: oreNamed(want.ore)?.name ?? want.ore, rings, here: origin ? { system: origin } : s.here };
}

/** Rings with a hotspot of `want` within the finder's radius of `origin` (or the commander). */
export async function hotspots(want: Want, origin: Origin, filters: Filters): Promise<Hotspots> {
  if (!inTauri) return previewLookup("mining_hotspots", origin, () => preview(want, origin));
  return await invoke<Hotspots>("mining_hotspots", { want, origin, filters });
}

/** "5 A Ring" for a ring named after its system, else the whole name. */
export function ringLabel(ring: string, system: string): string {
  return ring.startsWith(`${system} `) ? ring.slice(system.length + 1) : ring;
}

/** "Platinum 1, Osmium 1": the ring's other hotspots. */
export function alsoLine(also: Signal[]): string {
  return also.map((s) => `${s.name} ${s.count}`).join(", ");
}

/**
 * The search kit's rows: ranked by the ring's hotspots of the mineral, with its other hotspots,
 * ring type and reserve as the finder's columns. Hotspots never change, so there is no report age.
 */
export function hotspotResults(h: Hotspots): Results {
  return {
    origin: h.here.system,
    ...(h.reach_ly != null ? { reach: Math.floor(h.reach_ly) } : {}),
    notes: cutNotes(h),
    range: h.range,
    source: "Spansh",
    fetched: new Date(h.fetched_at * 1000).toISOString(),
    place: "ring",
    main: { head: "Hotspots" },
    columns: [
      { key: "also", head: "Also", text: true },
      { key: "type", head: "Type", text: true, width: 88 },
      { key: "reserve", head: "Reserve", text: true, width: 72 },
    ],
    undated: true,
    rows: h.rings.map((r) => ({
      key: r.ring,
      system: r.system,
      station: ringLabel(r.ring, r.system),
      ly: r.ly,
      jumps: r.jumps,
      exact: r.exact,
      arrival_ls: r.arrival_ls,
      updated_at: null,
      main: String(r.count),
      facts: { also: alsoLine(r.also), type: r.ring_type, reserve: r.reserve ?? "" },
    })),
  };
}

/**
 * The summary line's words for what the answer covers: how many bodies the rings shown were
 * ranked from ("top 10 of 1,101 bodies with Painite"), or, when Spansh's answer was too big to
 * rank within the requests, what was ranked instead ("5 or more hotspots only").
 */
export function cutNotes(h: Hotspots): string[] {
  const bodies = `${h.bodies >= 10000 ? "10,000+" : figure(h.bodies)} bod${h.bodies === 1 ? "y" : "ies"}`;
  const radius = h.radius_ly != null ? ` within ${figure(h.radius_ly)} ly` : "";
  if (h.reach_ly != null) return [`${bodies}${radius}, too many to rank`];
  if (h.least != null) return [`${h.least} or more hotspots only: ${bodies}, too many to rank`];
  if (h.rings.length >= LIMIT && h.bodies > h.rings.length) return [`top ${h.rings.length} of ${bodies} with ${h.name}`];
  return [];
}

/** The summary line's words for the finder's own choices: "metal rich rings · pristine reserves". */
export function wantNotes(want: Want): string[] {
  const notes: string[] = [];
  if (want.ring_type) notes.push(`${want.ring_type.toLowerCase()} rings`);
  if (want.reserve === "pristine") notes.push("pristine reserves");
  if (want.reserve === "major") notes.push("major or pristine reserves");
  return notes;
}
