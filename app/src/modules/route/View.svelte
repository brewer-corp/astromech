<script lang="ts">
  // Route (docs/PLOTTING.md, layer 1): the route the core follows. Its next waypoint is on the
  // clipboard for the galaxy map's search box; jumping into it copies the one after. Routes come
  // from Spansh's plotters (Plot.svelte), the engineer trip, Go on a search result or an import
  // (a Spansh CSV, or a list of systems).
  import { backWaypoint, clearRoute, copyNext, getRoute, importRoute, skipWaypoint, type RouteView } from "../../lib/route";
  import Plot from "./Plot.svelte";

  let view = $state<RouteView | null>(null);
  let err = $state<string | null>(null);
  let name = $state("");
  let text = $state("");

  async function run(f: () => Promise<RouteView>) {
    try {
      view = await f();
      err = null;
    } catch (e) {
      err = String(e instanceof Error ? e.message : e);
    }
  }

  // The core moves the route on as you jump, so keep looking.
  $effect(() => {
    void run(getRoute);
    const t = setInterval(() => void run(getRoute), 2000);
    return () => clearInterval(t);
  });

  async function start() {
    await run(() => importRoute(name.trim() || "Imported route", text));
    if (!err) text = "";
  }

  const route = $derived(view?.route ?? null);
  const done = $derived(route ? route.next >= route.waypoints.length : false);
</script>

<section>
  {#if err}<p class="err">{err}</p>{/if}

  {#if route}
    <div class="head">
      <h3>{route.name}</h3>
      <span class="lo">{route.source ? `${route.source} · ` : ""}{view?.remaining} of {route.waypoints.length} waypoints left</span>
    </div>

    <div class="next">
      {#if view?.next}
        <span class="sublabel">Next waypoint</span>
        <span class="sys">{view.next.system}</span>
        {#if view.next.label}<span class="lo">{view.next.label}</span>{/if}
        <span class:warn={!!view.clipboard_error} class="clip">
          {view.clipboard_error ? `Not on the clipboard: ${view.clipboard_error}` : "On your clipboard"}
        </span>
      {:else}
        <span class="sublabel">Route finished</span>
        <span class="lo">You reached the last waypoint.</span>
      {/if}
    </div>

    <div class="actions">
      <button onclick={() => run(copyNext)} disabled={done}>Copy again</button>
      <button onclick={() => run(skipWaypoint)} disabled={done}>Skip</button>
      <button onclick={() => run(backWaypoint)} disabled={route.next === 0}>Back</button>
      <button onclick={() => run(clearRoute)}>Clear route</button>
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
    <p class="lo">No route. Plot one below, follow the engineer trip, press Go on a search result, or import one.</p>
  {/if}

  <div class="how">
    <h4>In the galaxy map</h4>
    <p>Select the search box, then say <span class="q">“paste”</span> or press your paste button (<span class="mono">controls.paste</span> in the config). astromech sends Ctrl+V and Enter, only while the galaxy map is open. Then plot the route as usual.</p>
    <p>When you jump into the waypoint, the next one is copied and read out. Say <span class="q">“next waypoint”</span> to hear it again, <span class="q">“skip waypoint”</span> or <span class="q">“clear route”</span>.</p>
  </div>

  <Plot onfollow={(v) => (view = v)} />

  <div class="import">
    <h4>Import</h4>
    <input placeholder="Name (optional)" bind:value={name} />
    <textarea rows="6" placeholder="Paste a Spansh CSV (neutron, road to riches, carrier…) or one system name per line" bind:value={text}></textarea>
    <div class="actions">
      <button onclick={start} disabled={!text.trim()}>Follow this route</button>
    </div>
  </div>
</section>

<style>
  section { padding: 28px 0; max-width: 60rem; display: flex; flex-direction: column; gap: 20px; }
  h3 { margin: 0; color: var(--text-hi); font-weight: 500; }
  h4 { margin: 0 0 6px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  p { margin: 0 0 6px; }
  .head { display: flex; align-items: baseline; gap: 16px; }
  .next { display: flex; flex-direction: column; gap: 2px; background: var(--cell-opt); padding: 12px; }
  .next .sys { font-size: 1.4em; color: var(--text-hi); }
  .sublabel { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .clip { color: var(--accent); font-size: var(--fs-s); }
  .warn { color: var(--text-hi); }
  .actions { display: flex; gap: 28px; }
  button { all: unset; cursor: pointer; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover { color: var(--text-hi); }
  .stops { margin: 0; padding-left: 1.6em; display: flex; flex-direction: column; gap: 4px; font-size: var(--fs-s); }
  .stops .sys { color: var(--text); }
  .stops .past .sys { color: var(--text-lo); text-decoration: line-through; }
  .stops .current .sys { color: var(--accent); }
  .lo { color: var(--text-lo); }
  .q { color: var(--text-hi); }
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
  .how p { color: var(--text); font-size: var(--fs-s); line-height: 1.45; }
  .import { display: flex; flex-direction: column; gap: 8px; }
  input, textarea { background: var(--cell-opt); color: var(--text); border: 0; padding: 8px 10px; font: inherit; font-size: var(--fs-s); }
  textarea { font-family: var(--mono); resize: vertical; }
  .err { color: var(--text-hi); background: var(--accent); padding: 8px 12px; }
</style>
