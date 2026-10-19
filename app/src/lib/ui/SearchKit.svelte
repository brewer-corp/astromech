<script lang="ts">
  // The "search near me" kit every lookup uses: pick where to search from (current or typed
  // system) and the lookup's remembered filters, run the lookup, and list the nearest results
  // with jumps at the current range, distance, pad size, arrival distance and data age. Go on a
  // result hands it to the core's route follower (docs/PLOTTING.md): its system goes on the
  // clipboard for the galaxy map's search box.
  import { ago, jumps } from "../galaxy";
  import { changeFilters, openFilters, type FilterStore } from "../filterstate";
  import { followedNote, followRoute } from "../route";
  import { arrivalLabel, DEFAULT_FILTERS, loadFilters, saveFilters, stateLabel, type FilterHints, type FilterKey, type Filters, type Origin, type Pad, type Results } from "../search";
  import Allegiance from "./Allegiance.svelte";
  import OriginInput from "./OriginInput.svelte";
  import SearchFilters from "./SearchFilters.svelte";
  import StationKind from "./StationKind.svelte";
  import SystemChip from "./SystemChip.svelte";
  import type { Snippet } from "svelte";

  let {
    lookup,
    offers,
    hints = {},
    run,
    title = "Nearest",
    joiner = "to",
    busyText = "Searching…",
    start = null,
    header,
    lead,
  }: {
    /** Stable id of the lookup: filters are remembered under it, and a change re-runs the search. */
    lookup: string;
    /** The filters this lookup can apply. */
    offers: FilterKey[];
    hints?: FilterHints;
    /** A new function (e.g. another commodity) re-runs the search. */
    run: (origin: Origin, filters: Filters) => Promise<Results>;
    title?: string;
    /** Between the title and the origin: "Nearest to Sol", "Best prices near Sol". */
    joiner?: string;
    busyText?: string;
    /** Where to search from until the commander picks another system; a new one resets the box. */
    start?: Origin;
    /** Mode tabs or other chrome at the top of the query band. */
    header?: Snippet;
    /** What to find, rendered in the query band (service, module, commodity…). */
    lead?: Snippet;
  } = $props();

  // Follows `start`; the commander's pick (or ×) overrides it until `start` changes.
  let origin = $derived<Origin>(start);
  /** Filters loaded for `lookup` (null until they arrive). */
  let saved = $state<{ lookup: string; filters: Filters } | null>(null);
  /** The current ship's pad as of the last search (the shell resolves it on every search). */
  let ship = $state<{ lookup: string; pad: Pad | null } | null>(null);
  let res = $state<Results | null>(null);
  let err = $state<string | null>(null);
  let filterErr = $state<string | null>(null);
  const store: FilterStore = { load: loadFilters, save: saveFilters };
  let busy = $state(false);
  /** What Go did, or why it failed. */
  let goNote = $state<string | null>(null);
  let asked = 0;
  let changed = 0; // filter changes made, so a slow earlier one can't overwrite a later one

  $effect(() => {
    const l = lookup;
    filterErr = null;
    void openFilters(store, l, DEFAULT_FILTERS).then((st) => {
      if (l === lookup) saved = { lookup: l, filters: st.filters };
    });
  });

  async function change(next: Filters) {
    const l = lookup;
    const prev = saved?.filters ?? DEFAULT_FILTERS;
    saved = { lookup: l, filters: next };
    const n = ++changed;
    const st = await changeFilters(store, l, prev, next);
    if (l !== lookup || n !== changed) return;
    filterErr = st.error;
    if (JSON.stringify(st.filters) !== JSON.stringify(next)) saved = { lookup: l, filters: st.filters };
  }

  $effect(() => {
    const o = origin;
    const s = saved;
    if (!s || s.lookup !== lookup) return;
    const n = ++asked;
    busy = true;
    err = null;
    goNote = null;
    void followShip(lookup, n);
    run(o, s.filters)
      .then((r) => {
        if (n === asked) res = r;
      })
      .catch((e: unknown) => {
        if (n !== asked) return;
        res = null;
        err = String(e);
      })
      .finally(() => {
        if (n === asked) busy = false;
      });
  });

  /** The pad label follows the ship the search ran with, not the ship the filters loaded with. */
  async function followShip(l: string, n: number) {
    try {
      const f = await store.load(l);
      if (n === asked && !f.pad_set) ship = { lookup: l, pad: f.pad };
    } catch {
      // the label keeps what it shows; the search itself doesn't depend on this
    }
  }

  /** A one-stop route to the result's system, labelled with the station or what was found there. */
  async function go(r: Results["rows"][number]) {
    const label = r.station ?? res?.label;
    try {
      goNote = followedNote(await followRoute(r.system, "search", [{ system: r.system, ...(label ? { label } : {}) }]));
    } catch (e) {
      goNote = `Couldn't follow it: ${e instanceof Error ? e.message : String(e)}`;
    }
  }
</script>

<div class="kit">
  <div class="query">
    {#if header}{@render header()}{/if}
    {#if lead}{@render lead()}{/if}
    <OriginInput {origin} onchange={(o) => (origin = o)} />
    {#if saved && offers.length}<SearchFilters filters={saved.filters} shipPad={ship?.lookup === lookup ? ship.pad : undefined} {offers} {hints} onchange={(f) => void change(f)} />{/if}
    {#if filterErr}<p class="lo">{filterErr}</p>{/if}
  </div>

  <div class="out">
    {#if busy && !res}
      <p class="lo">{busyText}</p>
    {:else if err}
      <p class="lo">{err}</p>
    {:else if res}
      <h4>{title} {joiner} <SystemChip name={res.origin} /></h4>
      {#if res.warning}<p class="lo">{res.warning}</p>{/if}
      <ul class:stale={busy}>
        {#each res.rows as r (r.key)}
          <li>
            <span class="place">
              <span class="name">
                {#if r.station}<StationKind type={r.station_type} />{/if}
                <span class="who">{r.station ?? r.system}{#if r.station && r.arrival_ls != null}<span class="ls"> ({arrivalLabel(r.arrival_ls)})</span>{/if}</span>
              </span>
              {#if r.lead}<span class="lead">{r.lead}</span>{/if}
            </span>
            <span class="sys">
              <SystemChip name={r.system} icon={!r.station} />
              <span class="dist">{r.ly.toFixed(1)} ly · {jumps(r.jumps, r.exact)}</span>
            </span>
            <span class="facts">
              {#if r.allegiance || r.faction || stateLabel(r.state)}
                <span class="pol">
                  {#if r.allegiance && r.allegiance !== "None"}<Allegiance name={r.allegiance} />{/if}
                  {#if r.faction && r.faction !== "None"}<span class="meta">{r.faction}</span>{/if}
                  {#if stateLabel(r.state)}<span class="meta">{stateLabel(r.state)}</span>{/if}
                </span>
              {/if}
              {#if r.updated_at}<span class="meta">{ago(r.updated_at)}</span>{/if}
              {#if r.detail}<span class="extra">{r.detail}</span>{/if}
            </span>
            <button type="button" class="go" aria-label={`Go to ${r.system}`} title="Follow it: the system goes on the clipboard for the galaxy map" onclick={() => void go(r)}>Go</button>
          </li>
        {:else}
          <li class="none">None found.</li>
        {/each}
      </ul>
      {#if goNote}<p class="lo small">{goNote}</p>{/if}
      <p class="lo small">At {res.range.toFixed(1)} ly per jump · {res.source}, {ago(res.fetched)}{res.rows.some((r) => !r.exact) ? " · ~ = straight-line estimate" : ""}</p>
    {/if}
  </div>
</div>

<style>
  .kit { display: flex; flex-direction: column; gap: 0; min-width: 0; }
  .query { display: flex; flex-direction: column; gap: 12px; padding: 18px 20px 20px; background: var(--bg-list); border-bottom: 1px solid var(--accent-dim); }
  .query :global(.origin) { margin: 0; }
  .out { padding: 24px 4px 8px; min-width: 0; }
  h4 { margin: 0 0 16px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-l); letter-spacing: 0.04em; text-transform: uppercase; }
  ul { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; }
  ul.stale { opacity: 0.5; }
  li { display: grid; grid-template-columns: minmax(12rem, 1.5fr) minmax(12rem, 1.3fr) minmax(10rem, 1.4fr) auto; gap: 10px 18px; align-items: start; padding: 16px 8px; border-bottom: 1px solid #1a1c1e; font-size: var(--fs); line-height: 1.3; }
  li.none { display: block; color: var(--text-lo); border: 0; }
  .place { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
  .name { display: flex; align-items: center; gap: 8px; min-width: 0; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  .who { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .ls { color: var(--text-lo); font-family: var(--mono); font-weight: 400; font-size: var(--fs); }
  .lead { color: var(--accent); font-family: var(--mono); font-size: var(--fs-s); }
  .sys { display: flex; flex-wrap: wrap; align-items: center; gap: 8px; min-width: 0; overflow: hidden; font-size: var(--fs-l); }
  .facts { display: flex; flex-wrap: wrap; align-items: center; gap: 8px 14px; min-width: 0; }
  .pol { display: flex; flex-wrap: wrap; align-items: center; gap: 8px 12px; min-width: 0; }
  .dist { color: var(--accent); font-family: var(--mono); font-size: var(--fs); white-space: nowrap; }
  .meta { color: var(--text-lo); font-size: var(--fs-s); white-space: nowrap; }
  .extra { color: var(--text-lo); font-size: var(--fs-s); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 16rem; }
  .go { all: unset; cursor: pointer; height: 32px; padding: 0 14px; background: var(--cell); color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); justify-self: end; }
  .go:hover, .go:focus-visible { color: var(--text-hi); }
  .lo { color: var(--text-lo); }
  .small { font-size: var(--fs-s); margin-top: 14px; }
  p { margin: 0 0 8px; }
  @media (max-width: 900px) {
    li { grid-template-columns: minmax(0, 1fr) auto; }
    .sys, .facts { grid-column: 1 / -1; }
  }
</style>
