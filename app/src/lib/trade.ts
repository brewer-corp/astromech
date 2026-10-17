/**
 * Trade lookups (astromech-market): where to sell or buy a commodity near a system or anywhere
 * in the galaxy, best price first, and where to sell the cargo in the hold, from Ardent Insight through the shell. The
 * browser preview answers from recorded searches and a fixture hold (fixtures/trade.json) with
 * the filters applied roughly.
 */
import { invoke } from "@tauri-apps/api/core";
import catalog from "../../../data/catalog/commodities.json";
import { inTauri } from "./core";
import { previewData } from "./fakecore";
import compareSample from "./fixtures/market-compare.json";
import sample from "./fixtures/trade.json";
import type { Filters, Origin, Pad, Results } from "./search";

export type Side = "sell" | "buy";

export interface Commodity {
  /** Journal and Ardent symbol: "lowtemperaturediamond". */
  symbol: string;
  name: string;
  category: string;
  aliases?: string[];
}

/** One market for one commodity (astromech_market::Offer). */
export interface Offer {
  commodity: string;
  market_id: number;
  station: string;
  station_type: string | null;
  system: string;
  ly: number;
  jumps: number;
  exact: boolean;
  /** Credits per tonne: paid (selling) or asked (buying). */
  price: number;
  /** Tonnes of demand (selling) or stock (buying). */
  volume: number;
  pad: Pad | null;
  arrival_ls: number | null;
  fleet_carrier: boolean;
  odyssey: boolean;
  updated_at: string;
}

export interface Prices {
  commodity: string;
  side: Side;
  /** Unix seconds. */
  fetched_at: number;
  truncated: boolean;
  offers: Offer[];
}

export interface TradeSearch {
  side: Side;
  here: { system: string; pos: [number, number, number] };
  range: number;
  results: Prices[];
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

const PAD_RANK: Record<Pad, number> = { S: 1, M: 2, L: 3 };

type Market = Pick<Offer, "ly" | "fleet_carrier" | "odyssey" | "pad" | "arrival_ls" | "updated_at"> & { volume?: number };

function previewKeeps(o: Market, f: Filters, now: number): boolean {
  if (f.min_volume != null && o.volume != null && o.volume < f.min_volume) return false;
  if (o.ly > (f.max_distance_ly ?? DEFAULT_DISTANCE_LY)) return false;
  if (f.exclude_carriers && o.fleet_carrier) return false;
  if (f.exclude_odyssey && o.odyssey) return false;
  if (f.pad && o.pad && PAD_RANK[o.pad] < PAD_RANK[f.pad]) return false;
  if (f.max_arrival_ls != null && o.arrival_ls != null && o.arrival_ls > f.max_arrival_ls) return false;
  if (now - Date.parse(o.updated_at) > (f.max_age_days ?? DEFAULT_MAX_AGE_DAYS) * 86400000) return false;
  return true;
}

/** Where to sell or buy each of `symbols` near `origin` (or the commander), best first. */
export async function tradeSearch(side: Side, symbols: string[], origin: Origin = null, filters: Filters | null = null): Promise<TradeSearch> {
  if (inTauri) return await invoke<TradeSearch>("trade_search", { side, commodities: symbols, origin, filters });
  const s = (sample as unknown as Record<Side, TradeSearch>)[side];
  const [p] = s.results;
  if (!p) return s;
  const keep = (o: Offer) => !filters || previewKeeps(o, filters, p.fetched_at * 1000);
  const found = (offers: (o: Offer) => boolean): TradeSearch => ({
    ...s,
    here: origin ? { ...s.here, system: origin } : s.here,
    results: symbols.map((c) => ({ ...p, commodity: c, offers: p.offers.filter(offers).map((o) => ({ ...o, commodity: c })) })),
  });
  return previewData("trade_search", () => found(keep), () => found(() => false));
}

/** Where to sell or buy `symbol` anywhere in the galaxy (Ardent's best 100), distances from `origin` or the commander. */
export async function tradeAnywhere(side: Side, symbol: string, origin: Origin = null, filters: Filters | null = null): Promise<TradeSearch> {
  if (inTauri) return await invoke<TradeSearch>("trade_anywhere", { side, commodity: symbol, origin, filters });
  const s = (sample as unknown as { anywhere: Record<Side, TradeSearch> }).anywhere[side];
  const [p] = s.results;
  if (!p) return s;
  const keep = (o: Offer) => !filters || previewKeeps(o, { ...filters, max_distance_ly: Infinity }, p.fetched_at * 1000);
  const found = (offers: (o: Offer) => boolean): TradeSearch => ({
    ...s,
    here: origin ? { ...s.here, system: origin } : s.here,
    results: [{ ...p, commodity: symbol, offers: p.offers.filter(offers).map((o) => ({ ...o, commodity: symbol })) }],
  });
  return previewData("trade_anywhere", () => found(keep), () => found(() => false));
}

/**
 * The first commodity's markets as the search kit's rows. Ardent cuts a nearby search at 1000
 * markets: say so, so a cut answer doesn't look complete. The anywhere lookup has no radius to
 * narrow, and its list is always cut to the best 100 (the sub-heading says so).
 */
export function toResults(s: TradeSearch, anywhere = false): Results {
  const volume = s.side === "sell" ? "demand" : "stock";
  const p = s.results[0];
  return {
    origin: s.here.system,
    range: s.range,
    source: "Ardent Insight",
    fetched: new Date((p?.fetched_at ?? 0) * 1000).toISOString(),
    warning: p?.truncated && !anywhere ? `Too many markets for ${findCommodity(p.commodity)?.name ?? p.commodity}: narrow the radius to see them all.` : undefined,
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
      lead: `${credits(o.price)} cr`,
      detail: [`${volume} ${credits(o.volume)}`, o.fleet_carrier ? "fleet carrier" : "", o.odyssey ? "Odyssey settlement" : ""].filter(Boolean).join(" · "),
    })),
  };
}

/** One commodity sold at a station (astromech_market::cargo::SaleLine). */
export interface SaleLine {
  commodity: string;
  name: string;
  carried: number;
  /** Tonnes the station takes: what is carried, capped by its demand. */
  sold: number;
  price: number;
  proceeds: number;
}

/** A station's offer for the whole load. */
export interface StationSale {
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
  total: number;
  sold: number;
  lines: SaleLine[];
}

export interface CargoPlan {
  load: { symbol: string; name: string; tonnes: number }[];
  kept: { name: string; tonnes: number; reason: string }[];
  stations: StationSale[];
  per_commodity: { commodity: string; name: string; carried: number; best: Offer | null; partial?: Offer | null }[];
  fetched_at: number;
}

export interface CargoSearch {
  here: { system: string; pos: [number, number, number] };
  range: number;
  plan: CargoPlan;
}

/** Where to sell the hold (Cargo.json) near `origin` or the commander. */
export async function tradeCargo(origin: Origin = null, filters: Filters | null = null): Promise<CargoSearch> {
  if (inTauri) return await invoke<CargoSearch>("trade_cargo", { origin, filters });
  const s = sample as unknown as { cargo: CargoPlan; sell: TradeSearch };
  const now = s.cargo.fetched_at * 1000;
  const keep = (m: Market) => !filters || previewKeeps(m, { ...filters, min_volume: null }, now);
  const found = (): CargoSearch => ({
    here: origin ? { ...s.sell.here, system: origin } : s.sell.here,
    range: s.sell.range,
    plan: {
      ...s.cargo,
      stations: s.cargo.stations.filter(keep),
      per_commodity: s.cargo.per_commodity.map((c) => ({
        ...c,
        best: c.best && keep(c.best) ? c.best : null,
        partial: c.partial && keep(c.partial) ? c.partial : null,
      })),
    },
  });
  // An empty hold: nothing to sell, so nothing to look up.
  const emptyHold = (): CargoSearch => ({ ...found(), plan: { ...s.cargo, load: [], stations: [], per_commodity: [], kept: [] } });
  return previewData("trade_cargo", found, emptyHold);
}

/** The whole-load stations as the search kit's rows. */
export function cargoResults(s: CargoSearch): Results {
  const carried = s.plan.load.reduce((n, l) => n + l.tonnes, 0);
  return {
    origin: s.here.system,
    range: s.range,
    source: "Ardent Insight",
    fetched: new Date(s.plan.fetched_at * 1000).toISOString(),
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
      lead: `${credits(x.total)} cr`,
      detail: [x.sold === carried ? `takes all ${credits(carried)} t` : `takes ${credits(x.sold)} of ${credits(carried)} t`, x.fleet_carrier ? "fleet carrier" : "", x.odyssey ? "Odyssey settlement" : ""]
        .filter(Boolean)
        .join(" · "),
    })),
  };
}

/** One commodity at the docked market against the best nearby price (astromech_market::here::Line). */
export interface MarketLine {
  commodity: string;
  name: string;
  /** Tonnes carried that can be sold (0 for goods for sale here). */
  carried: number;
  /** Whether nearby prices were asked for (carried goods past the dozen per dock aren't). */
  checked: boolean;
  /** What this station pays; null: it doesn't buy it. */
  sell_here: number | null;
  /** What this station asks; null: not for sale here. */
  buy_here: number | null;
  stock: number;
  /** The best price a nearby station pays (for the carried, one with demand for all of it). */
  best: Offer | null;
  /** Credits per tonne more than here (paid here for the carried, asked here for the goods for sale). */
  gain: number | null;
}

/** The docked market and the hold against nearby prices (astromech_market::here::Comparison). */
export interface MarketCompare {
  market_id: number | null;
  station: string;
  system: string;
  /** When the game wrote Market.json. */
  seen: string;
  carried: MarketLine[];
  /** The biggest gain first. */
  for_sale: MarketLine[];
  /** Goods for sale here that weren't looked up. */
  unchecked: string[];
  kept: { name: string; tonnes: number; reason: string }[];
  fetched_at: number;
}

/** The docked market (Market.json) and the hold against the best nearby prices, with the sell
 *  lookup's filters; null when not docked. The preview answers from a recorded comparison. */
export async function compareMarket(): Promise<MarketCompare | null> {
  if (inTauri) return await invoke<MarketCompare | null>("market_compare");
  return previewData("market_compare", () => compareSample as unknown as MarketCompare, () => null);
}

/** "+3,921" or "−120": credits per tonne against here. */
export function gainText(n: number): string {
  return `${n < 0 ? "−" : "+"}${credits(Math.abs(n))}`;
}

export type RouteKind = "single" | "loop";

/** What a trade route is asked for (astromech_market::route::Ask). */
export interface RouteAsk {
  kind: RouteKind;
  /** The station to start from; null: the docked one, else the one nearest the star. */
  station: string | null;
  capacity: number;
  credits: number;
  max_hop_ly: number;
}

/** The current ship's and commander's numbers (astromech_market::route::Defaults). */
export interface RouteDefaults {
  capacity: number | null;
  credits: number | null;
  range: number | null;
  /** The station docked at. */
  station: string | null;
  system: string | null;
}

export interface RouteStop {
  station: string;
  system: string;
  market_id: number;
  arrival_ls: number | null;
  /** The largest landing pad (Ardent Insight); null: not known. */
  pad: Pad | null;
}

export interface RouteCargo {
  name: string;
  tonnes: number;
  buy: number;
  sell: number;
  profit_per_t: number;
  profit: number;
}

export interface RouteLeg {
  from: RouteStop;
  to: RouteStop;
  ly: number;
  jumps: number;
  /** Empty: nothing worth carrying (a loop's way back). */
  cargo: RouteCargo[];
  profit: number;
  profit_per_t: number;
  /** "Spansh" or "Ardent Insight". */
  source: string;
  seconds: number;
}

export interface TradeRoute {
  kind: RouteKind;
  /** Empty: nothing profitable within the limits. */
  legs: RouteLeg[];
  profit: number;
  /** One run back to the start (a single hop flies back empty), estimated. */
  minutes: number;
  /** Estimated: Spansh gives no figure per hour. */
  per_hour: number;
  /** A stop the ship can't land at. */
  warning: string | null;
  range: number;
  fetched_at: number;
}

export interface RouteSearch {
  here: { system: string; pos: [number, number, number] };
  route: TradeRoute;
}

/** The time model behind the per-hour estimate (astromech_market::route). */
export const ROUTE_ESTIMATE = "50 s a jump, 2 min to dock and trade, supercruise by distance from the star";

type RouteSample = { defaults: RouteDefaults } & Record<RouteKind, TradeRoute>;
const routeSample = () => (sample as unknown as { route: RouteSample; sell: TradeSearch }).route;

/** The current ship's capacity and range, the commander's credits and the station docked at. */
export async function tradeRouteDefaults(): Promise<RouteDefaults> {
  if (inTauri) return await invoke<RouteDefaults>("trade_route_defaults");
  return previewData("trade_route_defaults", () => routeSample().defaults, () => ({ capacity: null, credits: null, range: null, station: null, system: null }));
}

/** A single-hop or loop route from `origin` or the commander (Spansh; can take a minute). */
export async function tradeRoute(ask: RouteAsk, origin: Origin = null, filters: Filters | null = null): Promise<RouteSearch> {
  if (inTauri) return await invoke<RouteSearch>("trade_route", { ask, origin, filters });
  const s = sample as unknown as { sell: TradeSearch };
  const found = (route: TradeRoute): RouteSearch => ({ here: origin ? { ...s.sell.here, system: origin } : s.sell.here, route });
  return previewData("trade_route", () => found(routeSample()[ask.kind]), () => found({ ...routeSample()[ask.kind], legs: [], profit: 0, minutes: 0, per_hour: 0, warning: null }));
}
