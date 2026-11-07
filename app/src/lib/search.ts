/**
 * The "search near me" kit's model (docs/ARCHITECTURE.md §18): where a search starts (the
 * current system or a typed one), the rows every lookup shows, system-name autocomplete
 * (Spansh, through the shell), and each lookup's remembered filters (the shell keeps them in
 * astromech-store settings). Lookups adapt their own answers into {@link Results}.
 */
import { call } from "./transport";

/** Where a search measures from: the commander's system (`null`) or a typed system name. */
export type Origin = string | null;

export type Pad = "S" | "M" | "L";

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
  /** Spansh station type, shown as a HUD mark. */
  station_type?: string | null;
}

/** A column a lookup adds to the shared ones (DESIGN.md Lookup result row). */
export interface Column {
  key: string;
  /** The column head, in Label. */
  head: string;
  /** Figures: right-aligned, with this unit after each. */
  unit?: string;
  /** Prose (factions, bodies, a module): left-aligned, truncated, sharing the row's spare width with the place. */
  text?: boolean;
  /** A figure column's width in px; on a prose column, a fixed width instead of a share (a ring type). */
  width?: number;
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

/** One lookup's filters (astromech_galaxy::Filters). */
export interface Filters {
  /** Hide stations without a pad at least this big; null: any pad. */
  pad: Pad | null;
  /** The commander picked `pad`; otherwise it follows the current ship. */
  pad_set: boolean;
  exclude_carriers: boolean;
  exclude_odyssey: boolean;
  max_age_days: number | null;
  max_arrival_ls: number | null;
  /** Search radius in ly; null: the lookup's default. */
  max_distance_ly: number | null;
  /** Hide markets with less demand (selling) or stock (buying) than this, in tonnes. */
  min_volume: number | null;
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
  return await call<Filters>("search_filters", { lookup });
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
  return await call<string[]>("galaxy_systems", { prefix: p });
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

/** A report older than this many days is stale when the lookup has no age filter of its own (DESIGN.md States). */
export const STALE_DAYS = 30;

/** "1,234" with no decimals, "1,234.5" with `digits`. */
export function figure(n: number, digits = 0): string {
  return n.toLocaleString("en", { minimumFractionDigits: digits, maximumFractionDigits: digits });
}

/** One cell of a result row: a figure with its unit, or prose. */
export interface Cell {
  value: string;
  unit?: string;
  text?: boolean;
}

/** A fact column of the row as the kit draws it: the lookup's own, then the shared ones. */
export interface FactColumn extends Column {
  cell: (r: Row, now: number) => Cell;
}

const DASH: Cell = { value: "—" };

/** "~3" for a straight-line estimate, "3" for a route over known stars. */
export function jumpsFigure(r: Pick<Row, "jumps" | "exact">): string {
  return `${r.exact ? "" : "~"}${figure(r.jumps)}`;
}

/** How old a report is, as a figure and unit: "12 min", "5 h", "212 d". */
export function ageCell(iso: string | null, now: number): Cell {
  if (!iso) return DASH;
  const s = Math.max(0, (now - Date.parse(iso)) / 1000);
  if (s < 3600) return { value: figure(Math.max(1, Math.round(s / 60))), unit: "min" };
  if (s < 172800) return { value: figure(Math.round(s / 3600)), unit: "h" };
  return { value: figure(Math.round(s / 86400)), unit: "d" };
}

/**
 * The fact columns after the place, in order: the lookup's own, distance, jumps (unless they
 * are the main value), pad for stations, arrival for stations and bodies, report age.
 */
export function factColumns(res: Pick<Results, "place" | "main" | "columns" | "undated">): FactColumn[] {
  const own = (res.columns ?? []).map(
    (c): FactColumn => ({ ...c, cell: (r) => (r.facts?.[c.key] ? { value: r.facts[c.key]!, unit: c.unit, text: c.text } : DASH) }),
  );
  const shared: FactColumn[] = [{ key: "ly", head: "Distance", unit: "ly", width: 88, cell: (r) => ({ value: figure(r.ly, 1), unit: "ly" }) }];
  if (res.main) shared.push({ key: "jumps", head: "Jumps", width: 56, cell: (r) => ({ value: jumpsFigure(r) }) });
  if (res.place === "station") shared.push({ key: "pad", head: "Pad", width: 40, cell: (r) => (r.pad ? { value: r.pad } : DASH) });
  if (res.place !== "system") {
    shared.push({ key: "arrival", head: "Arrival", unit: "ls", width: 96, cell: (r) => (r.arrival_ls == null ? DASH : { value: figure(r.arrival_ls), unit: "ls" }) });
  }
  if (!res.undated) shared.push({ key: "age", head: "Updated", width: 72, cell: (r, now) => ageCell(r.updated_at, now) });
  return [...own, ...shared];
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
  const has = (k: FilterKey) => offers.includes(k);
  const radius = reach ?? filters.max_distance_ly ?? hints.distance ?? null;
  const parts = [has("distance") && radius != null ? `Within ${figure(radius)} ly of ${origin}` : `${lead} ${origin}`, ...notes];
  if (has("volume") && filters.min_volume != null) parts.push(`${(hints.volume ?? "volume").toLowerCase()} at least ${figure(filters.min_volume)} t`);
  if (has("pad") && filters.pad && PAD_NAMES[filters.pad]) parts.push(PAD_NAMES[filters.pad]);
  if (has("arrival") && filters.max_arrival_ls != null) parts.push(`arrival up to ${figure(filters.max_arrival_ls)} ls`);
  if (has("carriers") && filters.exclude_carriers) parts.push("no carriers");
  if (has("odyssey") && filters.exclude_odyssey) parts.push("no Odyssey settlements");
  if (has("age")) {
    const days = ageLimit(filters, hints);
    parts.push(days == null ? "reports of any age" : `reports up to ${figure(days)} day${days === 1 ? "" : "s"} old`);
  }
  if (hidden > 0) parts.push(`${figure(hidden)} older report${hidden === 1 ? "" : "s"} hidden`);
  return parts.join(" · ");
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
