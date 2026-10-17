/**
 * The route the core follows (docs/PLOTTING.md, layer 1). The next waypoint is kept on the
 * clipboard, ready to paste into the galaxy map's search box; arriving in it copies the one
 * after. The core owns the route, so it is followed with the window shut. The browser preview
 * (`?core=ready`) keeps one in memory (fakecore.ts). Routes can be plotted with Spansh's
 * neutron and exact plotters for the current ship (the shell's `route_plot`).
 */
import { invoke } from "@tauri-apps/api/core";
import { coreCall, inTauri, type CoreCmd } from "./core";
import plotSample from "./fixtures/plot.json";

export interface Waypoint {
  system: string;
  /** What is there, e.g. the engineer's name. */
  label?: string;
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
  /** Why the last clipboard write failed, if it did. */
  clipboard_error: string | null;
}

async function call(cmd: CoreCmd, args?: Record<string, unknown>): Promise<RouteView> {
  const r = await coreCall(cmd, args ?? null);
  if (!r.ok || !r.view) throw new Error(r.error ?? "the core didn't answer");
  return r.view as RouteView;
}

export const getRoute = () => call("route");
export const skipWaypoint = () => call("route_skip");
export const backWaypoint = () => call("route_back");
export const clearRoute = () => call("route_clear");
export const copyNext = () => call("route_copy");

/** Follow these stops. The first is copied straight away. */
export function followRoute(name: string, source: string, waypoints: Waypoint[]): Promise<RouteView> {
  return call("route_set", { name, source, waypoints });
}

/** Follow pasted text: a Spansh CSV export, or one system name per line (parsed by the core). */
export function importRoute(name: string, text: string): Promise<RouteView> {
  return call("route_set", { name, source: "import", text });
}

/** "Deciat is on your clipboard." after following a route, or why it isn't. */
export function followedNote(v: RouteView): string {
  if (!v.next) return "Following.";
  return v.clipboard_error ? `Following. ${v.next.system} is not on the clipboard: ${v.clipboard_error}` : `Following. ${v.next.system} is on your clipboard (Galaxy → Route).`;
}

/** Spansh's plotters (astromech_galaxy::plot). */
export type Plotter = "neutron" | "exact";

export interface PlotAsk {
  plotter: Plotter;
  /** Blank: the commander's system. */
  from: string;
  to: string;
  /** Neutron routes, 1–100: how far Spansh may stray to use neutron stars. */
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
}

/** Where the commander is and the ship a route would be plotted for. The browser preview
 * answers from fixtures/plot.json (the routes astromech_galaxy::plot makes from the recorded
 * Spansh jobs). */
export async function plotSetup(): Promise<PlotSetup> {
  if (inTauri) return await invoke<PlotSetup>("route_plot_ship");
  return plotSample.ship as PlotSetup;
}

/** Plot with Spansh (can take a minute) and follow the route straight away. */
export async function plotRoute(ask: PlotAsk): Promise<{ plotted: Plotted; view: RouteView }> {
  if (inTauri) return await invoke<{ plotted: Plotted; view: RouteView }>("route_plot", { ask });
  const plotted = (ask.plotter === "neutron" ? plotSample.neutron : plotSample.exact) as Plotted;
  const view = await followRoute(plotted.name, plotted.source, plotted.waypoints);
  return { plotted, view };
}
