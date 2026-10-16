/**
 * The "search near me" kit's model (docs/ARCHITECTURE.md §18): where a search starts (the
 * current system or a typed one), the rows every lookup shows, and system-name autocomplete
 * (Spansh, through the shell). Lookups adapt their own answers into {@link Results}.
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

/** Public system names for the browser preview's autocomplete. */
const PREVIEW_SYSTEMS = ["Sol", "Shinrarta Dezhra", "Shingon", "Shinigami", "Deciat", "Colonia", "Achenar", "Alioth", "Maia"];

/** System names starting with `prefix` (at most a handful). Blank asks nothing. */
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
