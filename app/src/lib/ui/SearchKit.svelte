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
  // system goes on the clipboard for the galaxy map. Inside a ListDetail (a Places finder) a
  // row can be selected, and the side panel shows all of its facts with the same two actions.
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
  import { useDetail } from "./layout/listDetail.svelte";
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
    around = false,
    notes = [],
    header,
    lead,
    onanswer,
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
    /** A Places finder's origin: labelled "Around", offering here, the route's next stop and a typed system. */
    around?: boolean;
    /** The lookup's own constraints for the summary line, after where it searched ("metallic rings"). */
    notes?: string[];
    /** Mode tabs or other chrome at the top of the query band. */
    header?: Snippet;
    /** What to find, rendered in the query band (service, module, commodity…). */
    lead?: Snippet;
    /**
     * Each answer as shown: the first row's ranked value and its unit, or null when nothing was
     * found. A Places finder keeps it for its tile.
     */
    onanswer?: (top: { figure: string; unit: string } | null) => void;
  } = $props();

  // Follows `start`; the commander's pick (or ×) overrides it until `start` changes.
  let origin = $derived<Origin>(start);
  /** Filters loaded for `lookup` (null until they arrive). */
  let saved = $state<{ lookup: string; filters: Filters } | null>(null);
  /** The current ship's pad as of the last search (the core resolves it on every search). */
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

  $effect(() => {
    if (!onanswer || busy || err || !res) return;
    const top = rows[0];
    if (!top) return onanswer(null);
    const figure = top.main ?? jumpsFigure(top);
    onanswer({ figure, unit: res.main ? (res.main.unit ?? "") : /^~?1$/.test(figure) ? "jump" : "jumps" });
  });

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
      : ["112px", "minmax(240px, 400px)", ...columns.map((c) => (c.text && !c.width ? "minmax(0, 240px)" : `${c.width ?? 0}px`)), "64px"].join(" "),
  );
  const shown = $derived(compact ? [] : columns);
  const PLACE_HEADS = { station: "Station", body: "Body", ring: "Ring", system: "System" } as const;
  /** The one constraint an empty answer can relax, and how. */
  const widen = $derived(widening(filters, offers, hints));

  /** The ListDetail this list fills, if any: its rows can be selected. */
  const detail = useDetail();
  /** The selected row's key; it stays selected while a new answer still has it. */
  let picked = $state<string | number | null>(null);
  const chosen = $derived(detail && picked != null ? rows.find((r) => r.key === picked) : undefined);
  $effect(() => {
    if (!detail) return;
    detail.min = fits;
    detail.selected = chosen?.key ?? null;
    detail.panel = chosen ? about : null;
  });
  $effect(() => () => {
    if (detail) Object.assign(detail, { panel: null, selected: null, min: null });
  });

  /** TRY AGAIN after a filter change couldn't be saved. */
  function retrySave() {
    if (unsaved) void change(unsaved);
  }

  /** The empty line's action: relax the one constraint it names. */
  function relax() {
    if (widen) void change(widen.filters);
  }
</script>

{#snippet name(r: Results["rows"][number], text: string)}
  {#if detail}
    <button class="name pick" title={text} aria-pressed={chosen?.key === r.key} onclick={() => (picked = r.key)}>{text}</button>
  {:else}
    <span class="name" title={text}>{text}</span>
  {/if}
{/snippet}

<!-- DESIGN.md Side panel: the selected result's name in white, where it is, its ranked value, every
     fact under its head (the compact row drops them), and its two actions as words. -->
{#snippet about()}
  {#if chosen && res}
    {@const r = chosen}
    {@const top = r.main ?? jumpsFigure(r)}
    <h2 class="ptitle">{r.station ?? r.system}</h2>
    <p class="psub">
      {#if r.station}<StationKind type={r.station_type} /><span>{r.station_type ? `${r.station_type} in` : "In"}</span> <SystemChip name={r.system} />{:else}<SystemChip name={r.system} icon />{/if}
    </p>
    <p class="pmain"><span class="v">{top}</span><span class="unit">{res.main ? (res.main.unit ?? "") : /^~?1$/.test(top) ? "jump" : "jumps"}</span></p>
    <dl class="pfacts">
      {#each columns as c (c.key)}
        {@const cell = c.cell(r, now)}
        <dt>{c.head}</dt>
        <dd class:none={cell.value === "—"}>{cell.value}{#if cell.unit}<span class="unit">{cell.unit}</span>{/if}</dd>
      {/each}
    </dl>
    <div class="pacts">
      <Button kind="primary" onclick={() => void destination(r)}>Set destination</Button>
      <Button onclick={() => void add(r)}>Add stop</Button>
    </div>
  {/if}
{/snippet}

<div class="kit">
  <div class="query">
    {#if header}{@render header()}{/if}
    {#if lead}{@render lead()}{/if}
    <OriginInput {origin} {around} onchange={(o) => (origin = o)} />
    {#if saved && offers.length}<SearchFilters filters={saved.filters} shipPad={ship?.lookup === lookup ? ship.pad : undefined} {offers} {hints} onchange={(f) => void change(f)} />{/if}
    {#if filterErr && unsaved}<Failure text={filterErr} onaction={retrySave} />{/if}
  </div>

  <div class="out" bind:this={out}>
    {#if err}
      <Failure text={err} onaction={() => attempt++} />
    {:else if busy}
      <p class="line">{busyText}</p>
    {:else if res}
      <p class="line">{summary(`${title} ${joiner}`, res.origin, filters, offers, hints, hidden, [...notes, ...(res.notes ?? [])], res.reach)}</p>
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
            <!-- The keyboard selects a row with its name, a button; a click anywhere on the row does too. -->
            <!-- svelte-ignore a11y_interactive_supports_focus, a11y_click_events_have_key_events -->
            <div class="row result" class:stale={limit == null && isStale(r, STALE_DAYS, asOf)} class:sel={chosen?.key === r.key} role="row" onclick={() => detail && (picked = r.key)}>
              <span class="r main" role="cell"><span class="v">{r.main ?? jumpsFigure(r)}</span>{#if res.main?.unit}<span class="unit">{res.main.unit}</span>{/if}</span>
              <span class="place" role="cell">
                {#if r.station}
                  <StationKind type={r.station_type} />
                  {@render name(r, r.station)}
                  <span class="sys" title={r.system}>{r.system}</span>
                {:else}
                  {@render name(r, r.system)}
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
          {#if res.reach != null}
            No {noun} within {figure(res.reach)} ly.
          {:else if widen}
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
  /* As wide as the query's controls or the list's columns, whichever is wider: spare width stays
     empty (DESIGN.md The Next To It Rule). */
  .kit { display: flex; flex-direction: column; gap: 0; min-width: 0; width: fit-content; max-width: 100%; }
  .query { display: flex; flex-direction: column; gap: 12px; padding: 18px 20px 20px; border-bottom: 1px solid var(--line); }
  .query :global(.origin) { margin: 0; }
  .out { padding: var(--sp-6) 0 var(--sp-2); min-width: 0; }
  p { margin: 0; }

  /* Label lines: the summary above the heads, loading, a warning and the source line below. */
  .line { margin: 0 0 var(--sp-3); font-size: var(--fs-s); color: var(--orange-mid); }
  .warn { color: var(--accent); }
  .warn b { font-weight: 600; }
  .foot { margin: var(--sp-3) 0 0; }
  .empty { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-2); min-height: var(--row); color: var(--prose); }

  /* DESIGN.md Lookup result row: one 44 px line per result, every row on the list's columns. */
  /* A row is as wide as its columns, not as the window (DESIGN.md The Next To It Rule). */
  .list { display: grid; grid-template-columns: var(--cols); column-gap: var(--sp-3); width: fit-content; max-width: 100%; }
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
  .pick { all: unset; cursor: pointer; font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 0 1 auto; }
  .pick:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .result:has(.pick) { cursor: pointer; }
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
  /* Selected: solid orange with On Orange text, while the side panel shows it (DESIGN.md States). */
  .result.sel, .result.sel:hover { background: var(--accent); }
  .result.sel, .result.sel :is(.v, .unit, .sys, .fact, .none) { color: var(--on-accent); }
  .result.sel :global(:is(.ibtn, .kind)) { color: var(--on-accent); }
  .result.sel .pick:focus-visible { outline-color: var(--on-accent); }

  /* The side panel. */
  .ptitle { margin: 0; color: var(--text-hi); font-size: var(--fs-xl); font-weight: 600; line-height: 1.2; overflow-wrap: anywhere; }
  .psub { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-1) var(--sp-2); margin: var(--sp-1) 0 0; font-size: var(--fs-s); color: var(--orange-mid); }
  .psub :global(.chip) { color: var(--accent); }
  .pmain { margin: var(--sp-6) 0 0; }
  .pmain .v { color: var(--text-hi); font-size: var(--fs-xxl); font-weight: 600; }
  .pfacts { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: var(--sp-2) var(--sp-6); margin: var(--sp-6) 0 0; }
  .pfacts dt { color: var(--orange-mid); font-size: var(--fs-s); align-self: baseline; }
  .pfacts dd { margin: 0; overflow-wrap: anywhere; }
  .pacts { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-4); margin: var(--sp-8) 0 0; }

  /* Loading: the old rows stay, Dim, until the answer replaces them. */
  .busy .result, .busy .result .main .v, .busy .result .fact, .busy .result .sys, .busy .result .unit { color: var(--orange-dim); }
</style>
