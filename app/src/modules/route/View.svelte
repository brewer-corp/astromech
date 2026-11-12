<script lang="ts">
  // Route (docs/PLOTTING.md, layer 1): the route the core follows. Its next waypoint is on the
  // clipboard for the galaxy map's search box; jumping into it copies the one after. The page's
  // title is the plotter (Title.svelte): plot, replot, the plot setup, and Import, Open on Spansh
  // and Clear route in its ⋯ menu. Routes also come from the header's Find or plot (PlotBar),
  // the engineer trip, a search result's Add stop or Set destination, or the galaxy map itself: a
  // route plotted there is followed quietly. A route replaced by any of them is parked to resume.
  // The route strip (Strip.svelte) shows the next stop on every page and links here. A failed
  // action or first read is DESIGN.md's hard error: a red "!", the words, and TRY AGAIN.
  import { tip } from "../../lib/ui/tip";
  import { GAME_SOURCE, backWaypoint, copyNext, getRoute, resumeRoute, skipWaypoint, watchRoute, type RouteView } from "../../lib/route";
  import { pollFailure } from "../../lib/ui/errors";
  import Button from "../../lib/ui/Button.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import Glyph from "../../lib/ui/Glyph.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
  import Title from "./Title.svelte";

  let view = $state<RouteView | null>(null);
  /** The last action (Copy, Skip, Back, Clear) failed; it stays until the next action. */
  let err = $state<string | null>(null);
  /** That action again, for TRY AGAIN. */
  let again = $state<(() => void) | null>(null);
  /** The first read failed: nothing to show yet. */
  let readErr = $state<string | null>(null);
  /** The last poll failed while a route is shown: the route stays, with a line saying so. */
  let stale = $state<string | null>(null);

  async function run(f: () => Promise<RouteView>, polled = false) {
    try {
      view = await f();
      if (!polled) err = null;
      readErr = null;
      stale = null;
    } catch (e) {
      if (!polled) {
        err = String(e instanceof Error ? e.message : e);
        again = () => void run(f);
      }
      else if (view) stale = pollFailure("read the route", e, true);
      else readErr = pollFailure("read the route", e, false);
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

  const route = $derived(view?.route ?? null);
  const done = $derived(route ? route.next >= route.waypoints.length : false);
  const fromGame = $derived(route?.source === GAME_SOURCE);
  const parked = $derived(view?.parked ?? null);
</script>

<section>
  {#if view}<Title {view} onview={(v) => { view = v; err = null; }} run={(f) => run(f)} />{/if}
  {#if err}<Failure text={err} onaction={() => again?.()} />{/if}
  {#if readErr}<Failure text={readErr} onaction={() => void run(getRoute, true)} />{/if}
  <Stale error={stale} />

  {#if parked}
    <div class="parked">
      <span><span class="sublabel">Paused</span> {parked.name} · {parked.waypoints.length - parked.next} waypoints left</span>
      <Button onclick={() => run(resumeRoute)}>Resume</Button>
    </div>
  {/if}

  {#if route}
    <div class="head">
      <span class="lo">{route.source ? `${route.source} · ` : ""}{view?.remaining} of {route.waypoints.length} waypoints left</span>
    </div>

    <div class="next">
      {#if view?.next && fromGame}
        <span class="sublabel">Next jump</span>
        <span class="sys">{view.next.system}</span>
        <span class="lo">Plotted in the galaxy map, so astromech follows it quietly and copies nothing as you jump.</span>
      {:else if view?.next}
        <span class="sublabel">Next waypoint</span>
        <span class="sys">{view.next.system}</span>
        {#if view.next.label}<span class="lo">{view.next.label}</span>{/if}
        {#if view.clipboard_error}
          <Failure text={`Not on the clipboard: ${view.clipboard_error}`} onaction={() => void run(copyNext)} />
        {:else}
          <span class="clip"><Glyph name="copied" /> On your clipboard</span>
        {/if}
      {:else}
        <span class="sublabel">Route finished</span>
        <span class="lo">You reached the last waypoint.</span>
      {/if}
    </div>

    <div class="actions">
      <Button onclick={() => run(copyNext)} disabled={done} {@attach tip("Copy the next waypoint again")}><Glyph name="copy" /> Copy</Button>
      <Button onclick={() => run(skipWaypoint)} disabled={done}>Skip</Button>
      <Button onclick={() => run(backWaypoint)} disabled={route.next === 0}>Back</Button>
    </div>

    <ol class="stops">
      {#each route.waypoints as w, i (i)}
        <li class:past={i < route.next} class:current={i === route.next}>
          <span class="sys">{w.system}</span>{#if w.label}<span class="lo">&nbsp;· {w.label}</span>{/if}
          {#if i === 0 && view?.here && w.system.toLowerCase() === view.here.toLowerCase()}<span class="lo"> · you are here</span>{/if}
        </li>
      {/each}
    </ol>
  {:else if view}
    <p class="lo empty">No route. Type a destination above to plot one, or plot in the galaxy map, follow the engineer trip, or add a search result as a stop.</p>
  {/if}
</section>

<style>
  section { padding: 28px 0; max-width: 60rem; display: flex; flex-direction: column; gap: 20px; }
  p { margin: 0 0 6px; }
  .head { display: flex; flex-direction: column; gap: 4px; }
  /* The next waypoint is a read-only fact: hull (DESIGN.md Surfaces and cells). */
  .next { display: flex; flex-direction: column; gap: 4px; background: var(--surface); padding: 14px 16px; }
  .next .sys { font-size: 1.45em; color: var(--orange-lit); line-height: 1.15; }
  .sublabel { color: var(--orange-mid); font-size: var(--fs-s); }
  .clip { display: inline-flex; align-items: center; gap: 6px; color: var(--accent); font-size: var(--fs-s); }
  .actions { display: flex; flex-wrap: wrap; gap: 12px; align-items: center; }
  .stops { margin: 0; padding-left: 1.4em; display: flex; flex-direction: column; gap: 6px; font-size: var(--fs); }
  .stops .sys { color: var(--accent); }
  .stops .past .sys { color: var(--orange-dim); text-decoration: line-through; }
  .stops .current .sys { color: var(--orange-lit); }
  .lo { color: var(--orange-mid); }
  .empty { line-height: 1.45; color: var(--prose); }
  .parked { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 8px 12px; padding: 10px 0; border-bottom: 1px solid var(--line); color: var(--accent); font-size: var(--fs-s); }
</style>
