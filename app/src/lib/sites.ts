/**
 * Raw-material sites near a system (crystalline shards, brain trees, crashed ships, geological
 * sites, the known grade 4 farms) with jump counts from the local star map. The shell asks
 * Spansh's bodies search and does the maths (astromech_galaxy::sites).
 */
import type { Filters, Origin, Results } from "./search";
import { call } from "./transport";

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

/** The kinds the shell offers, in its order (astromech_galaxy::SITE_KINDS). */
export const SITE_KINDS: SiteKind[] = [
  {
    id: "crystalline_shards",
    label: "Crystalline shards",
    noun: "crystalline shards",
    yields: "Grade 4 raw (polonium, ruthenium, tellurium, technetium, yttrium, antimony and more). Shoot the shards from the SRV and collect the fragments.",
  },
  { id: "brain_trees", label: "Brain trees", noun: "brain trees", yields: "Grade 4 raw. Shoot them (turrets or flak), then collect with the SRV." },
  { id: "crashed_ships", label: "Crashed ships", noun: "crashed ship", yields: "Encoded data points and salvage. Scan the data points from the SRV." },
  { id: "geological", label: "Geological sites", noun: "geological sites", yields: "Grade 1 to 3 raw from the outcrops around geysers, fumaroles and vents." },
  { id: "raw_farms", label: "Known raw farms", noun: "raw material farm", yields: "The well-known grade 4 farms: one body for each element." },
];

/** The kind with this id, else the first (crystalline shards). */
export function siteKind(id: string): SiteKind {
  return SITE_KINDS.find((k) => k.id === id) ?? SITE_KINDS[0]!;
}

/** Sites of `kind` matching `filters` near `origin` (a typed system name), or near the commander. */
export async function sites(kind: string, origin: Origin = null, filters: Filters | null = null): Promise<Sites> {
  return await call<Sites>("galaxy_sites", { kind, origin, filters });
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
    place: "body",
    columns: [{ key: "detail", head: "Site", text: true }],
    rows: r.hits.map((h) => ({
      key: h.body,
      system: h.system,
      station: bodyLabel(h.body, h.system),
      ly: h.ly,
      jumps: h.jumps,
      exact: h.exact,
      arrival_ls: h.arrival_ls,
      updated_at: h.updated_at,
      facts: { detail: h.detail },
    })),
  };
}
