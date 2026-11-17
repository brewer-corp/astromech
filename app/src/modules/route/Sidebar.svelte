<script lang="ts">
  // The route sidebar (DESIGN.md Route sidebar; docs/PLOTTING.md, layer 1): the route the core
  // follows, part of the frame on every page. Its heading is ROUTE with a chevron that folds it to
  // the route strip, or closes it where it is open over the page. Under it the plotter
  // (Title.svelte): plot, replot, the plot setup, and Import, Open on Spansh and Clear route in its
  // ⋯ menu; then the route as a timeline (Timeline.svelte), which scrolls on its own so the title's
  // dropdowns are never clipped. Its next waypoint is on the clipboard for the galaxy map's search
  // box; jumping into it copies the one after. Routes also come from the header's Find or plot
  // (PlotBar), the engineer trip, a search result's Add stop or Set destination, or the galaxy map
  // itself: a route plotted there is followed quietly. A route replaced by any of them is parked
  // to resume. Skip and Back are voice commands only. A failed action or first read is DESIGN.md's
  // hard error: a red "!", the words, and TRY AGAIN.
  import { getRouteDetails, resumeRoute, type RouteView } from "../../lib/route";
  import Button from "../../lib/ui/Button.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import Glyph from "../../lib/ui/Glyph.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
  import { tip } from "../../lib/ui/tip";
  import { useFollow } from "./follow.svelte";
  import Timeline from "./Timeline.svelte";
  import Title from "./Title.svelte";

  /** `onfold` folds the docked sidebar to the strip, or closes it over the page (`over`). */
  let { onfold, over = false }: { onfold?: () => void; over?: boolean } = $props();

  const { follow, own } = useFollow();
  if (own) $effect(() => follow.start());

  /** The last action (Copy, Make next, Clear, import) failed; it stays until the next action. */
  let err = $state<string | null>(null);
  /** That action again, for TRY AGAIN. */
  let again = $state<(() => void) | null>(null);

  /** Run a route action and show its view; false when it failed (shown here). */
  async function run(f: () => Promise<RouteView>): Promise<boolean> {
    try {
      await follow.run(f);
      err = null;
      return true;
    } catch (e) {
      err = String(e instanceof Error ? e.message : e);
      again = () => void run(f);
      return false;
    }
  }

  const view = $derived(follow.view);
  // The inhabited stops' details come from Spansh, once a stop: asked for while the sidebar is
  // shown, and whenever the route's stops change.
  const stopsKey = $derived(view?.route ? `${view.route.name}\n${view.route.waypoints.map((w) => w.system).join("\n")}` : "");
  $effect(() => {
    if (stopsKey) void follow.run(getRouteDetails, true);
  });

  const route = $derived(view?.route ?? null);
  const done = $derived(route ? route.next >= route.waypoints.length : false);
  const parked = $derived(view?.parked ?? null);
  /** "38 jumps left" where every stop ahead knows its jumps, else "3 stops left". */
  const left = $derived.by(() => {
    const jumps = view?.jumps_left ?? null;
    const n = jumps ?? view?.remaining ?? 0;
    return `${n.toLocaleString("en")} ${jumps !== null ? "jump" : "stop"}${n === 1 ? "" : "s"} left`;
  });
</script>

<div class="side" role="complementary" aria-label="Route">
  <div class="head">
    <span class="heading">Route</span>
    {#if onfold}
      <button class="fold" aria-label={over ? "Close the route" : "Collapse the route to a strip"} onclick={onfold} {@attach tip(over ? "Close the route" : "Collapse the route to a strip")}>
        <Glyph name="chevron" />
      </button>
    {/if}
  </div>

  <div class="top">
    {#if view}
      <Title {view} onview={(v) => { follow.view = v; err = null; }} run={async (f) => void (await run(f))}>
        {#snippet where()}{#if route && !done}{route.next + 1} of {route.waypoints.length} · {left}{/if}{/snippet}
      </Title>
    {/if}
    {#if err}<Failure text={err} onaction={() => again?.()} />{/if}
    {#if follow.readErr}<Failure text={follow.readErr} onaction={() => void follow.read()} />{/if}
    <Stale error={follow.stale} />

    {#if parked}
      <div class="parked type-label">
        <span><span class="paused">Paused</span> {parked.name} · {parked.waypoints.length - parked.next} waypoints left</span>
        <Button onclick={() => run(resumeRoute)}>Resume</Button>
      </div>
    {/if}
    {#if route && done}
      <p class="type-label">Route finished: you reached the last waypoint.</p>
    {:else if view && !route}
      <p class="type-label">No route. Type a destination above to plot one, or plot in the galaxy map, follow the engineer trip, or add a search result as a stop.</p>
    {/if}
  </div>

  {#if route && view}
    <div class="tl"><Timeline {view} act={run} /></div>
  {/if}
</div>

<style>
  /* DESIGN.md Route sidebar: Hull, its heading ROUTE in the Nav role (caps, Label Grey) with the
     chevron at its right; the plotter, then the timeline, which takes the rest and scrolls. */
  .side { box-sizing: border-box; display: flex; flex-direction: column; height: 100%; min-height: 0; background: var(--surface); color: var(--prose); }
  .head { display: flex; align-items: center; justify-content: space-between; flex: none; height: var(--row); margin: var(--sp-3) var(--sp-3) 0 var(--side); }
  .heading { text-transform: uppercase; letter-spacing: var(--caps); font-weight: 400; color: var(--label-grey); }
  button { all: unset; cursor: pointer; }
  .fold { display: grid; place-items: center; width: 1.75rem; height: 1.75rem; color: var(--label-grey); }
  .fold:hover { color: var(--orange-lit); }
  .fold:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .fold :global(.glyph) { transform: rotate(90deg); }
  .top { display: flex; flex-direction: column; gap: var(--sp-3); flex: none; padding: var(--sp-2) var(--sp-4) var(--sp-4) var(--side); }
  .tl { flex: 1 1 0; min-height: 0; overflow-y: auto; padding: 0 var(--sp-4) var(--sp-6) var(--side); scrollbar-gutter: stable; }
  p { margin: 0; }
  .paused { color: var(--label-grey); }
  .parked { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: var(--sp-2) var(--sp-3); padding: var(--sp-2) 0; border-bottom: 1px solid var(--line); color: var(--prose); }
</style>
