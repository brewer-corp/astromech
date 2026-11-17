<script lang="ts">
  // The "search near me" kit every lookup uses: pick where to search from (current or typed
  // system) and the lookup's remembered filters, run the lookup, and list the results as
  // DESIGN.md's Lookup result row: two lines in groups (search.ts `groups`). First the value the
  // list is ranked by (price, score, or jumps at the current range), large, over what qualifies
  // it (demand with its bars, measured against `measure`); then the place with its pad and
  // arrival over its system and copy icon; the lookup's own group; the trip (jumps over light
  // years, or a stale report's age); the actions. Facts the commander doesn't pick a row by go
  // to the side panel inside a ListDetail, and stay on the row without one. A one-line summary of
  // the constraints sits above the heads, as the filters' resting state (SearchFilters: CHANGE
  // opens the controls, the origin box among them, so the page opens on its results), and the list shows DESIGN.md's states: stale reports stepped down,
  // loading, empty with one way to widen, and a failure with TRY AGAIN.
  // Each result has two actions for the core's route follower
  // (docs/PLOTTING.md): Add stop makes it the next stop of the route followed, and Set
  // destination follows it on its own, parking the route it replaces for Resume. Either way its
  // system goes on the clipboard for the galaxy map. Inside a ListDetail (a Places finder) a
  // row can be selected, and the side panel shows all of its facts with the same two actions.
  import { ago } from "../galaxy";
  import { scale } from "../scale.svelte";
  import { at100 } from "./layout/layout";
  import { changeFilters, openFilters, type FilterStore } from "../filterstate";
  import { addedNote, addStop, failedNote, followedNote, followRoute, type FollowNote, type Waypoint } from "../route";
  import {
    ageCell,
    ageLimit,
    DEFAULT_FILTERS,
    factCell,
    figure,
    groupHead,
    groups,
    isStale,
    jumpsFigure,
    jumpsUnit,
    panelFacts,
    placeHead,
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
    type Column,
    type Results,
    type Row,
  } from "../search";
  import { shipStatus } from "../core";
  import { depth, padTooSmall, shortTip, type Against } from "../demand";
  import type { ShipStatus } from "../api/types/ShipStatus";
  import Bars from "./Bars.svelte";
  import Button from "./Button.svelte";
  import Failure from "./Failure.svelte";
  import FollowNoteLine from "./FollowNote.svelte";
  import IconButton from "./IconButton.svelte";
  import OriginInput from "./OriginInput.svelte";
  import SearchFilters from "./SearchFilters.svelte";
  import StationKind from "./StationKind.svelte";
  import SystemChip from "./SystemChip.svelte";
  import { useDetail } from "./layout/listDetail.svelte";
  import { tip } from "./tip";
  import PadLetter from "./PadLetter.svelte";
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
    more,
    onanswer,
    measure = null,
    onship,
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
    /** The lookup's own filters (ring type, reserve), shown with the others when CHANGE opens them. */
    more?: Snippet;
    /**
     * Each answer as shown: the first row's ranked value and its unit, or null when nothing was
     * found. A Places finder keeps it for its tile.
     */
    onanswer?: (top: { figure: string; unit: string } | null) => void;
    /** What a `bars` fact's tonnes are measured against (Sell: the commodity aboard; Buy: the free space). */
    measure?: { side: "sell" | "buy"; against: Against } | null;
    /** The ship as read every 2 s (`ship_status`), for what the finder measures against. */
    onship?: (ship: ShipStatus | null) => void;
  } = $props();

  // Follows `start`; the commander's pick (or ×) overrides it until `start` changes.
  let origin = $derived<Origin>(start);
  /** The origin was picked as the route's next stop; kept here, since the box closes with the filters. */
  let viaNext = $state<string | null>(null);
  /** Filters loaded for `lookup` (null until they arrive). */
  let saved = $state<{ lookup: string; filters: Filters } | null>(null);
  /** The current ship's pad as of the last search (the core resolves it on every search). */
  let ship = $state<{ lookup: string; pad: Pad | null } | null>(null);
  /** The ship as last read: the pad it needs, for a pad too small on a row. */
  let flying = $state<ShipStatus | null>(null);
  let res = $state<Results | null>(null);
  /** The system the last answer was measured from, kept when a later search fails. */
  let searched = $state<string | null>(null);
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
  /** Where the action that wrote the note was: its row's list, or the side panel, so the note sits beside its button. */
  let noteAt = $state<"list" | "panel">("list");
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
        if (n !== asked) return;
        res = r;
        searched = r.origin;
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

  // The ship, read as the status bar reads it, every 2 s: what the finder measures its markets
  // against changes as the commander mines, sells or swaps ships, without a new search. A read
  // still waiting skips the next tick, so answers never overlap.
  let reading = false;
  $effect(() => {
    const read = () => {
      if (reading) return;
      reading = true;
      void shipStatus()
        .then((st) => {
          flying = st;
          onship?.(st);
        })
        .finally(() => (reading = false));
    };
    read();
    const t = setInterval(read, 2000);
    return () => clearInterval(t);
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
  async function add(r: Results["rows"][number], from: "list" | "panel" = "list") {
    noteAt = from;
    try {
      goNote = addedNote(await addStop(stop(r), "search"));
    } catch (e) {
      goNote = failedNote("Couldn't add it", e, () => void add(r, from));
    }
  }

  /** Set destination: a one-stop route to the result; the route it replaces is parked. */
  async function destination(r: Results["rows"][number], from: "list" | "panel" = "list") {
    noteAt = from;
    try {
      goNote = followedNote(await followRoute(r.system, "search", [stop(r)]));
    } catch (e) {
      goNote = failedNote("Couldn't follow it", e, () => void destination(r, from));
    }
  }

  const filters = $derived(saved?.filters ?? DEFAULT_FILTERS);
  /** The age filter in force: reports older than it are hidden. Shown reports older than STALE_DAYS are stepped down. */
  const limit = $derived(offers.includes("age") ? ageLimit(filters, hints) : null);
  /** Ages are measured when the answer arrives, so a list doesn't age while it is read. */
  const now = $derived(res ? Date.now() : 0);
  /** Staleness is judged as of the answer, as the source's own age filter was (a cached answer included). */
  const asOf = $derived(res ? Date.parse(res.fetched) || now : 0);
  /** With an age filter, reports older than it are hidden and counted. Sources apply the filter in the query, so this
   * counts only what came back past it; the summary names the limit either way. */
  const rows = $derived(res ? (limit == null ? res.rows : res.rows.filter((r) => !isStale(r, limit, asOf))) : []);
  const hidden = $derived(res ? res.rows.length - rows.length : 0);
  /** The ListDetail this list fills, if any: its rows can be selected, and the side panel takes the facts not chosen by. */
  const detail = useDetail();
  const g = $derived(res ? groups(res, rows, !!detail) : null);

  $effect(() => {
    if (!onanswer || busy || err || !res) return;
    const top = rows[0];
    if (!top) return onanswer(null);
    const figure = top.main ?? jumpsFigure(top);
    onanswer({ figure, unit: res.main ? (res.main.unit ?? "") : jumpsUnit(figure) });
  });

  /**
   * The width the list is given, to fall back to the compact row where the columns don't fit. Not
   * the kit's own width: the kit fits its rows, so once they were compact it would keep them so.
   * In a ListDetail the layout says (its track fits the rows too); elsewhere it is the width of
   * the element the kit sits in.
   */
  let kitEl = $state<HTMLElement | null>(null);
  let hostWidth = $state(0);
  $effect(() => {
    const host = kitEl?.parentElement;
    if (detail || !host || typeof ResizeObserver === "undefined") return;
    const ro = new ResizeObserver(([e]) => (hostWidth = e!.contentRect.width));
    ro.observe(host);
    return () => ro.disconnect();
  });
  const room = $derived(detail ? detail.room : at100(hostWidth, scale.step));
  /** The row's own group holds prose (wider) or only figures. */
  const ownText = $derived(!!g?.own.some((c) => c.text));
  /** Without a side panel the trip's second line also holds the report's age ("18.8 ly · 12 d old"). */
  const ageOnRow = $derived(!detail && !res?.undated);
  /** The groups' widths in px at 100 percent Text size (DESIGN.md Lookup result row): ranked value, place, own, trip, actions. */
  /** Where the trip line holds the age too ("1,426.2 ly · 2,070 d old"), its track is 168 px, else 136 or 96. */
  const widths = $derived(
    [ageOnRow && !g?.trip ? 168 : 136, 280, g?.own.length ? (ownText ? 160 : 112) : 0, g?.trip ? (ageOnRow ? 168 : 96) : 0, 64].filter(Boolean),
  );
  /** The narrowest the full row fits in, in px at 100 percent Text size (its groups, their gaps and its padding); below this, only the first lines stay. */
  const fits = $derived(widths.reduce((n, w) => n + w, 0) + 24 * (widths.length - 1) + 24);
  /**
   * Too narrow for the groups (a side panel): the first lines of the ranked value and the place,
   * and the actions, by The Aligned Columns Rule (a fact that would wrap goes).
   */
  const compact = $derived(room > 0 && room < fits);
  /**
   * Compact in a list with a side panel: one line, the panel holds the rest. Without a panel
   * (Powerplay's column, Nearby) the first two groups keep their second lines, so the system's
   * fact and the trip or what qualifies the value stay reachable.
   */
  const oneLine = $derived(compact && !!detail);
  /** Compact with no side panel: the own group and the trip fold into a third line, since nothing else would show them. */
  const folded = $derived(compact && !detail && !!(g?.own.length || g?.trip));
  const template = $derived(
    compact
      ? "max-content minmax(0, 1fr) 4rem"
      : [
          ageOnRow && !g?.trip ? "10.5rem" : "8.5rem",
          "minmax(17.5rem, 25rem)",
          ...(g?.own.length ? [ownText ? "minmax(10rem, 15rem)" : "7rem"] : []),
          ...(g?.trip ? [ageOnRow ? "10.5rem" : "6rem"] : []),
          "4rem",
        ].join(" "),
  );
  /** The ranked value's group head: "Price · demand", or "Trip" where jumps rank the list. */
  const firstHead = $derived(res?.main ? groupHead([res.main.head, ...(g?.qual ? [g.qual.head] : [])]) : "Trip");
  /**
   * Where the search ran from: the answer's system once it is in; while a search runs or after
   * it failed, the system typed for it (else the last answer's), so a new origin shows at once.
   */
  const where = $derived(res && !busy ? res.origin : (origin ?? searched ?? "here"));
  /** The answer the summary may describe: none while a search runs or after one failed. */
  const shownAnswer = $derived(busy || err ? null : res);
  /**
   * The filters' summary line. It follows each edit at once. The radius an answer reached, the
   * hidden count and the source's notes describe that answer, so they show only once it is in:
   * while the next search runs, the line names the filters it asks with.
   */
  const line = $derived(
    summary(`${title} ${joiner}`, where, filters, offers, hints, shownAnswer ? hidden : 0, said([...notes, ...(shownAnswer?.notes ?? []), ...(shownAnswer ? (g?.same ?? []) : [])]), shownAnswer?.reach ?? null),
  );
  /** The one constraint an empty answer can relax, and how. */
  const widen = $derived(widening(filters, offers, hints));

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


  /** Notes without one already said (a ring type the filters name and every row shares). */
  function said(list: string[]): string[] {
    return list.filter((n, i) => !list.slice(0, i).some((m) => m.toLowerCase() === n.toLowerCase()));
  }

  /** A stale row: a report older than STALE_DAYS as of the answer. */
  const stale = (r: Row) => isStale(r, STALE_DAYS, asOf);

  /** The empty line's action: relax the one constraint it names. */
  function relax() {
    if (widen) void change(widen.filters);
  }
</script>

{#snippet name(r: Results["rows"][number], text: string)}
  {#if detail}
    <button class="name pick" aria-label={text} {@attach tip(text)} aria-pressed={chosen?.key === r.key} onclick={() => (picked = r.key)}>{text}</button>
  {:else}
    <span class="name" title={text}>{text}</span>
  {/if}
{/snippet}

<!-- A fact of the lookup's own: a figure with its unit, prose truncated, or a market's tonnes after
     its demand bars, coloured by what they mean for the hold (DESIGN.md Colours that carry a
     meaning). A short market says what happens in a Tip; its bar count and words are its second cue. -->
{#snippet fact(c: Column, r: Row)}
  {@const cell = factCell(c, r)}
  {#if c.bars && r.volume != null}
    {@const d = depth(r.volume, measure?.against.tonnes ?? null)}
    {@const said = `${c.head} ${cell.value} t${d && measure ? `, ${d === "short" ? "short of" : `${d} for`} the ${measure.against.text}` : ""}`}
    <span class="tonnes {d ?? 'plain'}" role="img" aria-label={said} {@attach tip(d === "short" && measure ? shortTip(measure.side, r.volume, measure.against) : null)}>
      <Bars depth={d} /><span>{cell.value}<span class="unit type-unit">t</span></span>
    </span>
  {:else if cell.text}
    {@const note = r.notes?.[c.key]}
    <span class="text" title={note ? undefined : cell.value} {@attach tip(note)}>{cell.value}</span>
  {:else}
    <span class:none={cell.value === "—"}>{cell.value}{#if cell.unit}<span class="unit type-unit">{cell.unit}</span>{/if}</span>
  {/if}
{/snippet}

<!-- The trip's second line: light years, or a stale report's age in their place. Without a side
     panel the age stays at any age, grey when fresh. -->
{#snippet tripLine(r: Row)}
  {@const age = ageCell(r.updated_at, now)}
  {#if ageOnRow && r.updated_at}
    <span class="l2"><span class="ly">{figure(r.ly, 1)}<span class="unit type-unit">ly</span></span>&nbsp;·&nbsp;<span class="age" class:old={stale(r)}>{age.value} {age.unit} old</span></span>
  {:else if stale(r)}
    <span class="l2 age old">{age.value} {age.unit} old</span>
  {:else}
    <span class="l2 ly">{figure(r.ly, 1)}<span class="unit type-unit">ly</span></span>
  {/if}
{/snippet}

<!-- DESIGN.md Side panel: the selected result's name in white, where it is, its ranked value and
     what qualifies it, every fact under its name (the row leaves some out), and its two actions as words. -->
{#snippet about()}
  {#if chosen && res}
    {@const r = chosen}
    {@const top = r.main ?? jumpsFigure(r)}
    <h2 class="ptitle">{r.station ?? r.system}</h2>
    <p class="psub">
      {#if r.station}<StationKind type={r.station_type} /><span>{r.station_type ? `${r.station_type} in` : "In"}</span> <SystemChip name={r.system} />{:else}<SystemChip name={r.system} icon />{/if}
    </p>
    <p class="pmain"><span class="v">{top}</span><span class="unit type-unit">{res.main ? (res.main.unit ?? "") : jumpsUnit(top)}</span></p>
    {#if g?.qual}<p class="pqual">{@render fact(g.qual, r)}<span class="unit type-unit">{g.qual.head.toLowerCase()}</span></p>{/if}
    <dl class="pfacts">
      {#each panelFacts(res, r, now).filter((f) => f.head !== g?.qual?.head) as f (f.head)}
        <dt>{f.head}</dt>
        <dd class:none={f.cell.value === "—"}>{f.cell.value}{#if f.cell.unit}<span class="unit type-unit">{f.cell.unit}</span>{/if}</dd>
      {/each}
    </dl>
    <div class="pacts">
      <Button kind="primary" onclick={() => void destination(r, "panel")}>Set destination</Button>
      <Button onclick={() => void add(r, "panel")}>Add stop</Button>
    </div>
    {#if noteAt === "panel"}<FollowNoteLine bind:note={goNote} />{/if}
  {/if}
{/snippet}

{#snippet from()}
  <OriginInput {origin} {around} bind:viaNext onchange={(o) => (origin = o)} />
{/snippet}

<div class="kit" bind:this={kitEl}>
  {#if header || lead}
    <div class="query">
      {#if header}{@render header()}{/if}
      {#if lead}{@render lead()}{/if}
    </div>
  {/if}

  <div class="out">
    {#if saved}
      <div class="constraints">
        <SearchFilters {filters} shipPad={ship?.lookup === lookup ? ship.pad : undefined} {offers} {hints} summary={line} extra={more} origin={from} onchange={(f) => void change(f)} />
        {#if filterErr && unsaved}<Failure text={filterErr} onaction={retrySave} />{/if}
      </div>
    {/if}
    {#if err}
      <Failure text={err} onaction={() => attempt++} />
    {:else if busy}
      <p class="line">{busyText}</p>
    {/if}
    {#if res && !err}
      {#if res.warning}<p class="line warn"><b>!</b> {res.warning}</p>{/if}
      {#if rows.length}
        <div class="list" class:busy class:compact={oneLine} role="table" aria-label="Results" aria-busy={busy} style:--cols={template}>
          <div class="row heads" role="row">
            <span class="r" role="columnheader">{firstHead}</span>
            <span role="columnheader">{placeHead(res.place, oneLine ? null : (g?.under ?? null))}</span>
            {#if !compact && g?.own.length}<span class:r={!ownText} role="columnheader">{groupHead(g.own.map((c) => c.head))}</span>{/if}
            {#if !compact && g?.trip}<span class="r" role="columnheader">Trip</span>{/if}
            <span role="columnheader" aria-label="Actions"></span>
          </div>
          {#each rows as r (r.key)}
            {@const top = r.main ?? jumpsFigure(r)}
            {@const unfit = padTooSmall(r.pad, flying?.pad)}
            <!-- The keyboard selects a row with its name, a button; a click anywhere on the row does too. -->
            <!-- svelte-ignore a11y_interactive_supports_focus, a11y_click_events_have_key_events -->
            <div class="row result" class:stale={stale(r)} class:sel={chosen?.key === r.key} class:tall={folded} role="row" onclick={() => detail && (picked = r.key)}>
              <span class="g r first" role="cell">
                <span class="l1 main"><span class="v">{top}</span><span class="unit type-unit">{res.main ? (res.main.unit ?? "") : jumpsUnit(top)}</span></span>
                {#if !oneLine}
                  {#if g?.trip}
                    {#if g.qual}<span class="l2">{@render fact(g.qual, r)}</span>{/if}
                  {:else}
                    {@render tripLine(r)}
                  {/if}
                {/if}
              </span>
              <span class="g place" role="cell">
                <span class="l1">
                  {#if r.station}
                    <!-- A station list keeps the type icon's slot when the source names no type, so names line up. -->
                    {#if res.place === "station"}<span class="slot"><StationKind type={r.station_type} /></span>{:else}<StationKind type={r.station_type} />{/if}
                    {@render name(r, r.station)}
                    {#if res.place === "station" && r.pad}
                      <PadLetter pad={r.pad} {unfit} />
                    {/if}
                    {#if r.arrival_ls != null}<span class="ls">{figure(r.arrival_ls)}<span class="unit type-unit">ls</span></span>{/if}
                    {#if oneLine}<span class="sys" title={r.system}>{r.system}</span>{/if}
                  {:else}
                    {@render name(r, r.system)}
                  {/if}
                  {#if oneLine || !r.station}<SystemChip name={r.system} icon />{/if}
                </span>
                {#if !oneLine}
                  {#if r.station}
                    <span class="l2 in"><span class="sys" title={r.system}>{r.system}</span><SystemChip name={r.system} icon /></span>
                  {:else if g?.under}
                    <span class="l2 in">{@render fact(g.under, r)}</span>
                  {/if}
                {/if}
              </span>
              {#if !compact && g?.own.length}
                <span class="g own" class:r={!ownText} role="cell">
                  {#each g.own.slice(0, 2) as c, i (c.key)}
                    <span class={i ? "l2" : "l1"}>{@render fact(c, r)}{#if i === 1}{#each g.own.slice(2) as more (more.key)}&nbsp;·&nbsp;{@render fact(more, r)}{/each}{/if}</span>
                  {/each}
                </span>
              {/if}
              {#if !compact && g?.trip}
                <span class="g r trip" role="cell">
                  <span class="l1">{jumpsFigure(r)}<span class="unit type-unit">{jumpsUnit(jumpsFigure(r))}</span></span>
                  {@render tripLine(r)}
                </span>
              {/if}
              <span class="acts" role="cell">
                <IconButton icon="add-stop" label={`Add ${r.system} as a stop`} tip={`Add ${r.system} as the next stop`} onclick={() => void add(r)} />
                <IconButton icon="destination" label={`Set ${r.system} as the destination`} tip={`Set ${r.system} as the destination (replaces the route, which Resume brings back)`} onclick={() => void destination(r)} />
              </span>
              {#if folded && g}
                <!-- Compact with no side panel: the groups that don't fit fold into a third line under the place. -->
                <!-- Only the own group's prose gives way on a narrow line; the trip keeps its width. -->
                <span class="folded" role="cell">
                  {#if g.own.length}<span class="fown">{#each g.own as c, i (c.key)}{#if i}&nbsp;·&nbsp;{/if}{@render fact(c, r)}{/each}</span>{/if}
                  {#if g.trip}<span class="ftrip">
                    {#if g.own.length}&nbsp;·&nbsp;{/if}{jumpsFigure(r)}<span class="unit type-unit">{jumpsUnit(jumpsFigure(r))}</span>&nbsp;·&nbsp;<span class="ly">{figure(r.ly, 1)}<span class="unit type-unit">ly</span></span>
                    {#if !res.undated && r.updated_at}&nbsp;·&nbsp;<span class="age" class:old={stale(r)}>{ageCell(r.updated_at, now).value} {ageCell(r.updated_at, now).unit} old</span>{/if}
                  </span>{/if}
                </span>
              {/if}
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
      {#if noteAt === "list"}<FollowNoteLine bind:note={goNote} />{/if}
      <p class="line foot">At {figure(res.range, 1)} ly per jump · {res.source}, {ago(res.fetched)}{res.rows.some((r) => !r.exact) ? " · ~ = straight-line estimate" : ""}</p>
    {/if}
  </div>
</div>

<style>
  /* As wide as the query's controls or the list's columns, whichever is wider: spare width stays
     empty (DESIGN.md The Next To It Rule). */
  .kit { display: flex; flex-direction: column; gap: 0; min-width: 0; width: fit-content; max-width: 100%; }
  .query { display: flex; flex-direction: column; gap: var(--sp-3); padding: var(--sp-4); border-bottom: 1px solid var(--line); }
  .out { padding: var(--sp-6) 0 var(--sp-2); min-width: 0; }
  p { margin: 0; }
  .constraints { display: flex; flex-direction: column; gap: var(--sp-2); margin: 0 0 var(--sp-3); }

  /* Label lines: the summary above the heads, loading, a warning and the source line below. */
  .line { margin: 0 0 var(--sp-3); font-size: var(--fs-s); color: var(--label-grey); }
  .warn { color: var(--accent); font-weight: 500; }
  .warn b { font-weight: 600; }
  .foot { margin: var(--sp-3) 0 0; }
  .empty { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-2); min-height: var(--row); color: var(--prose); }

  /* DESIGN.md Lookup result row: two lines in groups, 60 px, every row on the list's group tracks. */
  /* A row is as wide as its groups, not as the window (DESIGN.md The Next To It Rule). */
  .list { display: grid; grid-template-columns: var(--cols); column-gap: var(--sp-6); width: fit-content; max-width: 100%; }
  .row { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: center; height: var(--row-result); padding: 0 var(--sp-3); white-space: nowrap; color: var(--prose); font-size: var(--fs); }
  .compact .row { height: var(--row); }
  .row.tall { height: auto; min-height: var(--row-result); padding-block: var(--sp-2); }
  .folded { grid-column: 2 / -1; display: flex; align-items: baseline; overflow: hidden; font-size: var(--fs-s); white-space: nowrap; }
  /* The own group's prose shrinks and ellipsizes; the trip's figures, units and age don't. */
  .fown { display: flex; align-items: baseline; min-width: 0; overflow: hidden; }
  .fown > :global(*) { min-width: 0; }
  .ftrip { display: flex; align-items: baseline; flex: none; }
  .row > * { min-width: 0; }
  .heads { height: 1.75rem; border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); color: var(--label-grey); font-size: var(--fs-s); overflow: hidden; }
  .result:hover { background: var(--row-hover); }
  /* Icon buttons on a hovered or selected row step up to Orange Mid; the one under the pointer is Orange Lit. */
  .result:hover :global(.ibtn) { color: var(--orange-mid); }
  .result :global(.ibtn:hover) { color: var(--orange-lit); }
  .r { text-align: right; }
  /* A group: two lines, each truncating before it wraps; figures right-aligned, text left. */
  .g { display: flex; flex-direction: column; justify-content: center; overflow: hidden; }
  .g.r { align-items: flex-end; }
  .l1, .l2 { display: flex; align-items: baseline; min-width: 0; max-width: 100%; overflow: hidden; }
  .place .l1 { align-items: center; gap: var(--sp-2); }
  .l2 { font-size: var(--fs-s); }
  /* The value the list is ranked by is its one orange figure (DESIGN.md The Orange Means Act Rule). */
  .main .v { color: var(--accent); font-size: var(--fs-l); font-weight: 600; line-height: 1.2; }
  /* .unit is the Unit role (.type-unit); the class stays for the state rules below. */
  .place :global(.kind) { color: var(--label-grey); flex: none; }
  .name { color: var(--options-grey); font-weight: 500; overflow: hidden; text-overflow: ellipsis; flex: 0 1 auto; }
  .pick { all: unset; cursor: pointer; color: var(--options-grey); font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 0 1 auto; }
  .pick:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .result:has(.pick) { cursor: pointer; }
  .ls { flex: none; font-size: var(--fs-s); color: var(--prose); }
  /* The system under the place, indented to the name past the station's type icon. */
  .in { padding-left: var(--sp-6); gap: 0; }
  .place .l1:not(:has(.slot)) + .in { padding-left: 0; }
  .slot { display: inline-flex; flex: none; width: var(--sp-4); }
  .sys { color: var(--label-grey); font-size: var(--fs-s); overflow: hidden; text-overflow: ellipsis; flex: 0 1000 auto; }
  .place :global(.chip) { flex: none; }
  /* The copy icon keeps the row's two lines even: as tall as a line, not a 28 px button. */
  .place :global(.chip .ibtn) { height: 1.25rem; }
  .text { overflow: hidden; text-overflow: ellipsis; }
  .own .l2 .text { color: var(--label-grey); }
  .none { color: var(--grey-dim); }
  .ly { color: var(--label-grey); }
  .age { color: var(--label-grey); }
  /* A market's tonnes after its bars, coloured by what they mean for the hold. */
  .tonnes { display: inline-flex; align-items: center; gap: var(--sp-1); }
  .tonnes.short { color: var(--red-low); }
  .tonnes.tight { color: var(--orange-mid); }
  .tonnes:is(.plenty, .plain) { color: var(--prose); }
  .acts { display: flex; justify-content: flex-end; gap: var(--sp-1); }

  /* Stale: the ranked value steps down to Orange Mid and the age, now the brightest fact on the
     row, is Orange. */
  .stale .main .v { color: var(--orange-mid); }
  .age.old, .age .old { color: var(--accent); }
  .age.old { font-weight: 400; }
  /* Selected, while the side panel shows it (DESIGN.md States): Hull Raised, the ranked value
     Orange Lit, the name White, other text one step brighter, icon buttons Orange Mid. */
  .result.sel, .result.sel:hover { background: var(--surface-hi); }
  .result.sel .main .v { color: var(--orange-lit); }
  .result.sel.stale .main .v { color: var(--orange-mid); }
  .result.sel :is(.name, .pick) { color: var(--text-hi); }
  .result.sel :is(.unit, .sys, .ly, .age:not(.old)), .result.sel :global(.kind) { color: var(--prose); }
  .result.sel :global(.ibtn) { color: var(--orange-mid); }
  .result.sel :global(.ibtn:hover) { color: var(--orange-lit); }

  /* The side panel. */
  .ptitle { margin: 0; color: var(--text-hi); font-size: var(--fs-xl); font-weight: 600; line-height: 1.2; overflow-wrap: anywhere; }
  .psub { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-1) var(--sp-2); margin: var(--sp-1) 0 0; font-size: var(--fs-s); color: var(--prose); }
  .pmain { margin: var(--sp-6) 0 0; }
  /* The panel's one main value: Value XL, Orange Lit. */
  .pmain .v { color: var(--orange-lit); font-size: var(--fs-xxl); font-weight: 500; }
  .pqual { display: flex; align-items: center; gap: var(--sp-1); margin: var(--sp-2) 0 0; }
  .pfacts { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: var(--sp-2) var(--sp-6); margin: var(--sp-6) 0 0; }
  .pfacts dt { color: var(--label-grey); font-size: var(--fs-s); align-self: baseline; }
  .pfacts dd { margin: 0; overflow-wrap: anywhere; }
  .pacts { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-4); margin: var(--sp-8) 0 0; }

  /* Loading: the old rows stay, Grey Dim, until the answer replaces them. */
  .busy .result, .busy .result :is(.main .v, .name, .pick, .text, .sys, .unit, .ly, .age, .tonnes, .ls, :global(.pad)) { color: var(--grey-dim); }
</style>
