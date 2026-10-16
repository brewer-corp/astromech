/**
 * Trade lookups (astromech-market): where to sell or buy a commodity near a system, best price
 * first, from Ardent Insight through the shell. The browser preview answers from a recorded
 * search (fixtures/trade.json) with the filters applied roughly.
 */
import { invoke } from "@tauri-apps/api/core";
import catalog from "../../../data/catalog/commodities.json";
import { inTauri } from "./core";
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

const PAD_RANK: Record<Pad, number> = { S: 1, M: 2, L: 3 };

function previewKeeps(o: Offer, f: Filters, now: number): boolean {
  if (f.min_volume != null && o.volume < f.min_volume) return false;
  if (o.ly > (f.max_distance_ly ?? DEFAULT_DISTANCE_LY)) return false;
  if (f.exclude_carriers && o.fleet_carrier) return false;
  if (f.exclude_odyssey && o.odyssey) return false;
  if (f.pad && o.pad && PAD_RANK[o.pad] < PAD_RANK[f.pad]) return false;
  if (f.max_arrival_ls != null && o.arrival_ls != null && o.arrival_ls > f.max_arrival_ls) return false;
  if (f.max_age_days != null && now - Date.parse(o.updated_at) > f.max_age_days * 86400000) return false;
  return true;
}

/** Where to sell or buy each of `symbols` near `origin` (or the commander), best first. */
export async function tradeSearch(side: Side, symbols: string[], origin: Origin = null, filters: Filters | null = null): Promise<TradeSearch> {
  if (inTauri) return await invoke<TradeSearch>("trade_search", { side, commodities: symbols, origin, filters });
  const s = (sample as unknown as Record<Side, TradeSearch>)[side];
  const [p] = s.results;
  if (!p) return s;
  const keep = (o: Offer) => !filters || previewKeeps(o, filters, p.fetched_at * 1000);
  return {
    ...s,
    here: origin ? { ...s.here, system: origin } : s.here,
    results: symbols.map((c) => ({ ...p, commodity: c, offers: p.offers.filter(keep).map((o) => ({ ...o, commodity: c })) })),
  };
}

/** The first commodity's markets as the search kit's rows. */
export function toResults(s: TradeSearch): Results {
  const volume = s.side === "sell" ? "demand" : "stock";
  const p = s.results[0];
  return {
    origin: s.here.system,
    range: s.range,
    source: "Ardent Insight",
    fetched: new Date((p?.fetched_at ?? 0) * 1000).toISOString(),
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
