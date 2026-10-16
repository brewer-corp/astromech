/**
 * Raw-material sites near a system (crystalline shards, brain trees, crashed ships, geological
 * sites, the known grade 4 farms) with jump counts from the local star map. The shell asks
 * Spansh's bodies search and does the maths (astromech_galaxy::sites).
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import sample from "./fixtures/sites.json";
import type { Filters, Origin, Results } from "./search";

export interface SiteKind {
  id: string;
  label: string;
  noun: string;
  yields: string;
}

export interface SiteHit {
  system: string;
  id64: number;
  /** Full body name, e.g. "HIP 36601 C 1 a". */
  body: string;
  ly: number;
  jumps: number;
  exact: boolean;
  arrival_ls: number | null;
  updated_at: string | null;
  detail: string;
}

export interface Sites {
  kind: SiteKind;
  here: { system: string; pos: [number, number, number] };
  range: number;
  source: string;
  fetched: string;
  hits: SiteHit[];
  warning?: string;
}

const samples = sample as unknown as Record<string, Sites | undefined>;

/** The browser preview's answer for `kind` (recorded from Sol). */
function sampleFor(kind: string): Sites {
  const s = samples[kind];
  if (!s) throw new Error(`unknown site kind ${kind}`);
  return s;
}

/** The kinds the shell offers, in its order (astromech_galaxy::SITE_KINDS). */
export const SITE_KINDS: SiteKind[] = ["crystalline_shards", "brain_trees", "crashed_ships", "geological", "raw_farms"].map(
  (id) => sampleFor(id).kind,
);

/** The kind with this id, else the first (crystalline shards). */
export function siteKind(id: string): SiteKind {
  return SITE_KINDS.find((k) => k.id === id) ?? sampleFor("crystalline_shards").kind;
}

/** Sites of `kind` matching `filters` near `origin` (a typed system name), or near the commander. */
export async function sites(kind: string, origin: Origin = null, filters: Filters | null = null): Promise<Sites> {
  if (!inTauri) {
    const s = sampleFor(kind);
    const max = filters?.max_arrival_ls;
    const hits = max == null ? s.hits : s.hits.filter((h) => h.arrival_ls == null || h.arrival_ls <= max);
    return { ...s, hits, here: origin ? { ...s.here, system: origin } : s.here };
  }
  return await invoke<Sites>("galaxy_sites", { kind, origin, filters });
}

/** "B 6 a" for a body named after its system, else the whole name ("Mercury"). */
export function bodyLabel(body: string, system: string): string {
  return body.startsWith(`${system} `) ? body.slice(system.length + 1) : body;
}

/** The search kit's rows for a sites answer. */
export function siteResults(r: Sites): Results {
  return {
    origin: r.here.system,
    range: r.range,
    source: r.source === "astromech" ? "known farms list" : r.source,
    fetched: r.fetched,
    warning: r.warning,
    rows: r.hits.map((h) => ({
      key: h.body,
      system: h.system,
      station: bodyLabel(h.body, h.system),
      ly: h.ly,
      jumps: h.jumps,
      exact: h.exact,
      arrival_ls: h.arrival_ls,
      updated_at: h.updated_at,
      detail: h.detail,
    })),
  };
}
