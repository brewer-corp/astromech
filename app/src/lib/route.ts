/**
 * The route the core follows (docs/PLOTTING.md, layer 1). The next waypoint is kept on the
 * clipboard, ready to paste into the galaxy map's search box; arriving in it copies the one
 * after. The core owns the route, so it is followed with the window shut. The browser preview
 * (`?core=ready`) keeps one in memory (fakecore.ts).
 */
import { coreCall, type CoreCmd } from "./core";

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
