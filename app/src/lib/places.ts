/**
 * Places (decision astromech-bi94, DESIGN.md Places home): a family of finders, each with its
 * own title, main input and URL, listed on the Places home as tiles under section headings.
 * `FINDERS` is the one list of them: the home page, the Ctrl+K palette and, later, the header's
 * Find or plot box and voice read it. A finder's page is `#/places/<id>[/<input>]`; the input is
 * the finder's main value (a commodity, service, module, ship, trader or site kind, faction state
 * or mineral), URL-encoded.
 */
import { FACTION_STATES } from "./galaxy";
import { ORES, RING_TYPES, type Reserve, type RingType, type Want as HotspotWant } from "./hotspots";
import { SITE_KINDS } from "./sites";
import { SERVICES, type ServiceId } from "./stations";
import { commodities, type Side } from "./trade";

/** The home page's section headings, in order. A section with no finder built yet is left out. */
export const SECTIONS = ["Trade", "Ship", "Engineering", "Mining", "Route"] as const;
export type Section = (typeof SECTIONS)[number];

export type FinderId =
  | "sell"
  | "buy"
  | "sell-cargo"
  | "services"
  | "outfitting"
  | "shipyard"
  | "material-traders"
  | "raw-sites"
  | "faction-states"
  | "mining-hotspots";

export interface Finder {
  /** Stable id and URL part: `#/places/<id>`. */
  id: FinderId;
  /** The page title and tile name. */
  title: string;
  section: Section;
  /** The tile's one Label line: what it finds. */
  what: string;
  /** The main input's name ("Service"), shown beside it on the finder's page. Sell my cargo asks
   * nothing: its "input" is the hold it reads, listed above its results. */
  input: string;
}

export const FINDERS: readonly Finder[] = [
  { id: "sell", title: "Sell", section: "Trade", what: "Where a commodity fetches the most", input: "Commodity" },
  { id: "buy", title: "Buy", section: "Trade", what: "Where a commodity is cheapest", input: "Commodity" },
  { id: "sell-cargo", title: "Sell my cargo", section: "Trade", what: "The best stations for the whole hold", input: "Hold" },
  { id: "services", title: "Services", section: "Ship", what: "Interstellar Factors, traders, brokers, repair", input: "Service" },
  { id: "outfitting", title: "Outfitting", section: "Ship", what: "Stations selling a module", input: "Module" },
  { id: "shipyard", title: "Shipyard", section: "Ship", what: "Stations selling a ship", input: "Ship" },
  { id: "material-traders", title: "Material traders", section: "Engineering", what: "Raw, manufactured and encoded", input: "Trader" },
  { id: "raw-sites", title: "Raw sites", section: "Engineering", what: "Crystalline shards, brain trees, geological sites", input: "Site" },
  { id: "faction-states", title: "Faction states", section: "Engineering", what: "Boom, war, outbreak, for High Grade Emissions", input: "State" },
  { id: "mining-hotspots", title: "Mining hotspots", section: "Mining", what: "Rings with hotspots for a mineral", input: "Mineral" },
];

/** The home page's sections that have finders, each with its finders in order. */
export function homeSections(finders: readonly Finder[] = FINDERS): [Section, Finder[]][] {
  return SECTIONS.map((s) => [s, finders.filter((f) => f.section === s)] as [Section, Finder[]]).filter(([, fs]) => fs.length > 0);
}

/** The route (without "#/") of the Places home, a finder, or a finder with its input. */
export function placesRoute(finder?: FinderId, input?: string, query?: URLSearchParams): string {
  if (!finder) return "places";
  const qs = query?.toString();
  return `places/${finder}${input ? `/${encodeURIComponent(input)}` : ""}${qs ? `?${qs}` : ""}`;
}

/** Where a `#/places/...` hash points: the finder (null: the home page), its raw input and query. */
export interface PlacesAt {
  finder: Finder | null;
  /** The input part, still URL-encoded, with any older sub-parts ("Pulse%20Laser/2F"). */
  input: string;
  query: URLSearchParams;
}

/** Parse a hash ("#/places/outfitting/Pulse%20Laser?class=2"); null when it isn't a Places page. */
export function parsePlaces(hash: string): PlacesAt | null {
  const m = /^#\/places(?:\/([^/?]*)(?:\/([^?]*))?)?(?:\?(.*))?$/.exec(hash);
  if (!m) return null;
  const finder = FINDERS.find((f) => f.id === m[1]) ?? null;
  return { finder, input: finder ? (m[2] ?? "") : "", query: new URLSearchParams(finder ? (m[3] ?? "") : "") };
}

/**
 * Whether the address shows finder `id`, for a finder page's own hash listener and address
 * rewrite. The page passes its own id, never its `finder` prop: on a hash change that leaves the
 * page, Places has already set that prop to null when the page's listener runs.
 */
export function showing(id: FinderId): boolean {
  return parsePlaces(location.hash)?.finder?.id === id;
}

/** Decode a URL part, or null when it is mangled. */
export function decode(part: string): string | null {
  try {
    return decodeURIComponent(part);
  } catch {
    return null;
  }
}

/**
 * Pages that became finders keep working as links: `services`, `outfitting` and `shipyard`
 * routes, with whatever followed, open the matching finder.
 */
export const MOVED_FINDERS: Record<string, string> = {
  services: placesRoute("services"),
  outfitting: placesRoute("outfitting"),
  shipyard: placesRoute("shipyard"),
};

// Services: #/places/services/<service id>

export const serviceRoute = (id: ServiceId) => placesRoute("services", id);

/** The service a `#/places/services/<id>` hash asks for, or null for none or an unknown id. */
export function parseServiceRoute(hash: string): ServiceId | null {
  const at = parsePlaces(hash);
  if (at?.finder?.id !== "services") return null;
  const id = decode(at.input);
  return SERVICES.find((s) => s.id === id)?.id ?? null;
}

// Material traders: #/places/material-traders/<kind>, each kind one trader service.

export const TRADERS = [
  { kind: "raw", label: "Raw", name: "Raw trader", service: "raw_trader" },
  { kind: "manufactured", label: "Manufactured", name: "Manufactured trader", service: "manufactured_trader" },
  { kind: "encoded", label: "Encoded", name: "Encoded trader", service: "encoded_trader" },
  { kind: "any", label: "Any", name: "Any material trader", service: "material_trader" },
] as const satisfies readonly { kind: string; label: string; name: string; service: ServiceId }[];

export type TraderKind = (typeof TRADERS)[number]["kind"];

export const traderRoute = (kind: TraderKind) => placesRoute("material-traders", kind);

/** The trader kind a `#/places/material-traders/<kind>` hash asks for, or null. */
export function parseTraderRoute(hash: string): TraderKind | null {
  const at = parsePlaces(hash);
  if (at?.finder?.id !== "material-traders") return null;
  return TRADERS.find((t) => t.kind === at.input)?.kind ?? null;
}

// Sell and Buy: #/places/<sell|buy>/<commodity symbol>[?range=anywhere], near the origin unless
// the range is anywhere in the galaxy.

export interface MarketWant {
  side: Side;
  /** The commodity's journal symbol: "painite". */
  symbol: string;
  anywhere: boolean;
}

export function marketRoute(w: MarketWant): string {
  return placesRoute(w.side, w.symbol, w.anywhere ? new URLSearchParams({ range: "anywhere" }) : undefined);
}

/** What a `#/places/<sell|buy>/<symbol>` hash asks for; null for another page or an unknown commodity. */
export function parseMarketRoute(hash: string): MarketWant | null {
  const at = parsePlaces(hash);
  const side = at?.finder?.id;
  if (!at || (side !== "sell" && side !== "buy")) return null;
  const symbol = commodities.find((c) => c.symbol === decode(at.input))?.symbol;
  return symbol ? { side, symbol, anywhere: at.query.get("range") === "anywhere" } : null;
}

// Raw sites: #/places/raw-sites/<site kind id>.

export const siteRoute = (kind: string) => placesRoute("raw-sites", kind);

/** The site kind a `#/places/raw-sites/<kind>` hash asks for, or null. */
export function parseSiteRoute(hash: string): string | null {
  const at = parsePlaces(hash);
  if (at?.finder?.id !== "raw-sites") return null;
  return SITE_KINDS.find((k) => k.id === at.input)?.id ?? null;
}

// Faction states: #/places/faction-states/<query id>, the same id the voice query and the
// remembered filters (`faction.<id>`) use.

export const factionRoute = (query: string) => placesRoute("faction-states", query);

/** The faction-state lookup a `#/places/faction-states/<id>` hash asks for, or null. */
export function parseFactionRoute(hash: string): string | null {
  const at = parsePlaces(hash);
  if (at?.finder?.id !== "faction-states") return null;
  return FACTION_STATES.find((q) => q.id === at.input)?.id ?? null;
}

// Mining hotspots: #/places/mining-hotspots/<ore symbol>[?ring=<type>&reserve=<pristine|major>],
// the ring type and least reserve being the finder's own choices (absent: any).

const RING_PARAM: Record<RingType, string> = { Metallic: "metallic", "Metal Rich": "metal-rich", Rocky: "rocky", Icy: "icy" };

export function hotspotRoute(want: HotspotWant): string {
  const q = new URLSearchParams();
  if (want.ring_type) q.set("ring", RING_PARAM[want.ring_type]);
  if (want.reserve) q.set("reserve", want.reserve);
  return placesRoute("mining-hotspots", want.ore, q);
}

/** What a `#/places/mining-hotspots/<ore>` hash asks for; null for another page or an unknown
 * ore. An unknown ring type or reserve reads as any. */
export function parseHotspotRoute(hash: string): HotspotWant | null {
  const at = parsePlaces(hash);
  if (at?.finder?.id !== "mining-hotspots") return null;
  const ore = ORES.find((o) => o.symbol === decode(at.input))?.symbol;
  if (!ore) return null;
  const ring = RING_TYPES.find((t) => RING_PARAM[t] === at.query.get("ring")) ?? null;
  const r = at.query.get("reserve");
  const reserve: Reserve | null = r === "pristine" || r === "major" ? r : null;
  return { ore, ring_type: ring, reserve };
}

// The last answer of each finder this session, for its tile (decision astromech-g8d5, item 6):
// opening the home page never asks for one.

export interface Answer {
  /** What was found, as the tile's context line: "Interstellar Factors". */
  context: string;
  /** The ranked value of the first result: "2", "~3", "313,239". */
  figure: string;
  /** Its unit: "jumps", "cr". */
  unit: string;
  /** The finder page that gave it, so the tile opens it again. */
  route: string;
}

const answers = new Map<FinderId, Answer>();

/** Keep a finder's latest answer; null (nothing found) forgets the one before. */
export function rememberAnswer(id: FinderId, a: Answer | null): void {
  if (a) answers.set(id, a);
  else answers.delete(id);
}

export function lastAnswer(id: FinderId): Answer | undefined {
  return answers.get(id);
}

/** For tests: forget every answer. */
export function forgetAnswers(): void {
  answers.clear();
}
