<script lang="ts">
  // Route (docs/PLOTTING.md, layer 1): the route the core follows, as a timeline (Timeline.svelte).
  // Its next waypoint is on the clipboard for the galaxy map's search box; jumping into it copies
  // the one after. The page's title is the plotter (Title.svelte): plot, replot, the plot setup,
  // and Import, Open on Spansh and Clear route in its ⋯ menu. Routes also come from the header's
  // Find or plot (PlotBar), the engineer trip, a search result's Add stop or Set destination, or
  // the galaxy map itself: a route plotted there is followed quietly. A route replaced by any of
  // them is parked to resume. The route strip (Strip.svelte) shows the next stop on every page and
  // links here. Skip and Back are voice commands only. A failed action or first read is
  // DESIGN.md's hard error: a red "!", the words, and TRY AGAIN.
  import { getRoute, getRouteDetails, resumeRoute, watchRoute, type RouteView } from "../../lib/route";
  import { pollFailure } from "../../lib/ui/errors";
  import Board from "../../lib/ui/layout/Board.svelte";
  import Button from "../../lib/ui/Button.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
  import Timeline from "./Timeline.svelte";
  import Title from "./Title.svelte";

  let view = $state<RouteView | null>(null);
  /** The last action (Copy, Make next, Clear, import) failed; it stays until the next action. */
  let err = $state<string | null>(null);
  /** That action again, for TRY AGAIN. */
  let again = $state<(() => void) | null>(null);
  /** The first read failed: nothing to show yet. */
  let readErr = $state<string | null>(null);
  /** The last poll failed while a route is shown: the route stays, with a line saying so. */
  let stale = $state<string | null>(null);

  /** Run a route command and show its view; false when it failed. */
  async function run(f: () => Promise<RouteView>, polled = false): Promise<boolean> {
    try {
      view = await f();
      if (!polled) err = null;
      readErr = null;
      stale = null;
      return true;
    } catch (e) {
      if (!polled) {
        err = String(e instanceof Error ? e.message : e);
        again = () => void run(f);
      }
      else if (view) stale = pollFailure("read the route", e, true);
      else readErr = pollFailure("read the route", e, false);
      return false;
    }
  }

  // The core moves the route on as you jump, so keep looking. Route changes from other pages also ping watchRoute.
  $effect(() => {
    void run(getRoute, true);
    const t = setInterval(() => void run(getRoute, true), 2000);
    const stop = watchRoute(() => void run(getRoute, true));
    return () => {
      clearInterval(t);
      stop();
    };
  });

  // The inhabited stops' details come from Spansh, once a stop: asked for when the page opens and
  // whenever the route's stops change.
  const stopsKey = $derived(view?.route ? `${view.route.name}\n${view.route.waypoints.map((w) => w.system).join("\n")}` : "");
  $effect(() => {
    if (stopsKey) void run(getRouteDetails, true);
  });

  const route = $derived(view?.route ?? null);
  const done = $derived(route ? route.next >= route.waypoints.length : false);
  const parked = $derived(view?.parked ?? null);
</script>

<Board>
  <div class="page">
    {#if view}<Title {view} onview={(v) => { view = v; err = null; }} run={async (f) => void (await run(f))} />{/if}
    {#if err}<Failure text={err} onaction={() => again?.()} />{/if}
    {#if readErr}<Failure text={readErr} onaction={() => void run(getRoute, true)} />{/if}
    <Stale error={stale} />

    {#if parked}
      <div class="parked type-label">
        <span><span class="paused">Paused</span> {parked.name} · {parked.waypoints.length - parked.next} waypoints left</span>
        <Button onclick={() => run(resumeRoute)}>Resume</Button>
      </div>
    {/if}

    {#if route && view}
      <div class="head">
        <span class="type-label">{route.source ? `${route.source} · ` : ""}{view.remaining} of {route.waypoints.length} waypoints left</span>
      </div>

      <Timeline {view} act={run} />
      {#if done}<p class="type-body">Route finished: you reached the last waypoint.</p>{/if}
    {:else if view}
      <p class="type-body">No route. Type a destination above to plot one, or plot in the galaxy map, follow the engineer trip, or add a search result as a stop.</p>
    {/if}
  </div>
</Board>

<style>
  .page { display: flex; flex-direction: column; gap: var(--sp-6); }
  p { margin: 0; }
  .head { display: flex; flex-direction: column; gap: var(--sp-1); }
  .paused { color: var(--orange-mid); }
  .parked { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: var(--sp-2) var(--sp-3); padding: var(--sp-2) 0; border-bottom: 1px solid var(--line); color: var(--accent); }
</style>
