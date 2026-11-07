/**
 * Trade lookups (astromech-market): where to sell or buy a commodity near a system or anywhere
 * in the galaxy, best price first, and where to sell the cargo in the hold, from Ardent Insight, asked by the core. The
 * browser preview answers from recorded searches and a fixture hold (app/src/preview/fixtures/trade.json) with
 * the filters applied roughly.
 */
import catalog from "../../../data/catalog/commodities.json";
import type { CargoSale as CargoSearch } from "./api/types/CargoSale";
import type { Defaults as RouteDefaults } from "./api/types/Defaults";
import type { MarketComparison as MarketCompare } from "./api/types/MarketComparison";
import type { Search as TradeSearch } from "./api/types/Search";
import type { TradeRoute as RouteSearch } from "./api/types/TradeRoute";
import type { TradeRouteAsk as RouteAsk } from "./api/types/TradeRouteAsk";
import type { TradeSide as Side } from "./api/types/TradeSide";
import type { Filters, Origin, Results } from "./search";
import { call } from "./transport";

export type { TradeSide as Side } from "./api/types/TradeSide";
export type { Offer } from "./api/types/Offer";
export type { Prices } from "./api/types/Prices";
export type { Search as TradeSearch } from "./api/types/Search";
export type { SaleLine } from "./api/types/SaleLine";
export type { StationSale } from "./api/types/StationSale";
export type { CargoPlan } from "./api/types/CargoPlan";
export type { CargoSale as CargoSearch } from "./api/types/CargoSale";
export type { MarketLine } from "./api/types/MarketLine";
export type { MarketComparison as MarketCompare } from "./api/types/MarketComparison";
export type { TradeRouteKind as RouteKind } from "./api/types/TradeRouteKind";
/** What a trade route is asked for (astromech_market::route::Ask). */
export type { TradeRouteAsk as RouteAsk } from "./api/types/TradeRouteAsk";
export type { Defaults as RouteDefaults } from "./api/types/Defaults";
export type { TradeRouteStop as RouteStop } from "./api/types/TradeRouteStop";
export type { Cargo as RouteCargo } from "./api/types/Cargo";
export type { Leg as RouteLeg } from "./api/types/Leg";
export type { TradeRouteResult as TradeRoute } from "./api/types/TradeRouteResult";
export type { TradeRoute as RouteSearch } from "./api/types/TradeRoute";

export interface Commodity {
  /** Journal and Ardent symbol: "lowtemperaturediamond". */
  symbol: string;
  name: string;
  category: string;
  aliases?: string[];
}

/** The market commodities, by category (data/catalog/commodities.json). */
export const commodities: Commodity[] = catalog as Commodity[];

const squash = (s: string) => s.toLowerCase().replace(/[^a-z0-9]/g, "");

/** The commodity a name, alias or symbol stands for (any case, punctuation ignored). */
export function findCommodity(text: string): Commodity | undefined {
  const k = squash(text);
  if (!k) return undefined;
  return commodities.find((c) => squash(c.name) === k || c.symbol === k || (c.aliases ?? []).some((a) => squash(a) === k));
}

/** Up to `n` commodities whose name or alias contains `text`, names starting with it first. */
export function suggestCommodities(text: string, n = 8): Commodity[] {
  const k = text.trim().toLowerCase();
  if (!k) return [];
  const hit = (c: Commodity) => [c.name, ...(c.aliases ?? [])].some((s) => s.toLowerCase().includes(k));
  const starts = (c: Commodity) => (c.name.toLowerCase().startsWith(k) ? 0 : 1);
  return commodities.filter(hit).sort((a, b) => starts(a) - starts(b) || a.name.localeCompare(b.name)).slice(0, n);
}

/** "211,587". */
export function credits(n: number): string {
  return Math.round(n).toLocaleString("en");
}

/** The search radius the lookup uses when none is picked (astromech_market::trade::DEFAULT_DISTANCE_LY). */
export const DEFAULT_DISTANCE_LY = 100;

/** The data age the lookup uses when none is picked (astromech_market::trade::DEFAULT_MAX_AGE_DAYS). */
export const DEFAULT_MAX_AGE_DAYS = 30;

/** Where to sell or buy each of `symbols` near `origin` (or the commander), best first. */
export async function tradeSearch(side: Side, symbols: string[], origin: Origin = null, filters: Filters | null = null): Promise<TradeSearch> {
  return await call("trade_search", { side, commodities: symbols, origin, filters });
}

/** Where to sell or buy `symbol` anywhere in the galaxy (Ardent's best 100), distances from `origin` or the commander. */
export async function tradeAnywhere(side: Side, symbol: string, origin: Origin = null, filters: Filters | null = null): Promise<TradeSearch> {
  return await call("trade_anywhere", { side, commodity: symbol, origin, filters });
}

/**
 * The first commodity's markets as the search kit's rows. Ardent cuts a nearby search at 1000
 * markets: say so, so a cut answer doesn't look complete. The anywhere lookup has no radius to
 * narrow, and its list is always cut to the best 100 (the sub-heading says so).
 */
export function toResults(s: TradeSearch, anywhere = false): Results {
  const p = s.results[0];
  return {
    origin: s.here.system,
    range: s.range,
    source: "Ardent Insight",
    fetched: new Date((p?.fetched_at ?? 0) * 1000).toISOString(),
    warning: p?.truncated && !anywhere ? `Too many markets for ${findCommodity(p.commodity)?.name ?? p.commodity}: narrow the radius to see them all.` : undefined,
    place: "station",
    main: { head: "Price", unit: "cr" },
    columns: [{ key: "volume", head: s.side === "sell" ? "Demand" : "Stock", unit: "t", width: 88 }],
    rows: (p?.offers ?? []).map((o) => ({
      key: o.market_id,
      system: o.system,
      station: o.station,
      ly: o.ly,
      jumps: o.jumps,
      exact: o.exact,
      pad: o.pad,
      arrival_ls: o.arrival_ls,
      updated_at: o.updated_at,
      main: credits(o.price),
      facts: { volume: credits(o.volume) },
      station_type: o.fleet_carrier ? "Drake-Class Carrier" : o.odyssey ? "Settlement" : null,
    })),
  };
}

/** Where to sell the hold (Cargo.json) near `origin` or the commander. */
export async function tradeCargo(origin: Origin = null, filters: Filters | null = null): Promise<CargoSearch> {
  return await call("trade_cargo", { origin, filters });
}

/** The whole-load stations as the search kit's rows: what each pays for the load, and how much of it it takes. */
export function cargoResults(s: CargoSearch): Results {
  const carried = s.plan.load.reduce((n, l) => n + l.tonnes, 0);
  return {
    origin: s.here.system,
    range: s.range,
    source: "Ardent Insight",
    fetched: new Date(s.plan.fetched_at * 1000).toISOString(),
    place: "station",
    main: { head: "Total", unit: "cr" },
    columns: [{ key: "takes", head: "Takes", unit: "t", width: 112 }],
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
      main: credits(x.total),
      facts: { takes: x.sold === carried ? `all ${credits(carried)}` : `${credits(x.sold)} of ${credits(carried)}` },
      station_type: x.fleet_carrier ? "Drake-Class Carrier" : x.odyssey ? "Settlement" : null,
    })),
  };
}

/** The docked market (Market.json) and the hold against the best nearby prices, with the sell
 *  lookup's filters; null when not docked. The preview answers from a recorded comparison. */
export async function compareMarket(): Promise<MarketCompare | null> {
  return await call("market_compare");
}

/** "+3,921" or "−120": credits per tonne against here. */
export function gainText(n: number): string {
  return `${n < 0 ? "−" : "+"}${credits(Math.abs(n))}`;
}

/** The time model behind the per-hour estimate (astromech_market::route). */
export const ROUTE_ESTIMATE = "50 s a jump, 2 min to dock and trade, supercruise by distance from the star";

/** The current ship's capacity and range, the commander's credits and the station docked at. */
export async function tradeRouteDefaults(): Promise<RouteDefaults> {
  return await call("trade_route_defaults");
}

/** A single-hop or loop route from `origin` or the commander (Spansh; can take a minute). */
export async function tradeRoute(ask: RouteAsk, origin: Origin = null, filters: Filters | null = null): Promise<RouteSearch> {
  return await call("trade_route", { ask, origin, filters });
}
