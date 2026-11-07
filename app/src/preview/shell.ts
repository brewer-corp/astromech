/**
 * The shell's commands as the browser preview answers them: recorded samples (fixtures/), with
 * the arguments applied roughly the way the shell applies them. `?core=empty` answers as a
 * commander with no history (previewData); the network lookups then fail for want of a position
 * unless a system is typed (previewLookup). Synthetic or public data only.
 */
import type { Candidates, Interest } from "../lib/colonies";
import type { BuySearch, ListPlan, Project, Stockist } from "../lib/colonisation";
import { samePlan, type Engineers, type Fleet, type Headline, type Plan, type PlanResult, type StatsComparison, type Trip } from "../lib/engineering";
import type { Nearby } from "../lib/galaxy";
import { LIMIT, oreNamed, type Hotspots, type Reserve, type Want as OreWant } from "../lib/hotspots";
import type { Logbook, Session } from "../lib/logbook";
import type { Cell, Inventory } from "../lib/materials";
import type { SystemView, Unsold } from "../lib/exploration";
import { DOWNLOAD_MB, resetNeutronDownload, type NeutronFile, type NeutronHeader } from "../lib/neutron";
import { samePicks, type OdysseyView, type Pick as OdysseyPick, type Shopping } from "../lib/odyssey";
import type { Overview, Target, TargetHit, Targets } from "../lib/powerplay";
import type { PlotAsk, Plotted, PlotSetup, RouteView, Waypoint } from "../lib/route";
import { DEFAULT_FILTERS, type Filters, type Origin, type Pad } from "../lib/search";
import type { Sites } from "../lib/sites";
import type { DockedHere, OutfittingSearch, Seen } from "../lib/station";
import type { StationsNearby, Want as StationWant } from "../lib/stations";
import { DEFAULT_DISTANCE_LY, DEFAULT_MAX_AGE_DAYS, type CargoPlan, type CargoSearch, type MarketCompare, type Offer, type RouteAsk, type RouteDefaults, type RouteSearch, type Side, type TradeRoute, type TradeSearch } from "../lib/trade";
import type { UnlockTree } from "../lib/unlocks";
import { coreReply, previewData, previewGate, previewLookup } from "./fakecore";
import coloniesSample from "./fixtures/colonies.json";
import projectsSample from "./fixtures/colonisation.json";
import engineeringSample from "./fixtures/engineering.json";
import explorationSample from "./fixtures/exploration.json";
import hotspotsSample from "./fixtures/hotspots.json";
import logbookSample from "./fixtures/logbook.json";
import compareSample from "./fixtures/market-compare.json";
import materialsSample from "./fixtures/materials.json";
import hgeSample from "./fixtures/nearby-hge.json";
import nearbySample from "./fixtures/nearby.json";
import odysseySample from "./fixtures/odyssey.json";
import plotSample from "./fixtures/plot.json";
import powerplaySample from "./fixtures/powerplay.json";
import targetsSample from "./fixtures/powerplay-targets.json";
import sessionSample from "./fixtures/session.json";
import shipsSample from "./fixtures/ships.json";
import sitesSample from "./fixtures/sites.json";
import stationSample from "./fixtures/station.json";
import moduleSample from "./fixtures/stations-module.json";
import shipSample from "./fixtures/stations-ship.json";
import stationsSample from "./fixtures/stations.json";
import tradeSample from "./fixtures/trade.json";
import unlocksSample from "./fixtures/unlocks.json";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type A = Record<string, any>;

const PAD_RANK: Record<Pad, number> = { S: 1, M: 2, L: 3 };
const PADS: Pad[] = ["S", "M", "L"];

/** `here`, moved to the typed system. */
const from = <T extends { system: string }>(here: T, origin: Origin): T => (origin ? { ...here, system: origin } : here);

// ---- Trade and colonisation: recorded searches with the filters applied roughly.
type Market = Pick<Offer, "ly" | "fleet_carrier" | "odyssey" | "pad" | "arrival_ls" | "updated_at"> & { volume?: number };

function keeps(o: Market, f: Filters, now: number): boolean {
  if (f.min_volume != null && o.volume != null && o.volume < f.min_volume) return false;
  if (o.ly > (f.max_distance_ly ?? DEFAULT_DISTANCE_LY)) return false;
  if (f.exclude_carriers && o.fleet_carrier) return false;
  if (f.exclude_odyssey && o.odyssey) return false;
  if (f.pad && o.pad && PAD_RANK[o.pad] < PAD_RANK[f.pad]) return false;
  if (f.max_arrival_ls != null && o.arrival_ls != null && o.arrival_ls > f.max_arrival_ls) return false;
  if (now - Date.parse(o.updated_at) > (f.max_age_days ?? DEFAULT_MAX_AGE_DAYS) * 86400000) return false;
  return true;
}

function tradeSearch({ side, commodities, origin, filters }: A): Promise<TradeSearch> {
  const s = (tradeSample as unknown as Record<Side, TradeSearch>)[side as Side];
  const [p] = s.results;
  if (!p) return Promise.resolve(s);
  const keep = (o: Offer) => !filters || keeps(o, filters as Filters, p.fetched_at * 1000);
  const found = (offers: (o: Offer) => boolean): TradeSearch => ({
    ...s,
    here: from(s.here, origin),
    results: (commodities as string[]).map((c) => ({ ...p, commodity: c, offers: p.offers.filter(offers).map((o) => ({ ...o, commodity: c })) })),
  });
  return previewData("trade_search", () => found(keep), () => found(() => false));
}

function tradeAnywhere({ side, commodity, origin, filters }: A): Promise<TradeSearch> {
  const s = (tradeSample as unknown as { anywhere: Record<Side, TradeSearch> }).anywhere[side as Side];
  const [p] = s.results;
  if (!p) return Promise.resolve(s);
  const keep = (o: Offer) => !filters || keeps(o, { ...(filters as Filters), max_distance_ly: Infinity }, p.fetched_at * 1000);
  const found = (offers: (o: Offer) => boolean): TradeSearch => ({
    ...s,
    here: from(s.here, origin),
    results: [{ ...p, commodity, offers: p.offers.filter(offers).map((o) => ({ ...o, commodity })) }],
  });
  return previewData("trade_anywhere", () => found(keep), () => found(() => false));
}

function tradeCargo({ origin, filters }: A): Promise<CargoSearch> {
  const s = tradeSample as unknown as { cargo: CargoPlan; sell: TradeSearch };
  const now = s.cargo.fetched_at * 1000;
  const keep = (m: Market) => !filters || keeps(m, { ...(filters as Filters), min_volume: null }, now);
  const found = (): CargoSearch => ({
    here: from(s.sell.here, origin),
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

const routeSample = () => (tradeSample as unknown as { route: { defaults: RouteDefaults } & Record<RouteAsk["kind"], TradeRoute> }).route;

function tradeRoute({ ask, origin }: A): Promise<RouteSearch> {
  const kind = (ask as RouteAsk).kind;
  const s = tradeSample as unknown as { sell: TradeSearch };
  const found = (route: TradeRoute): RouteSearch => ({ here: from(s.sell.here, origin), route });
  return previewData("trade_route", () => found(routeSample()[kind]), () => found({ ...routeSample()[kind], legs: [], profit: 0, minutes: 0, per_hour: 0, warning: null }));
}

function colonisationBuy({ origin, filters }: A): Promise<BuySearch> {
  const s = tradeSample as unknown as { list: ListPlan; buy: { here: BuySearch["here"]; range: number } };
  const now = s.list.fetched_at * 1000;
  const keep = (x: Stockist) => !filters || keeps(x, filters as Filters, now);
  const found = (stations: Stockist[]): BuySearch => ({ here: from(s.buy.here, origin), range: s.buy.range, plan: { ...s.list, stations } });
  return previewData("colonisation_buy", () => found(s.list.stations.filter(keep)), () => found([]));
}

// ---- Lookups near a system (Spansh): recorded answers.
function stationsNearby({ want, origin, filters }: A): Promise<StationsNearby> {
  const w = want as StationWant;
  const f = filters as Filters;
  return previewLookup("stations_nearby", origin, () => {
    const s = (w.kind === "module" ? moduleSample : w.kind === "ship" ? shipSample : stationsSample) as unknown as StationsNearby;
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
  });
}

const RESERVE_LEVELS: Record<Reserve, string[]> = { pristine: ["Pristine"], major: ["Pristine", "Major"] };

/** The recorded Painite answer, with the ring type and reserve applied to its rings (other
 * minerals get the same rings). */
function miningHotspots({ want, origin }: A): Promise<Hotspots> {
  const w = want as OreWant;
  return previewLookup("mining_hotspots", origin, () => {
    const s = hotspotsSample as unknown as Hotspots;
    const levels = w.reserve ? RESERVE_LEVELS[w.reserve] : null;
    const rings = s.rings
      .filter((r) => (!w.ring_type || r.ring_type === w.ring_type) && (!levels || (r.reserve != null && levels.includes(r.reserve))))
      .slice(0, LIMIT);
    return { ...s, want: w, name: oreNamed(w.ore)?.name ?? w.ore, rings, here: origin ? { system: origin } : s.here };
  });
}

function galaxyNearby({ query, origin, filters }: A): Promise<Nearby> {
  return previewLookup("galaxy_nearby", origin, () => {
    const s = (query === "famine" ? nearbySample : hgeSample) as unknown as Nearby;
    const days = (filters as Filters | null)?.max_age_days;
    const hits = days == null ? s.hits : s.hits.filter((h) => h.updated_at && Date.parse(s.fetched) - Date.parse(h.updated_at) <= days * 86400000);
    return { ...s, hits, query: { id: query, label: s.query.label }, here: from(s.here, origin) };
  });
}

/** Recorded from Sol. */
function galaxySites({ kind, origin, filters }: A): Promise<Sites> {
  const s = (sitesSample as unknown as Record<string, Sites | undefined>)[kind];
  if (!s) return Promise.reject(new Error(`unknown site kind ${kind}`));
  return previewLookup("galaxy_sites", origin, () => {
    const max = (filters as Filters | null)?.max_arrival_ls;
    const hits = max == null ? s.hits : s.hits.filter((h) => h.arrival_ls == null || h.arrival_ls <= max);
    return { ...s, hits, here: from(s.here, origin) };
  });
}

/** A recorded search around Greae Phoea EM-K d9-675. */
function colonyCandidates({ interest, origin }: A): Promise<Candidates> {
  return previewLookup("colony_candidates", origin, () => {
    const s = (coloniesSample as unknown as Record<Interest, Candidates>)[interest as Interest];
    return origin ? { ...s, here: { ...s.here, system: origin } } : s;
  });
}

// ---- The journal history: the sample commander's.
const matches = (q: string) => (v: { star_system: string; station_name: string | null }) =>
  !q || v.star_system.toLowerCase().includes(q) || (v.station_name ?? "").toLowerCase().includes(q);

/** The sample history searched the way the core searches. */
function searchLogbook(query: string): Logbook {
  const book = logbookSample as Logbook;
  const q = query.trim().toLowerCase();
  const places = book.places.filter(matches(q));
  return { ...book, query: query.trim(), places, matches: places.length, timeline: book.timeline.filter(matches(q)), source: "Sample journal history" };
}

/** A history with no commander yet. */
const noHistory = (query: string): Logbook => ({
  commander: null,
  commanders: [],
  totals: { systems: 0, stations: 0, system_visits: 0, station_visits: 0 },
  query: query.trim(),
  places: [],
  matches: 0,
  timeline: [],
});

/** The sample book searched the way the core searches it: every word must be in the module's
 * symbol or name, and `fsd` stands for `hyperdrive`. */
function searchOutfitting(query: string): OutfittingSearch {
  const words = query.toLowerCase().split(/\s+/).filter(Boolean).map((w) => (w === "fsd" ? "hyperdrive" : w));
  const book = stationSample.book;
  const found = words.length ? (book.found as Seen[]).filter((s) => words.every((w) => `${s.item} ${s.name}`.toLowerCase().includes(w))) : [];
  return { stations: book.stations, snapshots: book.snapshots, found, source: "Sample outfitting history" };
}

const NOT_DOCKED: DockedHere = {
  docked: false, station: null, system: null, market_id: null, station_type: null, services: [], economies: [], faction: null,
  government: null, allegiance: null, landing_pads: null, dist_from_star_ls: null, max_pad: null, ship: null, outfitting: null,
};

/** The same layout with nothing held, no snapshot and nowhere known. */
function nothingHeld(): Inventory {
  const inv = structuredClone(materialsSample) as Inventory;
  const zero = (c: Cell) => ({ ...c, count: 0 });
  return {
    ...inv,
    source: "No journal yet",
    snapshot_ts: null,
    kinds: inv.kinds.map((k) => ({ ...k, held: 0, groups: k.groups.map((g) => ({ ...g, cells: g.cells.map(zero) })) })),
    other: [],
    here: null,
    farm: [],
  };
}

// ---- Engineering: the sample fleet; there is no planner, only the sample's precomputed plans.
const samplePlan = engineeringSample.plan as unknown as PlanResult;

/** The sample journal's fleet, with the synthetic combat Krait whose stats the sample has (stored). */
function sampleFleet(): Fleet {
  const f = engineeringSample.fleet as unknown as Fleet;
  const s = shipsSample.fleet as unknown as Fleet;
  const extra = s.ships.map((x) => ({ ...x, current: false }));
  return { ...f, ships: [...f.ships, ...extra], groups: { ...f.groups, ...s.groups } };
}

// ---- Search filters: kept in the browser; the pretend ship has a medium pad.
const filtersKey = (lookup: string) => `astromech.search.${lookup}`;

function searchFilters({ lookup }: A): Filters {
  let saved: Partial<Filters> = {};
  try {
    saved = JSON.parse(localStorage.getItem(filtersKey(lookup)) ?? "{}") as Partial<Filters>;
  } catch {
    saved = {};
  }
  const f = { ...DEFAULT_FILTERS, ...saved };
  return f.pad_set ? f : { ...f, pad: "M" };
}

/** Public system names for the autocomplete (prefixes only; Spansh's is fuzzy). */
const SYSTEMS = ["Sol", "Shinrarta Dezhra", "Shingon", "Shinigami", "Deciat", "Colonia", "Achenar", "Alioth", "Maia"];

// ---- The neutron star file: none until Download, then a short fake download.
const NEUTRON_FILE: NeutronHeader = { cell_ly: 128, stars: 5045154, cells: 1645100, built: 1792627200 };
let neutron: NeutronFile = { file: null, error: null };

/** Back to "not downloaded", with no download in flight (tests). */
export function resetPreviewNeutron(): void {
  neutron = { file: null, error: null };
  resetNeutronDownload();
}

async function neutronDownload(progress: (p: unknown) => void): Promise<NeutronFile> {
  await previewGate("neutron_download");
  const total = DOWNLOAD_MB * 1_000_000;
  for (let i = 1; i <= 4; i++) {
    await new Promise((r) => setTimeout(r, 40));
    progress({ done: (total * i) / 4, total });
  }
  neutron = { file: NEUTRON_FILE, error: null };
  return neutron;
}

/** Plot from fixtures/plot.json: neutron routes "on this PC" once the file is downloaded. */
async function routePlot({ ask }: A): Promise<{ plotted: Plotted; view: RouteView }> {
  const neutronRoute = neutron.file ? plotSample.local : plotSample.neutron;
  const plotted = ((ask as PlotAsk).plotter === "neutron" ? neutronRoute : plotSample.exact) as Plotted;
  const r = await coreReply("route_set", { name: plotted.name, source: plotted.source, waypoints: plotted.waypoints as Waypoint[] });
  if (!r.ok || !r.view) throw new Error(r.error ?? "the core didn't answer");
  return { plotted, view: r.view as RouteView };
}

/** Shell commands by name. `publish` sends a push (a command's progress channel). */
export function shellCommands(publish: (topic: string, payload: unknown) => void): Record<string, (args: A) => unknown> {
  return {
    trade_search: tradeSearch,
    trade_anywhere: tradeAnywhere,
    trade_cargo: tradeCargo,
    market_compare: () => previewData("market_compare", () => compareSample as unknown as MarketCompare, () => null),
    trade_route_defaults: () =>
      previewData("trade_route_defaults", () => routeSample().defaults, () => ({ capacity: null, credits: null, range: null, station: null, system: null })),
    trade_route: tradeRoute,
    colonisation_projects: () => previewData("colonisation_projects", () => projectsSample as unknown as Project[], () => []),
    colonisation_buy: colonisationBuy,
    stations_nearby: stationsNearby,
    mining_hotspots: miningHotspots,
    galaxy_nearby: galaxyNearby,
    galaxy_sites: galaxySites,
    colony_candidates: colonyCandidates,
    logbook: ({ query }) => previewData("logbook", () => searchLogbook(query), () => noHistory(query)),
    session_summary: () => previewData("session_summary", () => ({ ...(sessionSample as Session), source: "Sample journal history" }), () => null),
    docked_here: () => previewData("docked_here", () => ({ ...(stationSample.here as DockedHere), source: "Sample docking" }), () => NOT_DOCKED),
    outfitting_search: ({ query }) => previewData("outfitting_search", () => searchOutfitting(query), () => ({ stations: 0, snapshots: 0, found: [] })),
    exploration_system: () =>
      previewData("exploration_system", () => ({ ...(explorationSample.system as SystemView), source: "Sample system" }), (): SystemView => ({
        system: null, system_address: null, population: 0, body_count: null, all_found: false, bodies: [], value: 0, value_if_mapped: 0,
      })),
    exploration_unsold: () => previewData("exploration_unsold", () => ({ ...(explorationSample.unsold as Unsold), source: "Sample history" }), () => null),
    materials_inventory: () => previewData("materials_inventory", () => materialsSample as Inventory, nothingHeld),
    // Nothing unlocked yet: every engineer locked, none known.
    engineering_unlocks: () =>
      previewData("engineering_unlocks", () => unlocksSample as UnlockTree, () => ({
        nodes: (unlocksSample as UnlockTree).nodes.map((n) => ({ ...n, status: "locked" as const, rank: null, next: false })),
      })),
    powerplay_overview: () => {
      const o = powerplaySample as unknown as Overview;
      const unpledged: Overview = {
        standing: { power: null, rank: null, merits: null, time_pledged_secs: null, session_merits: 0, week_merits: 0, week_ends: o.standing.week_ends, next: null },
        here: null,
        system: null,
      };
      return previewData("powerplay_overview", () => o, () => unpledged);
    },
    powerplay_targets: ({ target, origin, within_ly: withinLy }) => {
      const s = (targetsSample as unknown as Record<Target, Targets>)[target as Target];
      const shown = (hits: TargetHit[]) => ({ ...s, hits, within_ly: withinLy, here: from(s.here, origin) });
      return previewData("powerplay_targets", () => shown(s.hits.filter((h) => h.ly <= withinLy)), () => shown([]));
    },
    odyssey_view: () =>
      previewData("odyssey_view", () => odysseySample.view as unknown as OdysseyView, () => ({
        ...(odysseySample.view as unknown as OdysseyView), source: "No journal yet", locker_ts: null, backpack_ts: null, items: [], owned: [],
      })),
    // Only the sample picks have a shopping list.
    odyssey_shopping: ({ picks }) =>
      previewData("odyssey_shopping", () => (samePicks(picks as OdysseyPick[], odysseySample.picks as OdysseyPick[]) ? (odysseySample.shopping as unknown as Shopping) : null), () => null),
    engineering_fleet: () => previewData("engineering_fleet", sampleFleet, () => ({ source: "No journal yet", ships: [], stored_modules: [], groups: {} })),
    engineering_plan: ({ plan }) => previewData("engineering_plan", () => ((plan as Plan).ship_id === samplePlan.ship_id ? samplePlan : null), () => null),
    engineering_stats: ({ plan }) =>
      previewData("engineering_stats", () => {
        const c = shipsSample.cases.find((c) => samePlan(c.plan as unknown as Plan, plan as Plan));
        return c ? (c.stats as unknown as StatsComparison) : null;
      }, () => null),
    engineering_headlines: () => previewData("engineering_headlines", () => shipsSample.headlines as unknown as Record<string, Headline>, () => ({})),
    engineering_engineers: () => previewData("engineering_engineers", () => engineeringSample.engineers as unknown as Engineers, () => ({ engineers: [], here: null })),
    engineering_trip: ({ plan }) =>
      previewData("engineering_trip", () => ((plan as Plan).ship_id === samplePlan.ship_id ? (engineeringSample.trip as unknown as Trip) : null), () => null),
    search_filters: searchFilters,
    set_search_filters: ({ lookup, filters }) => localStorage.setItem(filtersKey(lookup), JSON.stringify(filters)),
    galaxy_systems: ({ prefix }) => SYSTEMS.filter((s) => s.toLowerCase().startsWith(String(prefix).toLowerCase())),
    route_plot_ship: () => plotSample.ship as PlotSetup,
    route_plot: routePlot,
    neutron_file: async () => {
      await previewGate("neutron_file");
      return neutron;
    },
    neutron_check: async () => {
      await previewGate("neutron_check");
      return { newer: false };
    },
    neutron_download: () => neutronDownload((p) => publish("neutron_download", p)),
    shell_info: () => ({ version: "dev" }),
    open_external: ({ target }) => void window.open(target, "_blank"),
  };
}
