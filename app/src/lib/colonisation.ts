/**
 * Colonisation (astromech-colonisation, astromech-market::list): construction projects from the
 * journals with what is left to buy for each, and where to buy that list, stations stocking
 * most of it first (Ardent Insight through the shell). The browser preview answers from a
 * synthetic project (fixtures/colonisation.json) and a recorded search (fixtures/trade.json).
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import projectsSample from "./fixtures/colonisation.json";
import tradeSample from "./fixtures/trade.json";
import type { Filters, Origin, Pad, Results } from "./search";
import { credits, DEFAULT_DISTANCE_LY, DEFAULT_MAX_AGE_DAYS } from "./trade";

/** The id the search kit remembers this lookup's filters under (astromech_market::list::LOOKUP). */
export const LOOKUP = "colonisation.buy";

/** One commodity on a project's shopping list (astromech_colonisation::shopping::Line). */
export interface ShoppingLine {
  /** Market symbol: "steel". */
  symbol: string;
  /** Journal name: "$steel_name;". */
  name: string;
  name_localised: string | null;
  remaining: number;
  in_cargo: number;
  to_buy: number;
}

/** A construction project and its shopping list (astromech_colonisation::ProjectView). */
export interface Project {
  market_id: number;
  station_name: string | null;
  system_name: string | null;
  /** 0 to 1. */
  progress: number;
  complete: boolean;
  failed: boolean;
  depot_updated: string;
  shopping: {
    lines: ShoppingLine[];
    to_buy_total: number;
    capacity: number | null;
    loads: number | null;
  };
}

/** One commodity still to buy (astromech_market::list::Need). */
export interface Need {
  symbol: string;
  name: string;
  tonnes: number;
}

/** One listed commodity a station sells. */
export interface StockLine {
  commodity: string;
  name: string;
  needed: number;
  stock: number;
  price: number;
}

/** A station selling some of the list (astromech_market::list::Stockist). */
export interface Stockist {
  market_id: number;
  station: string;
  system: string;
  ly: number;
  jumps: number;
  exact: boolean;
  pad: Pad | null;
  arrival_ls: number | null;
  fleet_carrier: boolean;
  odyssey: boolean;
  updated_at: string;
  stocked: number;
  lines: StockLine[];
  missing: string[];
}

export interface ListPlan {
  stations: Stockist[];
  unavailable: string[];
  truncated: string[];
  /** Unix seconds. */
  fetched_at: number;
}

export interface BuySearch {
  here: { system: string; pos: [number, number, number] };
  range: number;
  plan: ListPlan;
}

/** Construction projects from the journals, those being built first. */
export async function loadProjects(): Promise<Project[]> {
  if (inTauri) return await invoke<Project[]>("colonisation_projects");
  return projectsSample as unknown as Project[];
}

/** A project's display name: the station, else its market id. */
export function projectName(p: Project): string {
  return p.station_name ?? `Construction site ${p.market_id}`;
}

/** What is left to buy for `p`, as the lookup asks for it. */
export function needsOf(p: Project): Need[] {
  return p.shopping.lines.filter((l) => l.to_buy > 0).map((l) => ({ symbol: l.symbol, name: l.name_localised ?? l.symbol, tonnes: l.to_buy }));
}

const PAD_RANK: Record<Pad, number> = { S: 1, M: 2, L: 3 };

function previewKeeps(s: Stockist, f: Filters, now: number): boolean {
  if (s.ly > (f.max_distance_ly ?? DEFAULT_DISTANCE_LY)) return false;
  if (f.exclude_carriers && s.fleet_carrier) return false;
  if (f.exclude_odyssey && s.odyssey) return false;
  if (f.pad && s.pad && PAD_RANK[s.pad] < PAD_RANK[f.pad]) return false;
  if (f.max_arrival_ls != null && s.arrival_ls != null && s.arrival_ls > f.max_arrival_ls) return false;
  if (now - Date.parse(s.updated_at) > (f.max_age_days ?? DEFAULT_MAX_AGE_DAYS) * 86400000) return false;
  return true;
}

/** Where to buy `needs` near `origin` (or the commander), stations stocking most of them first. */
export async function buyList(needs: Need[], origin: Origin = null, filters: Filters | null = null): Promise<BuySearch> {
  if (inTauri) return await invoke<BuySearch>("colonisation_buy", { needs, origin, filters });
  const s = tradeSample as unknown as { list: ListPlan; buy: { here: BuySearch["here"]; range: number } };
  const now = s.list.fetched_at * 1000;
  const keep = (x: Stockist) => !filters || previewKeeps(x, filters, now);
  return {
    here: origin ? { ...s.buy.here, system: origin } : s.buy.here,
    range: s.buy.range,
    plan: { ...s.list, stations: s.list.stations.filter(keep) },
  };
}

/** The stations as the search kit's rows: how much of the list each sells, and each stock. */
export function buyResults(s: BuySearch, listed: number): Results {
  const notes = [
    s.plan.unavailable.length ? `No market in range sells ${s.plan.unavailable.join(", ")}.` : "",
    s.plan.truncated.length ? `Too many markets for ${s.plan.truncated.join(", ")}: narrow the radius to see them all.` : "",
  ].filter(Boolean);
  return {
    origin: s.here.system,
    range: s.range,
    source: "Ardent Insight",
    fetched: new Date(s.plan.fetched_at * 1000).toISOString(),
    warning: notes.length ? notes.join(" ") : undefined,
    rows: s.plan.stations.map((x) => ({
      key: x.market_id,
      system: x.system,
      station: x.station,
      ly: x.ly,
      jumps: x.jumps,
      exact: x.exact,
      pad: x.pad,
      arrival_ls: x.arrival_ls,
      updated_at: x.updated_at,
      lead: `${x.stocked} of ${listed}`,
      detail: [
        x.lines.map((l) => `${l.name} ${credits(l.stock)} t`).join(", "),
        x.fleet_carrier ? "fleet carrier" : "",
        x.odyssey ? "Odyssey settlement" : "",
      ]
        .filter(Boolean)
        .join(" · "),
    })),
  };
}
