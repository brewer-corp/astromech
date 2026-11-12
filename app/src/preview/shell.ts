/**
 * The command layer's own commands (astromech-api's) as the browser preview answers them:
 * recorded samples (fixtures/), checked against the core's reply types (recorded.ts), with the
 * arguments applied roughly the way the core applies them. `?core=empty` answers as a commander
 * with no history (previewData); the network lookups then fail for want of a position unless a
 * system is typed (previewLookup). Synthetic or public data only.
 */
import type { CargoItem } from "../lib/api/types/CargoItem";
import type { Header } from "../lib/api/types/Header";
import type { Candidates, Interest } from "../lib/colonies";
import type { BuySearch, ListPlan, Project, Stockist } from "../lib/colonisation";
import { samePlan, type Engineers, type Fleet, type Headline, type Plan, type PlanResult, type StatsComparison, type Trip } from "../lib/engineering";
import type { SystemView, Unsold } from "../lib/exploration";
import type { Nearby } from "../lib/galaxy";
import { LIMIT, oreNamed, type Hotspots, type Reserve } from "../lib/hotspots";
import type { Logbook, Session } from "../lib/logbook";
import type { Cell, Inventory } from "../lib/materials";
import { DOWNLOAD_MB, resetNeutronDownload, type NeutronFile } from "../lib/neutron";
import { samePicks, type OdysseyView, type Pick as OdysseyPick, type Shopping } from "../lib/odyssey";
import type { Overview, Target, Targets } from "../lib/powerplay";
import type { Plotted, PlotSetup, RouteView } from "../lib/route";
import { DEFAULT_FILTERS, type Filters, type Pad } from "../lib/search";
import type { Sites } from "../lib/sites";
import type { DockedHere, OutfittingSearch } from "../lib/station";
import type { StationsNearby } from "../lib/stations";
import { DEFAULT_DISTANCE_LY, DEFAULT_MAX_AGE_DAYS, type CargoPlan, type CargoSearch, type MarketCompare, type Offer, type RouteDefaults, type RouteKind, type RouteSearch, type Side, type TradeRoute, type TradeSearch } from "../lib/trade";
import type { WindowCommand } from "../lib/transport";
import type { UnlockTree } from "../lib/unlocks";
import { coreReply, fakeScenario, previewData, previewGate, previewLookup } from "./fakecore";
import coloniesFixture from "./fixtures/colonies.json";
import projectsFixture from "./fixtures/colonisation.json";
import engineeringFixture from "./fixtures/engineering.json";
import explorationFixture from "./fixtures/exploration.json";
import hotspotsFixture from "./fixtures/hotspots.json";
import logbookFixture from "./fixtures/logbook.json";
import compareFixture from "./fixtures/market-compare.json";
import materialsFixture from "./fixtures/materials.json";
import hgeFixture from "./fixtures/nearby-hge.json";
import nearbyFixture from "./fixtures/nearby.json";
import odysseyFixture from "./fixtures/odyssey.json";
import plotFixture from "./fixtures/plot.json";
import powerplayFixture from "./fixtures/powerplay.json";
import targetsFixture from "./fixtures/powerplay-targets.json";
import sessionFixture from "./fixtures/session.json";
import shipsFixture from "./fixtures/ships.json";
import sitesFixture from "./fixtures/sites.json";
import stationFixture from "./fixtures/station.json";
import moduleFixture from "./fixtures/stations-module.json";
import shipFixture from "./fixtures/stations-ship.json";
import stationsFixture from "./fixtures/stations.json";
import tradeFixture from "./fixtures/trade.json";
import unlocksFixture from "./fixtures/unlocks.json";
import { labelled, recorded, type Handler, type Handlers } from "./recorded";

// ---- The samples, each as the reply type it holds.
const coloniesSample = recorded<Record<Interest, Candidates>>(coloniesFixture);
const projectsSample = recorded<Project[]>(projectsFixture);
const engineeringSample = recorded<{ engineers: Engineers; fleet: Fleet; plan: PlanResult; plan_input: Plan; trip: Trip }>(engineeringFixture);
const explorationSample = recorded<{ system: SystemView; unsold: Unsold }>(explorationFixture);
const hotspotsSample = recorded<Hotspots>(hotspotsFixture);
const logbookSample = recorded<Logbook>(logbookFixture);
const compareSample = recorded<MarketCompare>(compareFixture);
const materialsSample = recorded<Inventory>(materialsFixture);
const hgeSample = recorded<Nearby>(hgeFixture);
const nearbySample = recorded<Nearby>(nearbyFixture);
const odysseySample = recorded<{ picks: OdysseyPick[]; shopping: Shopping; view: OdysseyView }>(odysseyFixture);
const plotSample = recorded<{ ship: PlotSetup; neutron: Plotted; exact: Plotted; local: Plotted }>(plotFixture);
const powerplaySample = recorded<Overview>(powerplayFixture);
const targetsSample = recorded<Record<Target, Targets>>(targetsFixture);
const sessionSample = recorded<Session>(sessionFixture);
const shipsSample = recorded<{ cases: { plan: Plan; stats: StatsComparison }[]; fleet: Fleet; headlines: Record<number, Headline> }>(shipsFixture);
const sitesSample = recorded<Record<string, Sites | undefined>>(sitesFixture);
const stationSample = recorded<{ here: DockedHere; book: OutfittingSearch }>(stationFixture);
const moduleSample = recorded<StationsNearby>(moduleFixture);
const shipSample = recorded<StationsNearby>(shipFixture);
const stationsSample = recorded<StationsNearby>(stationsFixture);
const tradeSample = recorded<
  Record<Side, TradeSearch> & {
    anywhere: Record<Side, TradeSearch>;
    cargo: CargoPlan;
    hold: CargoItem[];
    list: ListPlan;
    route: { defaults: RouteDefaults } & Record<RouteKind, TradeRoute>;
  }
>(tradeFixture);
const unlocksSample = recorded<UnlockTree>(unlocksFixture);

/** The preview's answer to command `K` (recorded.ts). */
type H<K extends WindowCommand> = Handler<K>;

const PAD_RANK: Record<Pad, number> = { S: 1, M: 2, L: 3 };
const PADS: Pad[] = ["S", "M", "L"];

/** `here`, moved to the typed system. */
const from = <T extends { system: string }>(here: T, origin?: string | null): T => (origin ? { ...here, system: origin } : here);

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

const tradeSearch: H<"trade_search"> = ({ side, commodities, origin, filters }) => {
  const s = tradeSample[side];
  const [p] = s.results;
  if (!p) return Promise.resolve(s);
  const keep = (o: Offer) => !filters || keeps(o, filters, p.fetched_at * 1000);
  const found = (offers: (o: Offer) => boolean): TradeSearch => ({
    ...s,
    here: from(s.here, origin),
    results: commodities.map((c) => ({ ...p, commodity: c, offers: p.offers.filter(offers).map((o) => ({ ...o, commodity: c })) })),
  });
  return previewData("trade_search", () => found(keep), () => found(() => false));
};

const tradeAnywhere: H<"trade_anywhere"> = ({ side, commodity, origin, filters }) => {
  const s = tradeSample.anywhere[side];
  const [p] = s.results;
  if (!p) return Promise.resolve(s);
  const keep = (o: Offer) => !filters || keeps(o, { ...filters, max_distance_ly: Infinity }, p.fetched_at * 1000);
  const found = (offers: (o: Offer) => boolean): TradeSearch => ({
    ...s,
    here: from(s.here, origin),
    results: [{ ...p, commodity, offers: p.offers.filter(offers).map((o) => ({ ...o, commodity })) }],
  });
  return previewData("trade_anywhere", () => found(keep), () => found(() => false));
};

const tradeCargo: H<"trade_cargo"> = ({ origin, filters }) => {
  const s = tradeSample;
  const now = s.cargo.fetched_at * 1000;
  const keep = (m: Market) => !filters || keeps(m, { ...filters, min_volume: null }, now);
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
    hold: s.hold,
  });
  // An empty hold: nothing to sell, so nothing to look up.
  const emptyHold = (): CargoSearch => ({ ...found(), plan: { ...s.cargo, load: [], stations: [], per_commodity: [], kept: [] }, hold: [] });
  return previewData("trade_cargo", found, emptyHold);
};

const tradeRoute: H<"trade_route"> = ({ ask, origin }) => {
  const found = (route: TradeRoute): RouteSearch => ({ here: from(tradeSample.sell.here, origin), route });
  const recordedRoute = tradeSample.route[ask.kind];
  return previewData("trade_route", () => found(recordedRoute), () => found({ ...recordedRoute, legs: [], profit: 0, minutes: 0, per_hour: 0, warning: null }));
};

const colonisationBuy: H<"colonisation_buy"> = ({ origin, filters }) => {
  const s = tradeSample;
  const now = s.list.fetched_at * 1000;
  const keep = (x: Stockist) => !filters || keeps(x, filters, now);
  const found = (stations: Stockist[]): BuySearch => ({ here: from(s.buy.here, origin), range: s.buy.range, plan: { ...s.list, stations } });
  return previewData("colonisation_buy", () => found(s.list.stations.filter(keep)), () => found([]));
};

// ---- Lookups near a system (Spansh): recorded answers.
const stationsNearby: H<"stations_nearby"> = ({ want, origin, filters }) => {
  const f = filters ?? DEFAULT_FILTERS;
  return previewLookup("stations_nearby", origin, () => {
    const s = want.kind === "module" ? moduleSample : want.kind === "ship" ? shipSample : stationsSample;
    const fetched = Date.parse(s.fetched);
    const hits = s.hits.filter(
      (h) =>
        (f.pad == null || h.pad == null || PADS.indexOf(h.pad) >= PADS.indexOf(f.pad)) &&
        (f.max_arrival_ls == null || h.arrival_ls == null || h.arrival_ls <= f.max_arrival_ls) &&
        (f.max_age_days == null || (h.updated_at != null && fetched - Date.parse(h.updated_at) <= f.max_age_days * 86400000)) &&
        !(f.exclude_carriers && h.station_type === "Drake-Class Carrier") &&
        !(f.exclude_odyssey && h.station_type === "Settlement"),
    );
    return { ...s, filters: f, hits, here: from(s.here, origin) };
  });
};

const RESERVE_LEVELS: Record<Reserve, string[]> = { pristine: ["Pristine"], major: ["Pristine", "Major"] };

/** The recorded Painite answer, with the ring type and reserve applied to its rings (other
 * minerals get the same rings). */
const miningHotspots: H<"mining_hotspots"> = ({ want: w, origin }) =>
  previewLookup("mining_hotspots", origin, () => {
    const s = hotspotsSample;
    const levels = w.reserve ? RESERVE_LEVELS[w.reserve] : null;
    const rings = s.rings
      .filter((r) => (!w.ring_type || r.ring_type === w.ring_type) && (!levels || (r.reserve != null && levels.includes(r.reserve))))
      .slice(0, LIMIT);
    return { ...s, want: w, name: oreNamed(w.ore)?.name ?? w.ore, rings, here: from(s.here, origin) };
  });

const galaxyNearby: H<"galaxy_nearby"> = ({ query, origin, filters }) =>
  previewLookup("galaxy_nearby", origin, () => {
    const s = query === "famine" ? nearbySample : hgeSample;
    const days = filters?.max_age_days;
    const hits = days == null ? s.hits : s.hits.filter((h) => h.updated_at && Date.parse(s.fetched) - Date.parse(h.updated_at) <= days * 86400000);
    return { ...s, hits, query: { ...s.query, id: query }, here: from(s.here, origin) };
  });

/** Recorded from Sol. */
const galaxySites: H<"galaxy_sites"> = ({ kind, origin, filters }) => {
  const s = sitesSample[kind];
  if (!s) return Promise.reject(new Error(`unknown site kind ${kind}`));
  return previewLookup("galaxy_sites", origin, () => {
    const max = filters?.max_arrival_ls;
    const hits = max == null ? s.hits : s.hits.filter((h) => h.arrival_ls == null || h.arrival_ls <= max);
    return { ...s, hits, here: from(s.here, origin) };
  });
};

/** A recorded search around Greae Phoea EM-K d9-675. */
const colonyCandidates: H<"colony_candidates"> = ({ interest, origin }) =>
  previewLookup("colony_candidates", origin, () => {
    const s = coloniesSample[interest];
    return { ...s, here: from(s.here, origin) };
  });

// ---- The journal history: the sample commander's.
const matches = (q: string) => (v: { star_system: string; station_name: string | null }) =>
  !q || v.star_system.toLowerCase().includes(q) || (v.station_name ?? "").toLowerCase().includes(q);

/** The sample history searched the way the core searches. */
function searchLogbook(query: string): Logbook {
  const book = logbookSample;
  const q = query.trim().toLowerCase();
  const places = book.places.filter(matches(q));
  return labelled({ ...book, query: query.trim(), places, matches: places.length, timeline: book.timeline.filter(matches(q)) }, "Sample journal history");
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
  const found = words.length ? book.found.filter((s) => words.every((w) => `${s.item} ${s.name}`.toLowerCase().includes(w))) : [];
  return labelled({ stations: book.stations, snapshots: book.snapshots, found }, "Sample outfitting history");
}

const NOT_DOCKED: DockedHere = {
  docked: false, station: null, system: null, market_id: null, station_type: null, services: [], economies: [], faction: null,
  government: null, allegiance: null, landing_pads: null, dist_from_star_ls: null, max_pad: null, ship: null, outfitting: null,
};

/** The same layout with nothing held, no snapshot and nowhere known. */
function nothingHeld(): Inventory {
  // `here` is absent when nowhere is known (the core skips it).
  const inv = structuredClone(materialsSample);
  delete inv.here;
  const zero = (c: Cell) => ({ ...c, count: 0 });
  return {
    ...inv,
    source: "No journal yet",
    snapshot_ts: null,
    kinds: inv.kinds.map((k) => ({ ...k, held: 0, groups: k.groups.map((g) => ({ ...g, cells: g.cells.map(zero) })) })),
    other: [],
    farm: [],
  };
}

// ---- Engineering: the sample fleet; there is no planner, only the sample's precomputed plans.
/** What the preview answers for a plan it has no numbers for (a stand-in for the planner). */
const NO_PLANNER = "Plan maths runs in the app. Outside it, only the sample plan has numbers.";
const NO_STATS = "Ship stats are worked out in the app. Outside it, only the sample Krait's sample plans have them.";
const NO_TRIP = "Engineer trips are planned in the app. Outside it, only the sample plan has one.";
const NO_SHOPPING = "The shopping list is worked out in the app (the preview only has the sample picks).";
/** The core's answer for a plan whose ship isn't in the fleet (`?core=empty` has none). */
const NOT_IN_FLEET = "ship not in the fleet";

/** The sample journal's fleet, with the synthetic combat Krait whose stats the sample has (stored). */
function sampleFleet(): Fleet {
  const f = engineeringSample.fleet;
  const s = shipsSample.fleet;
  const extra = s.ships.map((x) => ({ ...x, current: false }));
  return { ...f, ships: [...f.ships, ...extra], groups: { ...f.groups, ...s.groups } };
}

/** The sample's answer for a plan, else a failed call saying why there is none (`empty`: under
 * `?core=empty`). */
async function forPlan<T>(cmd: WindowCommand, sample: T | undefined, none: string, empty = NOT_IN_FLEET): Promise<T> {
  await previewGate(cmd);
  if (fakeScenario() === "empty") throw empty;
  if (sample === undefined) throw none;
  return sample;
}

// ---- Search filters: kept in the browser; the pretend ship has a medium pad.
const filtersKey = (lookup: string) => `astromech.search.${lookup}`;

const searchFilters: H<"search_filters"> = ({ lookup }) => {
  let saved: Partial<Filters> = {};
  try {
    saved = JSON.parse(localStorage.getItem(filtersKey(lookup)) ?? "{}") as Partial<Filters>;
  } catch {
    saved = {};
  }
  const f = { ...DEFAULT_FILTERS, ...saved };
  return f.pad_set ? f : { ...f, pad: "M" };
};

/** Public system names for the autocomplete (prefixes only; Spansh's is fuzzy). */
const SYSTEMS = ["Sol", "Shinrarta Dezhra", "Shingon", "Shinigami", "Deciat", "Colonia", "Achenar", "Alioth", "Maia"];

// ---- The neutron star file: none until Download, then a short fake download. The header is the
// 22 Oct 2026 file's (scripts/gen-neutron.py writes name groups of 256).
const NEUTRON_FILE: Header = { cell_ly: 128, stars: 5045154, cells: 1645100, built: 1792627200, name_group: 256 };
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

/** Plot from fixtures/plot.json: neutron routes "on this PC" once the file is downloaded (the
 * local route was plotted over the tests' small neutron star sample, so it isn't to Colonia). */
const routePlot: H<"route_plot"> = async ({ ask }) => {
  const neutronRoute = neutron.file ? plotSample.local : plotSample.neutron;
  const plotted = ask.plotter === "neutron" ? neutronRoute : plotSample.exact;
  const r = await coreReply("route_set", { name: plotted.name, source: plotted.source, waypoints: plotted.waypoints, plot: plotted.plot });
  if (!r.ok) throw new Error(r.error);
  const view: RouteView = r.view;
  return { plotted, view };
};

/** As the command layer's `route_replot`: plot again with the route's own settings. */
const routeReplot: H<"route_replot"> = async () => {
  const r = await coreReply("route", null);
  const plot = r.ok ? r.view.route?.plot : null;
  if (!plot) throw new Error("This route wasn't plotted here, so there is nothing to plot again. Plot it from the top bar.");
  const { to, plotter, engine, efficiency, refuel_every_scoopable, range } = plot;
  return routePlot({ ask: { to, plotter, engine, efficiency, refuel_every_scoopable, range } });
};

/** The command layer's commands. `publish` sends a push (a command's progress channel). */
export function shellCommands(publish: (topic: string, payload: unknown) => void) {
  return {
    trade_search: tradeSearch,
    trade_anywhere: tradeAnywhere,
    trade_cargo: tradeCargo,
    market_compare: () => previewData("market_compare", () => compareSample, () => null),
    trade_route_defaults: () =>
      previewData("trade_route_defaults", () => tradeSample.route.defaults, () => ({ capacity: null, credits: null, range: null, station: null, system: null })),
    trade_route: tradeRoute,
    colonisation_projects: () => previewData("colonisation_projects", () => projectsSample, () => []),
    colonisation_buy: colonisationBuy,
    stations_nearby: stationsNearby,
    mining_hotspots: miningHotspots,
    galaxy_nearby: galaxyNearby,
    galaxy_sites: galaxySites,
    colony_candidates: colonyCandidates,
    logbook: ({ query }) => previewData("logbook", () => searchLogbook(query), () => noHistory(query)),
    session_summary: () => previewData("session_summary", () => labelled(sessionSample, "Sample journal history"), () => null),
    docked_here: () => previewData("docked_here", () => labelled(stationSample.here, "Sample docking"), () => NOT_DOCKED),
    outfitting_search: ({ query }) => previewData("outfitting_search", () => searchOutfitting(query), () => ({ stations: 0, snapshots: 0, found: [] })),
    exploration_system: () =>
      previewData("exploration_system", () => labelled(explorationSample.system, "Sample system"), (): SystemView => ({
        system: null, system_address: null, population: 0, body_count: null, all_found: false, bodies: [], value: 0, value_if_mapped: 0,
      })),
    exploration_unsold: () => previewData("exploration_unsold", () => labelled(explorationSample.unsold, "Sample history"), () => null),
    materials_inventory: () => previewData("materials_inventory", () => materialsSample, nothingHeld),
    // Nothing unlocked yet: every engineer locked, none known.
    engineering_unlocks: () =>
      previewData("engineering_unlocks", () => unlocksSample, () => ({
        nodes: unlocksSample.nodes.map((n) => ({ ...n, status: "locked" as const, rank: null, next: false })),
      })),
    powerplay_overview: () => {
      const o = powerplaySample;
      const unpledged: Overview = {
        standing: { power: null, rank: null, merits: null, time_pledged_secs: null, session_merits: 0, week_merits: 0, week_ends: o.standing.week_ends, next: null },
        here: null,
        system: null,
      };
      return previewData("powerplay_overview", () => o, () => unpledged);
    },
    powerplay_targets: ({ target, origin, within_ly: withinLy }) => {
      const s = targetsSample[target];
      const shown = (hits: Targets["hits"]) => ({ ...s, hits, within_ly: withinLy, here: from(s.here, origin) });
      return previewData("powerplay_targets", () => shown(s.hits.filter((h) => h.ly <= withinLy)), () => shown([]));
    },
    odyssey_view: () =>
      previewData("odyssey_view", () => odysseySample.view, () => ({
        ...odysseySample.view, source: "No journal yet", locker_ts: null, backpack_ts: null, items: [], owned: [],
      })),
    // Only the sample picks have a shopping list.
    odyssey_shopping: ({ picks }) => forPlan("odyssey_shopping", samePicks(picks, odysseySample.picks) ? odysseySample.shopping : undefined, NO_SHOPPING, NO_SHOPPING),
    engineering_fleet: () => previewData("engineering_fleet", sampleFleet, () => ({ source: "No journal yet", ships: [], stored_modules: [], groups: {} })),
    engineering_plan: ({ plan }) => forPlan("engineering_plan", plan.ship_id === engineeringSample.plan.ship_id ? engineeringSample.plan : undefined, NO_PLANNER),
    engineering_stats: ({ plan }) => forPlan("engineering_stats", shipsSample.cases.find((c) => samePlan(c.plan, plan))?.stats, NO_STATS),
    engineering_headlines: () => previewData("engineering_headlines", () => shipsSample.headlines, () => ({})),
    engineering_engineers: () => previewData("engineering_engineers", () => engineeringSample.engineers, () => ({ engineers: [], here: null })),
    engineering_trip: ({ plan }) => forPlan("engineering_trip", plan.ship_id === engineeringSample.plan.ship_id ? engineeringSample.trip : undefined, NO_TRIP),
    search_filters: searchFilters,
    set_search_filters: ({ lookup, filters }) => {
      localStorage.setItem(filtersKey(lookup), JSON.stringify(filters));
      return null;
    },
    galaxy_systems: ({ prefix }) => SYSTEMS.filter((s) => s.toLowerCase().startsWith(prefix.toLowerCase())),
    route_plot_ship: () => plotSample.ship,
    route_plot: routePlot,
    route_replot: routeReplot,
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
    open_external: ({ target }) => {
      window.open(target, "_blank");
      return null;
    },
  } satisfies Partial<Handlers>;
}
