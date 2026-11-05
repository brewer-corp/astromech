<script lang="ts">
  // The "search near me" kit every lookup uses: pick where to search from (current or typed
  // system) and the lookup's remembered filters, run the lookup, and list the results as
  // DESIGN.md's Lookup result row: the value the list is ranked by first and largest (price,
  // score, or jumps at the current range), the place with its copy icon, the lookup's own fact
  // columns, then distance, jumps, pad, arrival and report age, each under a column head. A
  // one-line summary of the constraints sits above the heads, and the list shows DESIGN.md's
  // states: stale reports stepped down, loading, empty with one way to widen, and a failure
  // with TRY AGAIN. Each result has two actions for the core's route follower
  // (docs/PLOTTING.md): Add stop makes it the next stop of the route followed, and Set
  // destination follows it on its own, parking the route it replaces for Resume. Either way its
  // system goes on the clipboard for the galaxy map.
  import { ago } from "../galaxy";
  import { changeFilters, openFilters, type FilterStore } from "../filterstate";
  import { addedNote, addStop, failedNote, followedNote, followRoute, type FollowNote, type Waypoint } from "../route";
  import {
    ageLimit,
    DEFAULT_FILTERS,
    factColumns,
    figure,
    isStale,
    jumpsFigure,
    loadFilters,
    saveFilters,
    STALE_DAYS,
    summary,
    widening,
    type FilterHints,
    type FilterKey,
    type Filters,
    type Origin,
    type Pad,
    type Results,
  } from "../search";
  import Button from "./Button.svelte";
  import Failure from "./Failure.svelte";
  import FollowNoteLine from "./FollowNote.svelte";
  import IconButton from "./IconButton.svelte";
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
    noun = "results",
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
    /** How the summary says where it searched when there is no radius: "Nearest", "Best prices anywhere". */
    title?: string;
    /** Between the title and the origin: "Nearest to Sol", "Best prices anywhere · distances from Sol". */
    joiner?: string;
    /** What the results are, for the empty line: "No buyers within 100 ly." */
    noun?: string;
    /** The loading line, naming who is asked: "Asking Spansh…". */
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
  /** The filter change that couldn't be saved, for TRY AGAIN. */
  let unsaved = $state<Filters | null>(null);
  /** Bumped by TRY AGAIN after a failed search, to run it again. */
  let attempt = $state(0);
  const store: FilterStore = { load: loadFilters, save: saveFilters };
  let busy = $state(false);
  /** What Add stop or Set destination did, or why it failed. */
  let goNote = $state<FollowNote | null>(null);
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
    unsaved = st.error ? next : null;
    if (JSON.stringify(st.filters) !== JSON.stringify(next)) saved = { lookup: l, filters: st.filters };
  }

  $effect(() => {
    const o = origin;
    const s = saved;
    void attempt;
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

  /** The result as a stop: its system, labelled with the station or what was found there. */
  function stop(r: Results["rows"][number]): Waypoint {
    const label = r.station ?? res?.label;
    return { system: r.system, ...(label ? { label } : {}) };
  }

  /** Add stop: the result becomes the next stop of the route followed. */
  async function add(r: Results["rows"][number]) {
    try {
      goNote = addedNote(await addStop(stop(r), "search"));
    } catch (e) {
      goNote = failedNote("Couldn't add it", e, () => void add(r));
    }
  }

  /** Set destination: a one-stop route to the result; the route it replaces is parked. */
  async function destination(r: Results["rows"][number]) {
    try {
      goNote = followedNote(await followRoute(r.system, "search", [stop(r)]));
    } catch (e) {
      goNote = failedNote("Couldn't follow it", e, () => void destination(r));
    }
  }

  const filters = $derived(saved?.filters ?? DEFAULT_FILTERS);
  /** The age filter in force; with none, reports older than STALE_DAYS are shown stepped down. */
  const limit = $derived(offers.includes("age") ? ageLimit(filters, hints) : null);
  /** Ages are measured when the answer arrives, so a list doesn't age while it is read. */
  const now = $derived(res ? Date.now() : 0);
  /** Staleness is judged as of the answer, as the source's own age filter was (a cached answer included). */
  const asOf = $derived(res ? Date.parse(res.fetched) || now : 0);
  /** With an age filter, reports older than it are hidden and counted. Sources apply the filter in the query, so this
   * counts only what came back past it; the summary names the limit either way. */
  const rows = $derived(res ? (limit == null ? res.rows : res.rows.filter((r) => !isStale(r, limit, asOf))) : []);
  const hidden = $derived(res ? res.rows.length - rows.length : 0);
  const columns = $derived(res ? factColumns(res) : []);

  /** The list's width, to fall back to the compact row where the columns don't fit. */
  let out = $state<HTMLElement | null>(null);
  let width = $state(0);
  $effect(() => {
    const el = out;
    if (!el || typeof ResizeObserver === "undefined") return;
    const ro = new ResizeObserver(() => (width = el.clientWidth));
    ro.observe(el);
    return () => ro.disconnect();
  });
  /**
   * The narrowest the full row fits in. Text columns share what the place leaves (the place
   * first) and give way to nothing; below this, the facts go.
   */
  const fits = $derived(112 + 240 + 64 + columns.reduce((n, c) => n + (c.width ?? 0), 0) + 12 * (columns.length + 2));
  /**
   * Too narrow for the facts (a side panel): the ranked value, the place and the actions only,
   * by The Aligned Columns Rule (a fact that would wrap goes).
   */
  const compact = $derived(width > 0 && width < fits);
  const template = $derived(
    compact
      ? "max-content minmax(0, 1fr) 64px"
      : ["112px", "minmax(240px, 1.5fr)", ...columns.map((c) => (c.text ? "minmax(0, 1fr)" : `${c.width ?? 0}px`)), "64px"].join(" "),
  );
  const shown = $derived(compact ? [] : columns);
  const PLACE_HEADS = { station: "Station", body: "Body", system: "System" } as const;
  /** The one constraint an empty answer can relax, and how. */
  const widen = $derived(widening(filters, offers, hints));

  /** TRY AGAIN after a filter change couldn't be saved. */
  function retrySave() {
    if (unsaved) void change(unsaved);
  }

  /** The empty line's action: relax the one constraint it names. */
  function relax() {
    if (widen) void change(widen.filters);
  }
</script>

<div class="kit">
  <div class="query">
    {#if header}{@render header()}{/if}
    {#if lead}{@render lead()}{/if}
    <OriginInput {origin} onchange={(o) => (origin = o)} />
    {#if saved && offers.length}<SearchFilters filters={saved.filters} shipPad={ship?.lookup === lookup ? ship.pad : undefined} {offers} {hints} onchange={(f) => void change(f)} />{/if}
    {#if filterErr && unsaved}<Failure text={filterErr} onaction={retrySave} />{/if}
  </div>

  <div class="out" bind:this={out}>
    {#if err}
      <Failure text={err} onaction={() => attempt++} />
    {:else if busy}
      <p class="line">{busyText}</p>
    {:else if res}
      <p class="line">{summary(`${title} ${joiner}`, res.origin, filters, offers, hints, hidden)}</p>
    {/if}
    {#if res && !err}
      {#if res.warning}<p class="line warn"><b>!</b> {res.warning}</p>{/if}
      {#if rows.length}
        <div class="list" class:busy role="table" aria-label="Results" aria-busy={busy} style:--cols={template}>
          <div class="row heads" role="row">
            <span class="r" role="columnheader">{res.main?.head ?? "Jumps"}</span>
            <span role="columnheader">{PLACE_HEADS[res.place]}</span>
            {#each shown as c (c.key)}<span class:r={!c.text} role="columnheader">{c.head}</span>{/each}
            <span role="columnheader" aria-label="Actions"></span>
          </div>
          {#each rows as r (r.key)}
            <div class="row result" class:stale={limit == null && isStale(r, STALE_DAYS, asOf)} role="row">
              <span class="r main" role="cell"><span class="v">{r.main ?? jumpsFigure(r)}</span>{#if res.main?.unit}<span class="unit">{res.main.unit}</span>{/if}</span>
              <span class="place" role="cell">
                {#if r.station}
                  <StationKind type={r.station_type} />
                  <span class="name" title={r.station}>{r.station}</span>
                  <span class="sys" title={r.system}>{r.system}</span>
                {:else}
                  <span class="name" title={r.system}>{r.system}</span>
                {/if}
                <SystemChip name={r.system} icon />
              </span>
              {#each shown as c (c.key)}
                {@const cell = c.cell(r, now)}
                {#if cell.text}
                  <span class="fact text" class:age={c.key === "age"} role="cell" title={cell.value}>{cell.value}</span>
                {:else}
                  <span class="fact r" class:age={c.key === "age"} class:none={cell.value === "—"} role="cell">{cell.value}{#if cell.unit}<span class="unit">{cell.unit}</span>{/if}</span>
                {/if}
              {/each}
              <span class="acts" role="cell">
                <IconButton icon="add-stop" label={`Add ${r.system} as a stop`} tip={`Add ${r.system} as the next stop`} onclick={() => void add(r)} />
                <IconButton icon="destination" label={`Set ${r.system} as the destination`} tip={`Set ${r.system} as the destination (replaces the route, which Resume brings back)`} onclick={() => void destination(r)} />
              </span>
            </div>
          {/each}
        </div>
      {:else if !busy}
        <p class="empty">
          {#if widen}
            No {noun} {widen.phrase}.
            <Button onclick={relax}>{widen.label}</Button>
          {:else}
            No {noun} found.
          {/if}
        </p>
      {/if}
      <FollowNoteLine bind:note={goNote} />
      <p class="line foot">At {figure(res.range, 1)} ly per jump · {res.source}, {ago(res.fetched)}{res.rows.some((r) => !r.exact) ? " · ~ = straight-line estimate" : ""}</p>
    {/if}
  </div>
</div>

<style>
  .kit { display: flex; flex-direction: column; gap: 0; min-width: 0; }
  .query { display: flex; flex-direction: column; gap: 12px; padding: 18px 20px 20px; background: var(--bg-list); border-bottom: 1px solid var(--accent-dim); }
  .query :global(.origin) { margin: 0; }
  /* Mixed case whatever the host panel sets (The Caps Are Wayfinding Rule): system names carry meaning in their case. */
  .out { padding: var(--sp-6) 0 var(--sp-2); min-width: 0; max-width: var(--list-w); text-transform: none; letter-spacing: normal; }
  p { margin: 0; }

  /* Label lines: the summary above the heads, loading, a warning and the source line below. */
  .line { margin: 0 0 var(--sp-3); font-size: var(--fs-s); color: var(--orange-mid); }
  .warn { color: var(--accent); }
  .warn b { font-weight: 600; }
  .foot { margin: var(--sp-3) 0 0; }
  .empty { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-2); min-height: var(--row); color: var(--prose); }

  /* DESIGN.md Lookup result row: one 44 px line per result, every row on the list's columns. */
  .list { display: grid; grid-template-columns: var(--cols); column-gap: var(--sp-3); font-variant-numeric: tabular-nums; }
  .row { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: center; height: var(--row-result); white-space: nowrap; color: var(--accent); font-size: var(--fs); }
  .row > * { min-width: 0; }
  .heads { height: 28px; border-bottom: 1px solid var(--line); color: var(--orange-mid); font-size: var(--fs-s); }
  .result:hover { background: var(--row-hover); }
  .result:hover .acts :global(.ibtn) { color: var(--orange-lit); }
  .r { text-align: right; }
  .main .v { color: var(--orange-lit); font-size: var(--fs-l); font-weight: 600; }
  .unit { margin-left: 3px; color: var(--orange-mid); font-size: var(--fs-s); }
  .place { display: flex; align-items: center; gap: var(--sp-2); overflow: hidden; }
  .place :global(.kind) { color: var(--orange-mid); flex: none; }
  .name { font-weight: 500; overflow: hidden; text-overflow: ellipsis; flex: 0 1 auto; }
  .sys { color: var(--orange-mid); font-size: var(--fs-s); overflow: hidden; text-overflow: ellipsis; flex: 0 1000 auto; }
  .place :global(.chip) { flex: none; margin-left: calc(-1 * var(--sp-1)); }
  .fact { overflow: hidden; text-overflow: ellipsis; }
  .text { font-size: var(--fs-s); }
  .none { color: var(--orange-mid); }
  .acts { display: flex; justify-content: flex-end; gap: var(--sp-1); }

  /* Stale: the name stays Orange, the value and facts step down to Mid, and the age, now the
     brightest fact, stays Orange. */
  .stale .main .v, .stale .fact { color: var(--orange-mid); }
  .stale .fact.age { color: var(--accent); }
  /* Loading: the old rows stay, Dim, until the answer replaces them. */
  .busy .result, .busy .result .main .v, .busy .result .fact, .busy .result .sys, .busy .result .unit { color: var(--orange-dim); }
</style>
