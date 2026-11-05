<script lang="ts">
  // Route (docs/PLOTTING.md, layer 1): the route the core follows. Its next waypoint is on the
  // clipboard for the galaxy map's search box; jumping into it copies the one after. Routes come
  // from the top bar's plotter (PlotBar), the engineer trip, a search result's Add stop or Set
  // destination, an import (a Spansh CSV, or a list of systems) or the galaxy map itself: a route
  // plotted there is followed quietly. A route replaced by any of them is parked to resume.
  import { GAME_SOURCE, backWaypoint, clearRoute, copyNext, getRoute, importRoute, resumeRoute, skipWaypoint, watchRoute, type RouteView } from "../../lib/route";
  import { pollFailure } from "../../lib/ui/errors";
  import Button from "../../lib/ui/Button.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import Glyph from "../../lib/ui/Glyph.svelte";
  import Stale from "../../lib/ui/Stale.svelte";

  let { compact = false }: { compact?: boolean } = $props();

  let view = $state<RouteView | null>(null);
  /** The last action (Copy, Skip, Back, Clear, import) failed; it stays until the next action. */
  let err = $state<string | null>(null);
  /** The first read failed: nothing to show yet. */
  let readErr = $state<string | null>(null);
  /** The last poll failed while a route is shown: the route stays, with a line saying so. */
  let stale = $state<string | null>(null);
  let name = $state("");
  let text = $state("");

  async function run(f: () => Promise<RouteView>, polled = false) {
    try {
      view = await f();
      if (!polled) err = null;
      readErr = null;
      stale = null;
    } catch (e) {
      if (!polled) err = String(e instanceof Error ? e.message : e);
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

  async function start() {
    await run(() => importRoute(name.trim() || "Imported route", text));
    if (!err) text = "";
  }

  const route = $derived(view?.route ?? null);
  const done = $derived(route ? route.next >= route.waypoints.length : false);
  const fromGame = $derived(route?.source === GAME_SOURCE);
  const parked = $derived(view?.parked ?? null);
</script>

<section class:compact>
  {#if err}<p class="err">{err}</p>{/if}
  {#if readErr}<p class="err">{readErr}</p>{/if}
  <Stale error={stale} />

  {#if parked}
    <div class="parked">
      <span><span class="sublabel">Paused</span> {parked.name} · {parked.waypoints.length - parked.next} waypoints left</span>
      <Button onclick={() => run(resumeRoute)}>Resume</Button>
    </div>
  {/if}

  {#if route}
    <div class="head">
      <h3>{route.name}</h3>
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
        <span class:warn={!!view.clipboard_error} class="clip" title={view.clipboard_error ? `Not on the clipboard: ${view.clipboard_error}` : "On your clipboard"}>
          <Glyph name={view.clipboard_error ? "copy" : "copied"} />
          {view.clipboard_error ? view.clipboard_error : "On your clipboard"}
        </span>
      {:else}
        <span class="sublabel">Route finished</span>
        <span class="lo">You reached the last waypoint.</span>
      {/if}
    </div>

    <div class="actions">
      <Button onclick={() => run(copyNext)} disabled={done} title="Copy the next waypoint again"><Glyph name="copy" /> Copy</Button>
      <Button onclick={() => run(skipWaypoint)} disabled={done}>Skip</Button>
      <Button onclick={() => run(backWaypoint)} disabled={route.next === 0}>Back</Button>
      <Button onclick={() => run(clearRoute)}>Clear route</Button>
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
    <p class="lo empty">{compact ? "No route. Search a system in the top bar to plot one, plot in the galaxy map, or add a search result as a stop." : "No route. Search a system in the top bar to plot one, plot in the galaxy map, follow the engineer trip, add a search result as a stop or make it the destination, or import one."}</p>
  {/if}

  {#if compact}
    <details class="block">
      <summary>Galaxy map</summary>
      <div class="how">
        <p>Select the search box, then say <span class="q">“paste”</span> or press your paste button. astromech sends Ctrl+V and Enter while the galaxy map is open.</p>
        <p>Jumping into the waypoint copies the next. Say <span class="q">“next waypoint”</span>, <span class="q">“skip waypoint”</span> or <span class="q">“clear route”</span>.</p>
      </div>
    </details>
    <details class="block">
      <summary>Import</summary>
      <div class="import">
        <Input aria-label="Route name" placeholder="Name (optional)" bind:value={name} />
        <Input rows={5} aria-label="Route" placeholder="Paste a Spansh CSV or one system name per line" bind:value={text} />
        <div class="actions">
          <Button kind="primary" onclick={start} disabled={!text.trim()}>Follow this route</Button>
        </div>
      </div>
    </details>
  {:else}
    <div class="how">
      <h4>In the galaxy map</h4>
      <p>Select the search box, then say <span class="q">“paste”</span> or press your paste button (<span class="mono">controls.paste</span> in the config). astromech sends Ctrl+V and Enter, only while the galaxy map is open. Then plot the route as usual.</p>
      <p>When you jump into the waypoint, the next one is copied and read out. Say <span class="q">“next waypoint”</span> to hear it again, <span class="q">“skip waypoint”</span> or <span class="q">“clear route”</span>.</p>
    </div>

    <div class="import">
      <h4>Import</h4>
      <Input aria-label="Route name" placeholder="Name (optional)" bind:value={name} />
      <Input rows={6} aria-label="Route" placeholder="Paste a Spansh CSV (neutron, road to riches, carrier…) or one system name per line" bind:value={text} />
      <div class="actions">
        <Button kind="primary" onclick={start} disabled={!text.trim()}>Follow this route</Button>
      </div>
    </div>
  {/if}
</section>

<style>
  section { padding: 28px 0; max-width: 60rem; display: flex; flex-direction: column; gap: 20px; }
  section.compact { padding: 0; max-width: none; gap: 16px; }
  h3 { margin: 0; color: var(--text-hi); font-weight: 500; }
  h4 { margin: 0 0 6px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  p { margin: 0 0 6px; }
  .head { display: flex; flex-direction: column; gap: 4px; }
  .next { display: flex; flex-direction: column; gap: 4px; background: var(--cell-opt); padding: 14px 16px; }
  .next .sys { font-size: 1.45em; color: var(--text-hi); line-height: 1.15; }
  .sublabel { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .clip { display: inline-flex; align-items: center; gap: 6px; color: var(--accent); font-size: var(--fs-s); }
  .warn { color: var(--text-hi); }
  .actions { display: flex; flex-wrap: wrap; gap: 12px; align-items: center; }
  .stops { margin: 0; padding-left: 1.4em; display: flex; flex-direction: column; gap: 6px; font-size: var(--fs); }
  .stops .sys { color: var(--text); }
  .stops .past .sys { color: var(--text-lo); text-decoration: line-through; }
  .stops .current .sys { color: var(--accent); }
  .lo { color: var(--text-lo); }
  .empty { line-height: 1.45; }
  .q { color: var(--text-hi); }
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
  .how p { color: var(--text); font-size: var(--fs-s); line-height: 1.45; }
  .import { display: flex; flex-direction: column; gap: 8px; }
  .err { color: var(--text-hi); background: var(--accent); padding: 8px 12px; }
  .parked { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 8px 12px; padding: 10px 0; border-bottom: 1px solid var(--accent-dim); color: var(--text); font-size: var(--fs-s); }
  details.block { border-top: 1px solid var(--accent-dim); padding-top: 12px; }
  details.block summary { cursor: pointer; color: var(--text-hi); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); font-weight: 600; margin-bottom: 10px; }
  details.block[open] summary { color: var(--accent); }
</style>
