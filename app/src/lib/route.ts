/**
 * The route the core follows (docs/PLOTTING.md, layer 1). The next waypoint is kept on the
 * clipboard, ready to paste into the galaxy map's search box; arriving in it copies the one
 * after. The core owns the route, so it is followed with the window shut. The browser preview
 * (`?core=ready`) keeps one in memory (fakecore.ts). Routes can be plotted from the top bar for
 * the current ship, from its system (the shell's `route_plot`): neutron routes on this PC from
 * the neutron star file when it is downloaded, else by Spansh, and exact routes by Spansh. A
 * route plotted in the galaxy map is followed too (core: route.rs).
 */
import { invoke } from "@tauri-apps/api/core";
import { coreCall, inTauri, type CoreCmd } from "./core";
import { previewHasNeutronFile } from "./neutron";
import plotSample from "./fixtures/plot.json";

export interface Waypoint {
  system: string;
  /** What is there, e.g. the engineer's name, or a plotter's "neutron boost" and "fuel". */
  label?: string;
  /** Jumps from the waypoint before: plotted routes and the galaxy map's know them; imports
   * and a lookup's stops don't. */
  jumps?: number;
}

export interface Route {
  name: string;
  /** Where it came from: "engineer trip", a lookup's id, "import". */
  source: string;
  waypoints: Waypoint[];
  /** Index of the next waypoint; waypoints.length once finished. */
  next: number;
}

export interface RouteView {
  route: Route | null;
  /** The last system the journal put the commander in. */
  here: string | null;
  next: Waypoint | null;
  /** Waypoints still to reach, the next one included. */
  remaining: number;
  /** Jumps to the end, when every waypoint still to reach knows its jumps. */
  jumps_left: number | null;
  /** Why the last clipboard write failed, if it did. */
  clipboard_error: string | null;
  /** The route a galaxy-map plot or a newly followed route replaced, to resume. */
  parked: Route | null;
}

/** What is at a stop, one badge each: "neutron boost and fuel" (astromech_galaxy::plot) is two,
 * and stops merged in one system ("Felicity Farseer, Professor Palin") are one each. */
export function stopFlags(label: string | undefined): string[] {
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

async function call(cmd: CoreCmd, args?: Record<string, unknown>, mutate = false): Promise<RouteView> {
  const r = await coreCall(cmd, args ?? null);
  if (!r.ok || !r.view) throw new Error(r.error ?? "the core didn't answer");
  if (mutate) for (const f of watchers) f();
  return r.view as RouteView;
}

export const getRoute = () => call("route");
export const skipWaypoint = () => call("route_skip", undefined, true);
export const backWaypoint = () => call("route_back", undefined, true);
export const clearRoute = () => call("route_clear", undefined, true);
export const copyNext = () => call("route_copy", undefined, true);
export const resumeRoute = () => call("route_resume", undefined, true);

/** Follow these stops. The first is copied straight away. */
export function followRoute(name: string, source: string, waypoints: Waypoint[]): Promise<RouteView> {
  return call("route_set", { name, source, waypoints }, true);
}

/** Follow pasted text: a Spansh CSV export, or one system name per line (parsed by the core). */
export function importRoute(name: string, text: string): Promise<RouteView> {
  return call("route_set", { name, source: "import", text }, true);
}

/** Make this stop the next one on the route followed (merged with a stop in the same system),
 * or, with no route of our own to add to, follow it on its own. It is copied straight away. */
export function addStop(stop: Waypoint, source: string): Promise<RouteView> {
  return call("route_add", { ...stop, source }, true);
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

/** The plotters (astromech_galaxy::plot). */
export type Plotter = "neutron" | "exact";

/** A route from the commander's system. */
export interface PlotAsk {
  plotter: Plotter;
  to: string;
  /** Neutron routes plotted by Spansh, 1–100: how far it may stray to use neutron stars. */
  efficiency: number;
  /** Exact routes: fill up at every scoopable star, not only when the tank needs it. */
  refuel_every_scoopable: boolean;
}

/** The current ship as the plotters see it. */
export interface PlotShip {
  name: string;
  /** One jump with a full tank and the cargo aboard now, ly. */
  range: number;
  cargo: number;
  /** Modules the calculations left out: the range may be off. */
  unresolved: string[];
}

export interface PlotSetup {
  here: string | null;
  ship: PlotShip | null;
  /** Why there is no ship to plot for. */
  ship_error: string | null;
}

export interface Plotted {
  name: string;
  source: string;
  waypoints: Waypoint[];
  /** Jumps in all (a neutron route has more jumps than stops). */
  jumps: number;
  ly: number;
  neutrons: number;
  refuels: number;
  range: number;
  /** Worked out on this PC (the neutron star file), not by Spansh. */
  local: boolean;
  /** Local routes: false when it is near the fewest jumps but not proven the fewest. */
  fewest?: boolean;
  /** Why Spansh plotted a neutron route although the neutron star file is there. */
  fallback?: string;
}

/** Where the commander is and the ship a route would be plotted for. The browser preview
 * answers from fixtures/plot.json (the routes astromech_galaxy::plot makes from the recorded
 * Spansh jobs, and a local neutron route from the 22 Oct 2026 neutron star file). */
export async function plotSetup(): Promise<PlotSetup> {
  if (inTauri) return await invoke<PlotSetup>("route_plot_ship");
  return plotSample.ship as PlotSetup;
}

/** Plot (on this PC in seconds, or by Spansh in up to a minute) and follow the route straight
 * away. */
export async function plotRoute(ask: PlotAsk): Promise<{ plotted: Plotted; view: RouteView }> {
  if (inTauri) {
    const r = await invoke<{ plotted: Plotted; view: RouteView }>("route_plot", { ask });
    for (const f of watchers) f();
    return r;
  }
  const neutron = previewHasNeutronFile() ? plotSample.local : plotSample.neutron;
  const plotted = (ask.plotter === "neutron" ? neutron : plotSample.exact) as Plotted;
  const view = await followRoute(plotted.name, plotted.source, plotted.waypoints);
  return { plotted, view };
}
