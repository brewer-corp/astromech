// The route followed, as the frame shows it (DESIGN.md Route sidebar, Route strip): one 2 s poll
// of the core's route, read by the strip and the sidebar alike, and where the sidebar is (docked,
// collapsed to the strip, or open over the page below the dock width). The frame (App.svelte)
// makes one and hands it down by context; a strip or sidebar drawn on its own (a test) makes its
// own. Pages that change the route ping watchRoute, which reads it again at once.
import { getContext, setContext } from "svelte";
import { getRoute, watchRoute, type RouteView } from "../../lib/route";
import { pollFailure } from "../../lib/ui/errors";

const COLLAPSED = "astromech.routeCollapsed";

function readCollapsed(): boolean {
  try {
    return localStorage.getItem(COLLAPSED) === "1";
  } catch {
    return false;
  }
}

export class RouteFollow {
  view = $state<RouteView | null>(null);
  /** The first read failed: nothing to show yet. */
  readErr = $state<string | null>(null);
  /** The last poll failed while a route is shown: the route stays, with a line saying so. */
  stale = $state<string | null>(null);
  /** The commander folded the docked sidebar to the strip; kept across runs. */
  #collapsed = $state(readCollapsed());
  /** Below the dock width: the sidebar is open over the page. */
  overlay = $state(false);
  /** The window is at or above the dock width (the frame measures it). */
  docked = $state(false);
  /** Asked for (#/route, the palette's Route) at the dock width with no route: the sidebar shows,
   * empty but for its plotter, instead of the no-route rail, until it is folded or a route comes. */
  asked = $state(false);

  get collapsed(): boolean {
    return this.#collapsed;
  }
  set collapsed(v: boolean) {
    this.#collapsed = v;
    try {
      localStorage.setItem(COLLAPSED, v ? "1" : "0");
    } catch {
      // Not kept past this run; the choice still holds now.
    }
  }

  /** Read the route again now. */
  read = (): Promise<boolean> => this.run(getRoute, true);

  /** Run a route command and show the view it answers with. A polled read's failure is kept
   * here (readErr, stale); an action's is thrown, for the surface that ran it to show. */
  async run(f: () => Promise<RouteView>, polled = false): Promise<boolean> {
    try {
      this.view = await f();
      this.readErr = null;
      this.stale = null;
      return true;
    } catch (e) {
      if (!polled) throw e;
      if (this.view) this.stale = pollFailure("read the route", e, true);
      else this.readErr = pollFailure("read the route", e, false);
      return false;
    }
  }

  /** Poll every 2 s and on every route change until the returned stop is called. */
  start(): () => void {
    void this.read();
    const t = setInterval(() => void this.read(), 2000);
    const stop = watchRoute(() => void this.read());
    return () => {
      clearInterval(t);
      stop();
    };
  }

  /** Show the route where the commander is: the docked sidebar unfolded, or below the dock
   * width the sidebar over the page. The page stays. */
  show(): void {
    if (this.docked) {
      this.collapsed = false;
      this.asked = true;
    } else this.overlay = true;
  }

  /** Fold the docked sidebar: to the strip, kept across runs, or with no route back to the rail. */
  fold(): void {
    if (this.view?.route || this.view?.parked) this.collapsed = true;
    this.asked = false;
  }
}

const KEY = Symbol("route-follow");

/** The frame's side: one poll for the whole window. */
export function provideFollow(): RouteFollow {
  const f = new RouteFollow();
  setContext(KEY, f);
  return f;
}

/** A strip's or sidebar's side: the frame's, or with no frame, one of its own (started by the
 * caller). */
export function useFollow(): { follow: RouteFollow; own: boolean } {
  const f = getContext<RouteFollow | undefined>(KEY);
  return f ? { follow: f, own: false } : { follow: new RouteFollow(), own: true };
}
