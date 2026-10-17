/**
 * Station lookups (astromech_galaxy::stations): the nearest stations offering a service, near
 * the commander or a typed system, through the shell (Spansh). Adapted into the search kit's
 * rows by {@link toResults}.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import { previewGate } from "./fakecore";
import sample from "./fixtures/stations.json";
import moduleSample from "./fixtures/stations-module.json";
import shipSample from "./fixtures/stations-ship.json";
import type { Filters, Origin, Pad, Results } from "./search";

/** Service ids (astromech_galaxy::stations::Service) with the names shown for them. */
export const SERVICES = [
  { id: "interstellar_factors", label: "Interstellar Factors" },
  { id: "raw_trader", label: "Raw material trader" },
  { id: "manufactured_trader", label: "Manufactured material trader" },
  { id: "encoded_trader", label: "Encoded data trader" },
  { id: "material_trader", label: "Material trader (any type)" },
  { id: "human_broker", label: "Human technology broker" },
  { id: "guardian_broker", label: "Guardian technology broker" },
  { id: "technology_broker", label: "Technology broker (any type)" },
  { id: "universal_cartographics", label: "Universal Cartographics" },
  { id: "vista_genomics", label: "Vista Genomics" },
  { id: "pioneer_supplies", label: "Pioneer Supplies" },
  { id: "bartender", label: "Bartender" },
  { id: "frontline_solutions", label: "Frontline Solutions" },
  { id: "apex", label: "Apex Interstellar" },
  { id: "refuel", label: "Refuel" },
  { id: "repair", label: "Repair" },
  { id: "rearm", label: "Rearm" },
  { id: "black_market", label: "Black market" },
  { id: "search_and_rescue", label: "Search and rescue" },
] as const;

export type ServiceId = (typeof SERVICES)[number]["id"];

export type Mount = "Fixed" | "Gimballed" | "Turreted";

/** A module to buy (astromech_galaxy::outfitting::ModuleWant); null: any. */
export interface ModuleWant {
  name: string;
  class: number | null;
  rating: string | null;
  mount: Mount | null;
}

/** What a station lookup looks for (astromech_galaxy::stations::Want). */
export type Want = { kind: "service"; service: ServiceId } | ({ kind: "module" } & ModuleWant) | { kind: "ship"; ship: string };

export interface StationHit {
  name: string;
  station_type: string | null;
  market_id: number | null;
  system: string;
  system_id64: number;
  ly: number;
  jumps: number;
  exact: boolean;
  arrival_ls: number | null;
  pad: Pad | null;
  updated_at: string | null;
  price: number | null;
  detail: string | null;
}

export interface StationsNearby {
  want: string;
  here: { system: string };
  range: number;
  filters: Filters;
  fetched: string;
  hits: StationHit[];
  warning?: string;
}

const PADS: Pad[] = ["S", "M", "L"];

/** The browser preview: a sample answer, with the filters applied the way the shell does. */
function preview(want: Want, origin: Origin, f: Filters): StationsNearby {
  const s = (want.kind === "module" ? moduleSample : want.kind === "ship" ? shipSample : sample) as unknown as StationsNearby;
  const fetched = Date.parse(s.fetched);
  const hits = s.hits.filter(
    (h) =>
      (f.pad == null || h.pad == null || PADS.indexOf(h.pad) >= PADS.indexOf(f.pad)) &&
      (f.max_arrival_ls == null || h.arrival_ls == null || h.arrival_ls <= f.max_arrival_ls) &&
      (f.max_age_days == null || (h.updated_at != null && fetched - Date.parse(h.updated_at) <= f.max_age_days * 86400000)) &&
      !(f.exclude_carriers && h.station_type === "Drake-Class Carrier") &&
      !(f.exclude_odyssey && h.station_type === "Settlement"),
  );
  return { ...s, filters: f, hits, here: origin ? { system: origin } : s.here };
}

/** The nearest stations offering `want` near `origin` (or the commander) that pass `filters`. */
export async function stations(want: Want, origin: Origin, filters: Filters): Promise<StationsNearby> {
  if (!inTauri) {
    await previewGate("stations_nearby");
    return preview(want, origin, filters);
  }
  return await invoke<StationsNearby>("stations_nearby", { want, origin, filters });
}

/** "Coriolis Starport", plus the price when something is sold there. */
function detail(h: StationHit): string {
  const price = h.price != null ? `${h.price.toLocaleString("en")} cr` : "";
  return [h.station_type ?? "", h.detail ?? "", price].filter(Boolean).join(" · ");
}

/** The search kit's rows for a station lookup's answer. */
export function toResults(r: StationsNearby): Results {
  return {
    origin: r.here.system,
    range: r.range,
    source: "Spansh",
    fetched: r.fetched,
    warning: r.warning,
    rows: r.hits.map((h) => ({
      key: `${h.system_id64}/${h.name}`,
      system: h.system,
      station: h.name,
      ly: h.ly,
      jumps: h.jumps,
      exact: h.exact,
      pad: h.pad,
      arrival_ls: h.arrival_ls,
      updated_at: h.updated_at,
      detail: detail(h),
    })),
  };
}
