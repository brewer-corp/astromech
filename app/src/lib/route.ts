/**
 * The route the core follows (docs/PLOTTING.md, layer 1). The next waypoint is kept on the
 * clipboard, ready to paste into the galaxy map's search box; arriving in it copies the one
 * after. Outside Tauri (vite dev, mockups) a route is kept in memory so the page can be tried.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";

export interface Waypoint {
  system: string;
  /** What is there, e.g. the engineer's name. */
  label?: string;
}

export interface Route {
  name: string;
  /** "engineer trip", "nearby", "import" */
  source: string;
  waypoints: Waypoint[];
  /** Index of the next waypoint; waypoints.length once finished. */
  next: number;
}

export interface RouteView {
  route: Route | null;
  here: string | null;
  next: Waypoint | null;
  remaining: number;
  clipboard_error: string | null;
}

type Cmd = "route" | "route_set" | "route_skip" | "route_back" | "route_clear" | "route_copy";

// --- dev stand-in -------------------------------------------------------------------------------
let dev: Route | null = null;
function devView(): RouteView {
  return {
    route: dev ? structuredClone(dev) : null,
    here: "Sol",
    next: dev?.waypoints[dev.next] ?? null,
    remaining: dev ? dev.waypoints.length - dev.next : 0,
    clipboard_error: null,
  };
}
function devCall(cmd: Cmd, args?: Record<string, unknown>): RouteView {
  if (cmd === "route_set") {
    const text = args?.text as string | undefined;
    const waypoints = text
      ? text.split("\n").map((s) => ({ system: s.trim().replace(/^"|"$/g, "") })).filter((w) => w.system)
      : (args?.waypoints as Waypoint[]);
    if (!waypoints.length) throw new Error("the route has no systems");
    dev = { name: String(args?.name || "Route"), source: String(args?.source ?? ""), waypoints, next: 0 };
  } else if (dev && cmd === "route_skip") dev.next = Math.min(dev.next + 1, dev.waypoints.length);
  else if (dev && cmd === "route_back") dev.next = Math.max(dev.next - 1, 0);
  else if (cmd === "route_clear") dev = null;
  return devView();
}
// ------------------------------------------------------------------------------------------------

async function call(cmd: Cmd, args?: Record<string, unknown>): Promise<RouteView> {
  if (!inTauri) return devCall(cmd, args);
  const r = await invoke<{ ok: boolean; view?: RouteView; error?: string }>("route_call", { cmd, args: args ?? null });
  if (!r.ok || !r.view) throw new Error(r.error ?? "the core didn't answer");
  return r.view;
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
