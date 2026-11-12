<script lang="ts">
  // Route (docs/PLOTTING.md, layer 1): the route the core follows, as a timeline (Timeline.svelte).
  // Its next waypoint is on the clipboard for the galaxy map's search box; jumping into it copies
  // the one after. Routes come from the top bar's plotter (PlotBar), the engineer trip, a search
  // result's Add stop or Set destination, an import (a Spansh CSV, or a list of systems) or the
  // galaxy map itself: a route plotted there is followed quietly. A route replaced by any of them
  // is parked to resume. The route strip (Strip.svelte) shows the next stop on every page and
  // links here. Skip and Back are voice commands only. A failed action or first read is
  // DESIGN.md's hard error: a red "!", the words, and TRY AGAIN.
  import { clearRoute, getRoute, getRouteDetails, importRoute, resumeRoute, watchRoute, type RouteView } from "../../lib/route";
  import { pollFailure } from "../../lib/ui/errors";
  import Board from "../../lib/ui/layout/Board.svelte";
  import Button from "../../lib/ui/Button.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
  import Timeline from "./Timeline.svelte";

  let view = $state<RouteView | null>(null);
  /** The last action (Copy, Make next, Clear, import) failed; it stays until the next action. */
  let err = $state<string | null>(null);
  /** That action again, for TRY AGAIN. */
  let again = $state<(() => void) | null>(null);
  /** The first read failed: nothing to show yet. */
  let readErr = $state<string | null>(null);
  /** The last poll failed while a route is shown: the route stays, with a line saying so. */
  let stale = $state<string | null>(null);
  let name = $state("");
  let text = $state("");

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

  async function start() {
    await run(() => importRoute(name.trim() || "Imported route", text));
    if (!err) text = "";
  }

  const route = $derived(view?.route ?? null);
  const done = $derived(route ? route.next >= route.waypoints.length : false);
  const parked = $derived(view?.parked ?? null);
</script>

<Board>
  <div class="page">
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
        <h3 class="type-title">{route.name}</h3>
        <span class="type-label">{route.source ? `${route.source} · ` : ""}{view.remaining} of {route.waypoints.length} waypoints left</span>
      </div>

      <Timeline {view} act={run} />
      {#if done}<p class="type-body">Route finished: you reached the last waypoint.</p>{/if}

      <div class="actions">
        <Button onclick={() => run(clearRoute)}>Clear route</Button>
      </div>
    {:else if view}
      <p class="type-body">No route. Search a system in the top bar to plot one, plot in the galaxy map, follow the engineer trip, add a search result as a stop or make it the destination, or import one.</p>
    {/if}

    <div class="how">
      <h4 class="section">In the galaxy map</h4>
      <p class="type-body">Select the search box, then say <span class="q">“paste”</span> or press your paste button (<span class="type-label">controls.paste</span> in the config). astromech sends Ctrl+V and Enter, only while the galaxy map is open. Then plot the route as usual.</p>
      <p class="type-body">When you jump into the waypoint, the next one is copied and read out. Say <span class="q">“next waypoint”</span> to hear it again, <span class="q">“skip waypoint”</span> or <span class="q">“clear route”</span>.</p>
    </div>

    <div class="import">
      <h4 class="section">Import</h4>
      <Input aria-label="Route name" placeholder="Name (optional)" bind:value={name} />
      <Input rows={6} aria-label="Route" placeholder="Paste a Spansh CSV (neutron, road to riches, carrier…) or one system name per line" bind:value={text} />
      <div class="actions">
        <Button kind="primary" onclick={start} disabled={!text.trim()}>Follow this route</Button>
      </div>
    </div>
  </div>
</Board>

<style>
  .page { display: flex; flex-direction: column; gap: var(--sp-6); }
  h3, h4, p { margin: 0; }
  h4 { margin-bottom: var(--sp-3); }
  .head { display: flex; flex-direction: column; gap: var(--sp-1); }
  .actions { display: flex; flex-wrap: wrap; gap: var(--sp-3); align-items: center; }
  .q { color: var(--accent); }
  .paused { color: var(--orange-mid); }
  .how p + p { margin-top: var(--sp-1); }
  .import { display: flex; flex-direction: column; gap: var(--sp-2); }
  .parked { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: var(--sp-2) var(--sp-3); padding: var(--sp-2) 0; border-bottom: 1px solid var(--line); color: var(--accent); }
</style>
