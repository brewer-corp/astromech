/**
 * The "search near me" kit's model (docs/ARCHITECTURE.md §18): where a search starts (the
 * current system or a typed one), the rows every lookup shows, system-name autocomplete
 * (Spansh, asked by the core), and each lookup's remembered filters (the core keeps them in
 * astromech-store settings). Lookups adapt their own answers into {@link Results}.
 */
import type { Filters } from "./api/types/Filters";
import type { Pad } from "./api/types/Pad";
import { call } from "./transport";

/** Where a search measures from: the commander's system (`null`) or a typed system name. */
export type Origin = string | null;

export type { Pad } from "./api/types/Pad";
export type { Filters } from "./api/types/Filters";

/** One result line. Station fields are absent for system lookups (faction states). */
export interface Row {
  key: string | number;
  system: string;
  /** The station or body; absent when the place is the system. */
  station?: string;
  ly: number;
  jumps: number;
  /** false: `jumps` is a straight-line estimate, not a route over known stars. */
  exact: boolean;
  pad?: Pad | null;
  /** Light seconds from the arrival star. */
  arrival_ls?: number | null;
  /** When the source last saw this data (ISO 8601). */
  updated_at: string | null;
  /** The value the list is ranked by when it isn't jumps (Results.main), e.g. "313,239". */
  main?: string;
  /** The lookup's own columns (Results.columns), by key. Missing: "—". */
  facts?: Record<string, string>;
  /** A prose fact's Tip, by key: what its short words stand for ("2 other powers": which). */
  notes?: Record<string, string>;
  /** Spansh station type, shown as a HUD mark. */
  station_type?: string | null;
  /** A market's demand or stock in tonnes, for the bars of a `bars` column (Sell, Buy). */
  volume?: number;
}

/**
 * A fact a lookup adds to the shared ones (DESIGN.md Lookup result row). Where it sits on the
 * grouped row is `at`: under the ranked value, in the row's own group, or in the side panel.
 */
export interface Column {
  key: string;
  /** Its name in the group head and the side panel, in Label. */
  head: string;
  /** Figures: right-aligned, with this unit after each. */
  unit?: string;
  /** Prose (factions, bodies, a module): left-aligned and truncated. */
  text?: boolean;
  /**
   * "qual": under the ranked value, the fact that says whether it holds for this commander
   * (demand under price); only with Results.main. "row": the row's own group, a fact the
   * commander picks a row by (the module, the faction in a state). Absent: the side panel, or
   * the row's own group where the list has no side panel.
   */
  at?: "qual" | "row";
  /** The fact is a market's tonnes (Row.volume), drawn after the demand bars. */
  bars?: boolean;
  /** How the summary line names a value every row shares ("Metallic rings"); default "<head>: <value>". */
  same?: (value: string) => string;
}

/** What a lookup's place column holds: stations get pad and arrival columns, bodies and rings arrival. */
export type Place = "station" | "body" | "ring" | "system";

export interface Results {
  /** The system the search measured from. */
  origin: string;
  /** Jump range used for counting, in ly. */
  range: number;
  /** Who answered, e.g. "Spansh". */
  source: string;
  /** When the source was asked (ISO 8601; may be a cached answer). */
  fetched: string;
  place: Place;
  /** What the list is ranked by, shown first and largest (Row.main). Absent: jumps. */
  main?: { head: string; unit?: string };
  /** The lookup's own fact columns, before distance, jumps, pad, arrival and report age. */
  columns?: Column[];
  rows: Row[];
  warning?: string;
  /** What the lookup found, e.g. "Famine": the label a result gets on the route (Add stop, Set destination) when it has no station. */
  label?: string;
  /** The source's data doesn't go stale (ring hotspots never change): no report age column. */
  undated?: boolean;
  /** The answer's own constraints for the summary line, after the lookup's ("5 or more hotspots only"). */
  notes?: string[];
  /**
   * The answer covers only this many ly, short of the radius asked (too much to rank in the
   * requests allowed): the summary and the empty line name it, and a wider radius is no help.
   */
  reach?: number;
}

/**
 * Which filters a lookup offers: system lookups only have an age; station lookups add pad,
 * carriers, Odyssey and arrival; market lookups add the search radius and a demand or stock.
 */
export type FilterKey = "pad" | "carriers" | "odyssey" | "age" | "arrival" | "distance" | "volume";

/** How a lookup labels its filters: what "volume" means, and its default radius. */
export interface FilterHints {
  /** "Demand" or "Stock". */
  volume?: string;
  /** The radius used when none is picked, in ly. */
  distance?: number;
  /** The data age used when none is picked, in days. */
  age?: number;
}

export const DEFAULT_FILTERS: Filters = {
  pad: null,
  pad_set: false,
  exclude_carriers: false,
  exclude_odyssey: false,
  max_age_days: null,
  max_arrival_ls: null,
  max_distance_ly: null,
  min_volume: null,
};

/** `lookup`'s remembered filters, the pad following the current ship unless the commander set it. */
export async function loadFilters(lookup: string): Promise<Filters> {
  return await call("search_filters", { lookup });
}

export async function saveFilters(lookup: string, filters: Filters): Promise<void> {
  await call("set_search_filters", { lookup, filters });
}

/**
 * System names resembling `prefix`, at most a handful: Spansh's autocomplete is fuzzy, not a
 * prefix match (the browser preview only does prefixes). Blank asks nothing.
 */
export async function systemNames(prefix: string): Promise<string[]> {
  const p = prefix.trim();
  if (!p) return [];
  return await call("galaxy_systems", { prefix: p });
}

/**
 * What Enter in the origin box picks. Spansh's autocomplete is fuzzy, so the first suggestion
 * is not always what was typed: a suggestion the commander arrowed to wins, else a suggestion
 * equal to the typed text (ignoring case), else the typed text itself.
 */
export function enterChoice(text: string, hits: string[], sel: number, moved: boolean): string {
  if (moved && hits[sel]) return hits[sel];
  const t = text.trim().toLowerCase();
  return hits.find((h) => h.toLowerCase() === t) ?? text;
}

/** "L pad", or "" when the source doesn't say. */
export function padLabel(p: Pad | null | undefined): string {
  return p ? `${p} pad` : "";
}

/** "450 ls", "12k ls", or "" when unknown. */
export function arrivalLabel(ls: number | null | undefined): string {
  if (ls == null) return "";
  return ls >= 10000 ? `${Math.round(ls / 1000)}k ls` : `${Math.round(ls)} ls`;
}

/** "Under repairs" from Spansh's "UnderRepairs"; empty when missing or "None". */
export function stateLabel(s: string | null | undefined): string {
  if (!s || /^none$/i.test(s)) return "";
  return s.replace(/([a-z])([A-Z])/g, "$1 $2");
}

/** The steps each filter's dropdown offers (SearchFilters), which widening walks up. */
export const AGES = [1, 3, 7, 30, 90, 365];
export const ARRIVALS = [1000, 5000, 25000];
export const DISTANCES = [20, 50, 100, 200, 500];
export const VOLUMES = [100, 500, 1000, 5000];

/** A report older than this many days is stale: shown with its age in Orange (DESIGN.md States). */
export const STALE_DAYS = 30;

/** "1,234" with no decimals, "1,234.5" with `digits`. */
export function figure(n: number, digits = 0): string {
  return n.toLocaleString("en", { minimumFractionDigits: digits, maximumFractionDigits: digits });
}

/** One fact of a result row: a figure with its unit, or prose. */
export interface Cell {
  value: string;
  unit?: string;
  text?: boolean;
}

const DASH: Cell = { value: "—" };

/** "~3" for a straight-line estimate, "3" for a route over known stars. */
export function jumpsFigure(r: Pick<Row, "jumps" | "exact">): string {
  return `${r.exact ? "" : "~"}${figure(r.jumps)}`;
}

/** "jump" or "jumps" after a jumps figure. */
export function jumpsUnit(f: string): string {
  return /^~?1$/.test(f) ? "jump" : "jumps";
}

/** How old a report is, as a figure and unit: "12 min", "5 h", "212 d". */
export function ageCell(iso: string | null, now: number): Cell {
  if (!iso) return DASH;
  const s = Math.max(0, (now - Date.parse(iso)) / 1000);
  if (s < 3600) return { value: figure(Math.max(1, Math.round(s / 60))), unit: "min" };
  if (s < 172800) return { value: figure(Math.round(s / 3600)), unit: "h" };
  return { value: figure(Math.round(s / 86400)), unit: "d" };
}

/** A lookup's own fact on a row: its figure with the column's unit, prose, or a dash when missing. */
export function factCell(c: Column, r: Row): Cell {
  const v = r.facts?.[c.key];
  return v ? { value: v, unit: c.unit, text: c.text } : DASH;
}

/**
 * Where a list's facts sit on DESIGN.md's grouped result row: two lines in groups, the ranked
 * value over what qualifies it, the place over its system, then the lookup's own group and the
 * trip. Worked out per answer, since a fact the same on every row leaves the row.
 */
export interface Groups {
  /** Under the ranked value: what says whether it holds for this commander. */
  qual: Column | null;
  /** A system place's second line (its faction in a state, its bodies). */
  under: Column | null;
  /** The row's own group: the first on its first line, the rest on its second. */
  own: Column[];
  /** Only in the side panel. */
  aside: Column[];
  /** The summary line's words for each prose fact every row shares, left off the rows. */
  same: string[];
  /** The list is ranked by something other than jumps, so the trip is a group of its own. */
  trip: boolean;
}

/**
 * The groups for `res`'s `rows`. `panel`: the list has a side panel, so a fact the commander
 * doesn't pick a row by goes there; without one it stays on the row.
 */
export function groups(res: Pick<Results, "place" | "main" | "columns">, rows: Row[], panel: boolean): Groups {
  const same: string[] = [];
  const shared = (c: Column) => {
    if (!c.text || c.at === "qual" || rows.length < 2) return false;
    const v = rows[0]?.facts?.[c.key] ?? "";
    if (!rows.every((r) => (r.facts?.[c.key] ?? "") === v)) return false;
    if (v) same.push(c.same ? c.same(v) : `${c.head}: ${v}`);
    return true;
  };
  const cols = (res.columns ?? []).filter((c) => !shared(c));
  const qual = res.main ? (cols.find((c) => c.at === "qual") ?? null) : null;
  const picked = cols.filter((c) => c !== qual && c.at);
  const rest = cols.filter((c) => !c.at);
  const onRow = panel ? picked : [...picked, ...rest];
  const under = res.place === "system" && onRow[0]?.text ? onRow[0] : null;
  return { qual, under, own: onRow.filter((c) => c !== under), aside: panel ? rest : [], same, trip: !!res.main };
}

/** A group's head: its facts' names, the first capitalised ("Price · demand"). */
export function groupHead(names: string[]): string {
  return names.map((n, i) => (i ? n.toLowerCase() : n)).join(" · ");
}

/** The place group's head: "Station · pad · arrival", "Ring · arrival", "System · faction". */
export function placeHead(place: Place, under: Column | null): string {
  const names = { station: ["Station", "pad", "arrival"], body: ["Body", "arrival"], ring: ["Ring", "arrival"], system: ["System"] }[place];
  return groupHead(under ? [...names, under.head] : names);
}

/** Every fact of a row for its side panel, under its name: the lookup's own, then pad, arrival, trip and report age. */
export function panelFacts(res: Pick<Results, "place" | "main" | "columns" | "undated">, r: Row, now: number): { head: string; cell: Cell }[] {
  const out = (res.columns ?? []).map((c) => ({ head: c.head, cell: factCell(c, r) }));
  if (res.place === "station") out.push({ head: "Pad", cell: r.pad ? { value: r.pad } : DASH });
  if (res.place !== "system") out.push({ head: "Arrival", cell: r.arrival_ls == null ? DASH : { value: figure(r.arrival_ls), unit: "ls" } });
  const j = jumpsFigure(r);
  out.push({ head: "Trip", cell: { value: `${j} ${jumpsUnit(j)} · ${figure(r.ly, 1)}`, unit: "ly" } });
  if (!res.undated) {
    const age = ageCell(r.updated_at, now);
    out.push({ head: "Updated", cell: age === DASH ? DASH : { value: age.value, unit: `${age.unit} ago` } });
  }
  return out;
}

/** The age filter in force, in days: the commander's, else the lookup's default; null: any age. */
export function ageLimit(filters: Filters, hints: FilterHints): number | null {
  return filters.max_age_days ?? hints.age ?? null;
}

/** Whether a row's report is older than `days`. A row with no date is not stale: its age is unknown. */
export function isStale(r: Pick<Row, "updated_at">, days: number, now: number): boolean {
  return r.updated_at != null && now - Date.parse(r.updated_at) > days * 86400000;
}

const PAD_NAMES: Record<Pad, string> = { S: "", M: "medium or large pad", L: "large pad" };

/**
 * The one-line summary of a search's constraints above the column heads ("Within 100 ly of Sol ·
 * large pad · reports up to 30 days old"), naming only the filters the lookup offers.
 * `lead` says where it searched when there is no radius ("Nearest to Sol"); `notes` are the
 * lookup's own constraints, after it ("metallic rings").
 */
export function summary(
  lead: string,
  origin: string,
  filters: Filters,
  offers: FilterKey[],
  hints: FilterHints,
  hidden: number,
  notes: string[] = [],
  reach: number | null = null,
): string {
  const radius = reach ?? filters.max_distance_ly ?? hints.distance ?? null;
  const parts = [offers.includes("distance") && radius != null ? `Within ${figure(radius)} ly of ${origin}` : `${lead} ${origin}`, ...notes, ...filterParts(filters, offers, hints)];
  if (hidden > 0) parts.push(`${figure(hidden)} older report${hidden === 1 ? "" : "s"} hidden`);
  return parts.join(" · ");
}

/**
 * What the filters a lookup offers hold, other than the radius, each as a summary phrase: "large
 * pad", "reports up to 30 days old". A filter away from its default is always named.
 */
export function filterParts(filters: Filters, offers: FilterKey[], hints: FilterHints): string[] {
  const has = (k: FilterKey) => offers.includes(k);
  const parts: string[] = [];
  if (has("volume") && filters.min_volume != null) parts.push(`${(hints.volume ?? "volume").toLowerCase()} at least ${figure(filters.min_volume)} t`);
  // S is any pad: unnamed when it is the ship's, named when the commander chose it over a bigger ship's.
  if (has("pad") && filters.pad && (PAD_NAMES[filters.pad] || filters.pad_set)) parts.push(PAD_NAMES[filters.pad] || "any pad");
  if (has("arrival") && filters.max_arrival_ls != null) parts.push(`arrival up to ${figure(filters.max_arrival_ls)} ls`);
  if (has("carriers") && filters.exclude_carriers) parts.push("no carriers");
  if (has("odyssey") && filters.exclude_odyssey) parts.push("no Odyssey settlements");
  if (has("age")) {
    const days = ageLimit(filters, hints);
    parts.push(days == null ? "reports of any age" : `reports up to ${figure(days)} day${days === 1 ? "" : "s"} old`);
  }
  return parts;
}

/** An empty search's one way forward: what held it back, and the filters with that relaxed. */
export interface Widening {
  /** Completes "No stations …": "within 100 ly". */
  phrase: string;
  /** The action: "Widen to 200 ly". */
  label: string;
  filters: Filters;
}

const nextStep = (steps: number[], v: number) => steps.find((s) => s > v);

/**
 * The first constraint an empty search can relax: the radius, the report age, the demand or
 * stock, the arrival distance, then carriers and Odyssey settlements. Never the pad: a bigger
 * search is no use if the ship can't land. Null when nothing is left to relax.
 */
export function widening(filters: Filters, offers: FilterKey[], hints: FilterHints): Widening | null {
  const has = (k: FilterKey) => offers.includes(k);
  const radius = filters.max_distance_ly ?? hints.distance ?? null;
  const wider = radius == null ? undefined : nextStep(DISTANCES, radius);
  if (has("distance") && radius != null && wider != null) {
    return { phrase: `within ${figure(radius)} ly`, label: `Widen to ${figure(wider)} ly`, filters: { ...filters, max_distance_ly: wider } };
  }
  const days = ageLimit(filters, hints);
  if (has("age") && days != null) {
    const older = nextStep(AGES, days);
    const phrase = `with reports up to ${figure(days)} day${days === 1 ? "" : "s"} old`;
    if (older != null) return { phrase, label: `Allow ${figure(older)} days`, filters: { ...filters, max_age_days: older } };
    if (hints.age == null) return { phrase, label: "Allow any age", filters: { ...filters, max_age_days: null } };
  }
  if (has("volume") && filters.min_volume != null) {
    const what = (hints.volume ?? "volume").toLowerCase();
    return { phrase: `with ${what} of at least ${figure(filters.min_volume)} t`, label: `Any ${what}`, filters: { ...filters, min_volume: null } };
  }
  if (has("arrival") && filters.max_arrival_ls != null) {
    return { phrase: `within ${figure(filters.max_arrival_ls)} ls of the star`, label: "Any arrival distance", filters: { ...filters, max_arrival_ls: null } };
  }
  if (has("carriers") && filters.exclude_carriers) return { phrase: "outside fleet carriers", label: "Include carriers", filters: { ...filters, exclude_carriers: false } };
  if (has("odyssey") && filters.exclude_odyssey) return { phrase: "outside Odyssey settlements", label: "Include Odyssey", filters: { ...filters, exclude_odyssey: false } };
  return null;
}
