/**
 * The route the core follows (docs/PLOTTING.md, layer 1). The next waypoint is kept on the
 * clipboard, ready to paste into the galaxy map's search box; arriving in it copies the one
 * after. The core owns the route, so it is followed with the window shut. The browser preview
 * (`?core=ready`) keeps one in memory (app/src/preview/fakecore.ts). Routes can be plotted from the top bar for
 * the current ship, from its system (the core's `route_plot`): neutron routes on this PC from
 * the neutron star file when it is downloaded, else by Spansh, and exact routes by Spansh. A
 * route plotted in the galaxy map is followed too (core: route.rs).
 */
import type { FollowedRoute } from "./api/types/FollowedRoute";
import type { Plotted } from "./api/types/Plotted";
import type { PlotAsk } from "./api/types/PlotAsk";
import type { PlotShip } from "./api/types/PlotShip";
import type { RouteView } from "./api/types/RouteView";
import type { RouteStop } from "./api/types/RouteStop";
import type { Scoop } from "./api/types/Scoop";
import type { StarGlyph } from "./ui/Glyph.svelte";
import type { Waypoint } from "./api/types/Waypoint";
import { coreCall } from "./core";
import { call, type ArgList } from "./transport";

/** A waypoint: the system, what is there (an engineer's name, a plotter's "neutron boost" and
 * "fuel") and the jumps from the one before (plotted routes and the galaxy map's know them;
 * imports and a lookup's stops don't). */
export type { Waypoint } from "./api/types/Waypoint";
/** The route followed: where it came from ("engineer trip", a lookup's id, "import") and the
 * index of the next waypoint (waypoints.length once finished). */
export type Route = FollowedRoute;
export type { RouteView } from "./api/types/RouteView";
export type { RouteStop } from "./api/types/RouteStop";
export type { RouteJump } from "./api/types/RouteJump";
export type { Plotted } from "./api/types/Plotted";
export type { PlotAsk } from "./api/types/PlotAsk";
export type { Plotter } from "./api/types/Plotter";
/** The current ship as the plotters see it. */
export type { PlotterShip as PlotShip } from "./api/types/PlotterShip";
/** Where the commander is and the ship a route would be plotted for. */
export type PlotSetup = PlotShip;

/** What is at a stop, one badge each: "neutron boost and fuel" (astromech_galaxy::plot) is two,
 * and stops merged in one system ("Felicity Farseer, Professor Palin") are one each. */
export function stopFlags(label: string | null | undefined): string[] {
  if (!label) return [];
  return label.split(", ").flatMap((l) => (l === "neutron boost and fuel" ? ["neutron boost", "fuel"] : [l]));
}

/** The source of a route plotted in the galaxy map: followed quietly, never copied. */
export const GAME_SOURCE = "galaxy map";

const watchers = new Set<() => void>();
/** The route strip and the Route page refresh as soon as a route mutates, not on the next poll. */
export function watchRoute(fn: () => void): () => void {
  watchers.add(fn);
  return () => watchers.delete(fn);
}

/** The route commands; each replies with the route's view. */
type RouteCmd = "route" | "route_details" | "route_set" | "route_add" | "route_skip" | "route_back" | "route_go" | "route_clear" | "route_copy" | "route_resume";

async function routeCall<K extends RouteCmd>(mutate: boolean, cmd: K, ...args: ArgList<K>): Promise<RouteView> {
  const r = await coreCall(cmd, ...args);
  if (!r.ok || !r.view) throw new Error((!r.ok && r.error) || "the core didn't answer");
  if (mutate) for (const f of watchers) f();
  return r.view;
}

export const getRoute = () => routeCall(false, "route");
/** The route with its inhabited stops' details, looked up on Spansh once per stop this run. */
export const getRouteDetails = () => routeCall(false, "route_details");
export const skipWaypoint = () => routeCall(true, "route_skip");
export const backWaypoint = () => routeCall(true, "route_back");
export const clearRoute = () => routeCall(true, "route_clear");
export const copyNext = () => routeCall(true, "route_copy");
export const resumeRoute = () => routeCall(true, "route_resume");
/** Make stop `index` (named `system`, as the page showed it) the next one, and copy it. */
export const makeNext = (index: number, system: string) => routeCall(true, "route_go", { index, system });

/** Follow these stops. The first is copied straight away. */
export function followRoute(name: string, source: string, waypoints: Waypoint[]): Promise<RouteView> {
  return routeCall(true, "route_set", { name, source, waypoints });
}

/** Follow pasted text: a Spansh CSV export, or one system name per line (parsed by the core). */
export function importRoute(name: string, text: string): Promise<RouteView> {
  return routeCall(true, "route_set", { name, source: "import", text });
}

/** Make this stop the next one on the route followed (merged with a stop in the same system),
 * or, with no route of our own to add to, follow it on its own. It is copied straight away. */
export function addStop(stop: Waypoint, source: string): Promise<RouteView> {
  return routeCall(true, "route_add", { ...stop, source });
}

/** What following or adding a stop did, for the line under the button. */
export interface FollowNote {
  text: string;
  /** The parked route's name: it was replaced and RESUME brings it back. */
  replaced: string | null;
  /** The action failed: `text` says why (a hard error), and this tries it again. */
  retry?: () => void;
}

const clip = (v: RouteView, w: Waypoint) => (v.clipboard_error ? `${w.system} is not on the clipboard: ${v.clipboard_error}` : `${w.system} is on your clipboard.`);

/** "Following. Deciat is on your clipboard." after following a route, or why it isn't, and the
 * route it replaced ("Replaced Engineer trip." and RESUME): following never drops a route
 * silently. */
export function followedNote(v: RouteView): FollowNote {
  return { text: v.next ? `Following. ${clip(v, v.next)}` : "Following.", replaced: v.parked?.name ?? null };
}

/** "Deciat is the next stop. Deciat is on your clipboard." after Add stop. Adding replaces
 * nothing, so a route parked before stays parked but isn't news here. */
export function addedNote(v: RouteView): FollowNote {
  return { text: v.next ? `${v.next.system} is the next stop. ${clip(v, v.next)}` : "Added.", replaced: null };
}

/** Only the replaced route, for a panel that says what it followed itself (the plotter). */
export function replacedNote(v: RouteView): FollowNote {
  return { text: "", replaced: v.parked?.name ?? null };
}

/** The action failed: "Couldn't follow it: …", with `retry` to try it again. */
export function failedNote(what: string, e: unknown, retry: () => void): FollowNote {
  return { text: `${what}: ${e instanceof Error ? e.message : String(e)}`, replaced: null, retry };
}

/** Where the commander is and the ship a route would be plotted for. The browser preview
 * answers from app/src/preview/fixtures/plot.json (the routes astromech_galaxy::plot makes from
 * the recorded Spansh jobs, and a short local neutron route over the tests' neutron star sample:
 * astromech-galaxy's dump-local-route example). */
export async function plotSetup(): Promise<PlotSetup> {
  return await call("route_plot_ship");
}

/** Plot (on this PC in seconds, or by Spansh in up to a minute) and follow the route straight
 * away. */
export async function plotRoute(ask: PlotAsk): Promise<{ plotted: Plotted; view: RouteView }> {
  const r = await call("route_plot", { ask });
  for (const f of watchers) f();
  // `view` is route_set's reply, which the command layer can't name (astromech-se44's JsonValue
  // allow-list).
  return { plotted: r.plotted, view: r.view as RouteView };
}

/** A star on the timeline as the route's sources know it: its class from `NavRoute.json` where
 * the game listed it, else the plotter's scoopable and neutron flags, else nothing. */
export interface StarFacts {
  star_class: string | null;
  scoopable: Scoop;
  neutron: boolean | null;
}

/** What a star is, for its icon, the word beside it and its Tip. */
export interface StarKind {
  glyph: StarGlyph;
  /** Its accessible name and Tip fact: "Class K star", "Neutron star", "Unknown star". */
  name: string;
  /** What it means for the commander. */
  note: string;
}

const GIANTS: Record<string, string> = {
  A_BlueWhiteSuperGiant: "Blue-white supergiant",
  F_WhiteSuperGiant: "White supergiant",
  M_RedSuperGiant: "Red supergiant",
  M_RedGiant: "Red giant",
  K_OrangeGiant: "Orange giant",
};
const SCOOP = "Scoopable: refuel here.";
const DRY = "Not scoopable.";

/** The star's type icon (DESIGN.md Icons: Stars), name and note. A class names its type; a star
 * with no class is told by its plotter's flags, and one with neither is the unknown star. */
export function starKind(s: StarFacts): StarKind {
  const c = s.star_class;
  if (c === "N" || (c == null && s.neutron === true))
    return { glyph: "star-neutron", name: "Neutron star", note: `${DRY} Fly into a beam and your next jump has four times the range.` };
  if (c == null) {
    if (s.scoopable === "yes") return { glyph: "star-scoopable", name: "Scoopable star", note: SCOOP };
    if (s.scoopable === "no") return { glyph: "star-other", name: "Non-scoopable star", note: DRY };
    return { glyph: "star-unknown", name: "Unknown star", note: "No source says what this star is, so it may not be scoopable." };
  }
  if (c.startsWith("D")) return { glyph: "star-white-dwarf", name: "White dwarf", note: `${DRY} Its beams boost a jump 1.5 times but can damage your FSD.` };
  if (c === "L" || c === "T" || c === "Y") return { glyph: "star-brown-dwarf", name: `Class ${c} brown dwarf`, note: DRY };
  if (c === "H" || c === "SupermassiveBlackHole") return { glyph: "star-black-hole", name: "Black hole", note: DRY };
  const name = GIANTS[c] ?? (c === "TTS" ? "T Tauri star" : c.startsWith("W") ? "Wolf-Rayet star" : `Class ${c} star`);
  return s.scoopable === "yes" ? { glyph: "star-scoopable", name, note: SCOOP } : { glyph: "star-other", name, note: DRY };
}

/** Whether a leg is known to have nowhere to scoop: its jumps (the last landing on its stop) are
 * all listed, there are two or more, and every star is known not to be scoopable. A leg with a
 * missing jump or an unknown star never claims it. */
export function noScoop(leg: Pick<RouteStop, "hops" | "jumps">): boolean {
  const hops = leg.hops;
  if (!hops || hops.length < 2 || (leg.jumps != null && leg.jumps !== hops.length)) return false;
  return hops.every((h) => h.scoopable === "no");
}
