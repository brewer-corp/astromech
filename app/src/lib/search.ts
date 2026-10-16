/**
 * The "search near me" kit's model (docs/ARCHITECTURE.md §18): where a search starts (the
 * current system or a typed one), the rows every lookup shows, system-name autocomplete
 * (Spansh, through the shell), and each lookup's remembered filters (the shell keeps them in
 * astromech-store settings). Lookups adapt their own answers into {@link Results}.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";

/** Where a search measures from: the commander's system (`null`) or a typed system name. */
export type Origin = string | null;

export type Pad = "S" | "M" | "L";

/** One result line. Station fields are absent for system lookups (faction states). */
export interface Row {
  key: string | number;
  system: string;
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
  /** Lookup-specific line, e.g. the matching faction and its states. */
  detail?: string;
  /** Shown first on the distance line, e.g. a market's price. */
  lead?: string;
}

export interface Results {
  /** The system the search measured from. */
  origin: string;
  /** Jump range used for counting, in ly. */
  range: number;
  /** Who answered, e.g. "Spansh". */
  source: string;
  /** When the source was asked (ISO 8601; may be a cached answer). */
  fetched: string;
  rows: Row[];
  warning?: string;
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

/** The browser preview's pretend ship (a medium-pad ship). */
const PREVIEW_SHIP_PAD: Pad = "M";
const previewKey = (lookup: string) => `astromech.search.${lookup}`;

/** `lookup`'s remembered filters, the pad following the current ship unless the commander set it. */
export async function loadFilters(lookup: string): Promise<Filters> {
  if (inTauri) return await invoke<Filters>("search_filters", { lookup });
  let saved: Partial<Filters> = {};
  try {
    saved = JSON.parse(localStorage.getItem(previewKey(lookup)) ?? "{}") as Partial<Filters>;
  } catch {
    saved = {};
  }
  const f = { ...DEFAULT_FILTERS, ...saved };
  return f.pad_set ? f : { ...f, pad: PREVIEW_SHIP_PAD };
}

export async function saveFilters(lookup: string, filters: Filters): Promise<void> {
  if (inTauri) return await invoke("set_search_filters", { lookup, filters });
  localStorage.setItem(previewKey(lookup), JSON.stringify(filters));
}

/** Public system names for the browser preview's autocomplete. */
const PREVIEW_SYSTEMS = ["Sol", "Shinrarta Dezhra", "Shingon", "Shinigami", "Deciat", "Colonia", "Achenar", "Alioth", "Maia"];

/**
 * System names resembling `prefix`, at most a handful: Spansh's autocomplete is fuzzy, not a
 * prefix match (the browser preview only does prefixes). Blank asks nothing.
 */
export async function systemNames(prefix: string): Promise<string[]> {
  const p = prefix.trim();
  if (!p) return [];
  if (!inTauri) return PREVIEW_SYSTEMS.filter((s) => s.toLowerCase().startsWith(p.toLowerCase()));
  return await invoke<string[]>("galaxy_systems", { prefix: p });
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
