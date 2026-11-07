/**
 * Station lookups (astromech_galaxy::stations): the nearest stations offering a service, near
 * the commander or a typed system, asked by the core (Spansh). Adapted into the search kit's
 * rows by {@link toResults}.
 */
import { figure, stateLabel, type Column, type Filters, type Origin, type Results } from "./search";
import type { StationNearby as StationHit } from "./api/types/StationNearby";
import type { StationsNearby } from "./api/types/StationsNearby";
import type { StationWant } from "./api/types/StationWant";
import { call } from "./transport";

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

export type { Mount } from "./api/types/Mount";
export type { ModuleWant } from "./api/types/ModuleWant";
export type { StationNearby as StationHit } from "./api/types/StationNearby";
export type { StationsNearby } from "./api/types/StationsNearby";
/** What a station lookup looks for (astromech_galaxy::stations::Want). */
export type Want = StationWant;

/** The nearest stations offering `want` near `origin` (or the commander) that pass `filters`. */
export async function stations(want: Want, origin: Origin, filters: Filters): Promise<StationsNearby> {
  return await call("stations_nearby", { want, origin, filters });
}

/** "Federation · Mother Gaia · Boom": who runs the station and its state, when the source says. */
export function factionLine(h: Pick<StationHit, "allegiance" | "faction" | "state">): string {
  const named = (v: string | null) => (v && v !== "None" ? v : "");
  return [named(h.allegiance), named(h.faction), stateLabel(h.state)].filter(Boolean).join(" · ");
}

/**
 * The search kit's rows for a station lookup's answer: ranked by jumps, with the modules that
 * matched (Outfitting), the price (Outfitting, Shipyard) and the faction as the lookup's columns.
 */
export function toResults(r: StationsNearby): Results {
  const columns: Column[] = [];
  if (r.hits.some((h) => h.detail)) columns.push({ key: "detail", head: "Module", text: true });
  if (r.hits.some((h) => h.price != null)) columns.push({ key: "price", head: "Price", unit: "cr", width: 120 });
  if (r.hits.some((h) => factionLine(h))) columns.push({ key: "faction", head: "Faction", text: true });
  return {
    origin: r.here.system,
    range: r.range,
    source: "Spansh",
    fetched: r.fetched,
    warning: r.warning,
    place: "station",
    columns,
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
      station_type: h.station_type,
      facts: { detail: h.detail ?? "", price: h.price == null ? "" : figure(h.price), faction: factionLine(h) },
    })),
  };
}
