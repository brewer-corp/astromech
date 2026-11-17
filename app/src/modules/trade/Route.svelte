<script lang="ts">
  // A trade route from here (or a typed system): the best single hop, or a loop that brings the
  // best cargo back, for the hold, credits, pad and hop distance. The numbers start from the
  // current ship and the journal and can be changed; the filters are remembered as
  // `trade.route`, and rest as their summary line until CHANGE opens them; an edit changes what
  // Find route will ask at once. Spansh works the route out as a job, so it runs only on "Find
  // route", which leaves the filters open or closed as they were.
  // DESIGN.md's Workbench: the question on the left, under the page's own lead, and the answer
  // on the right.
  import { tip } from "../../lib/ui/tip";
  import Failure from "../../lib/ui/Failure.svelte";
  import Button from "../../lib/ui/Button.svelte";
  import Choice from "../../lib/ui/Choice.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import { changeFilters, openFilters, type FilterStore } from "../../lib/filterstate";
  import { ago, jumps } from "../../lib/galaxy";
  import { arrivalLabel, DEFAULT_FILTERS, filterParts, loadFilters, saveFilters, type FilterKey, type Filters, type Origin, type Pad } from "../../lib/search";
  import { credits, DEFAULT_MAX_AGE_DAYS, ROUTE_ESTIMATE, tradeRoute, tradeRouteDefaults, type RouteKind, type RouteLeg, type RouteSearch, type TradeRoute } from "../../lib/trade";
  import { padTooSmall } from "../../lib/demand";
  import PadLetter from "../../lib/ui/PadLetter.svelte";
  import OriginInput from "../../lib/ui/OriginInput.svelte";
  import SearchFilters from "../../lib/ui/SearchFilters.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import { failure } from "../../lib/ui/errors";
  import Workbench from "../../lib/ui/layout/Workbench.svelte";
  import type { Snippet } from "svelte";

  let { lead }: { /** Above the inputs: where else to look. */ lead?: Snippet } = $props();

  const LOOKUP = "trade.route";
  /** Used when the journal has no Loadout yet. */
  const FALLBACK_HOP_LY = 20;

  let kind = $state<RouteKind>("single");
  let origin = $state<Origin>(null);
  let station = $state("");
  let docked = $state<string | null>(null);
  let capacity = $state<number | null>(null);
  let cash = $state<number | null>(null);
  let hop = $state<number | null>(null);
  let filters = $state<Filters | null>(null);
  /** The current ship's pad as of the last search (the core resolves it on every search). */
  let shipPad = $state<Pad | null | undefined>(undefined);
  let filterErr = $state<string | null>(null);
  /** The ship's cargo, credits and range could not be read: the fields start empty and the hop at the default. */
  let defaultsErr = $state<string | null>(null);
  let res = $state<RouteSearch | null>(null);
  /** The pad the answer was asked for (the filters' pad: the ship's unless the commander set one),
   * as the core's `warning` measures it; kept with the answer, since the filters change on edit. */
  let need = $state<Pad | null>(null);
  let err = $state<string | null>(null);
  let busy = $state(false);
  let asked = 0;
  const store: FilterStore = { load: loadFilters, save: saveFilters };

  $effect(() => {
    void tradeRouteDefaults()
      .then((d) => {
        capacity ??= d.capacity;
        cash ??= d.credits;
        hop ??= d.range != null ? Math.round(d.range * 10) / 10 : FALLBACK_HOP_LY;
        docked = d.station;
      })
      .catch((e: unknown) => {
        hop ??= FALLBACK_HOP_LY;
        defaultsErr = `${failure("read your ship and credits", e)} Enter cargo, credits and max hop yourself.`;
      });
    void openFilters(store, LOOKUP, DEFAULT_FILTERS).then((st) => (filters = st.filters));
  });

  async function changeFilter(next: Filters) {
    const prev = filters ?? DEFAULT_FILTERS;
    filters = next;
    const st = await changeFilters(store, LOOKUP, prev, next);
    filterErr = st.error;
    filters = st.filters;
  }

  /** The pad label follows the ship the search ran with, not the ship the filters loaded with. */
  async function followShip() {
    try {
      const f = await store.load(LOOKUP);
      if (!f.pad_set) shipPad = f.pad;
    } catch {
      // the label keeps what it shows; the search does not depend on this
    }
  }

  /** The station to start from: typed, else the docked one when starting from here. */
  const start = $derived(station.trim() || (origin == null ? docked : null));
  const ready = $derived(!!capacity && capacity > 0 && !!cash && cash > 0 && !!hop && hop > 0 && !!filters);

  async function find() {
    if (!ready || !filters || capacity == null || cash == null || hop == null) return;
    const n = ++asked;
    busy = true;
    err = null;
    void followShip();
    try {
      const ask_pad = filters.pad;
      const ask = { kind, station: start, capacity: Math.floor(capacity), credits: Math.floor(cash), max_hop_ly: hop };
      const r = await tradeRoute(ask, origin, filters);
      if (n === asked) {
        res = r;
        need = ask_pad;
      }
    } catch (e) {
      if (n === asked) {
        res = null;
        err = String(e);
      }
    } finally {
      if (n === asked) busy = false;
    }
  }

  const OFFERS = ["pad", "age", "arrival", "carriers"] as const satisfies FilterKey[];
  const HINTS = { age: DEFAULT_MAX_AGE_DAYS };
  /** The filters as one line: "Large pad · reports up to 30 days old". */
  const line = $derived.by(() => {
    const s = filters ? filterParts(filters, [...OFFERS], HINTS).join(" · ") : "";
    return s.charAt(0).toUpperCase() + s.slice(1);
  });

  const num = (v: string): number | null => (v.trim() === "" ? null : Number(v));
  const minutes = (m: number) => (m < 1 ? "under a minute" : `${Math.round(m)} min`);
  /** The per-hour figure's Tip: how it is estimated, which Spansh does not give. */
  const estimate = (r: TradeRoute) => ({
    title: "Estimated: Spansh gives no profit per hour",
    fact: `${ROUTE_ESTIMATE}, at ${r.range.toFixed(1)} ly a jump`,
    ...(r.kind === "single" ? { note: "A round counts the flight back empty, so it compares with a loop." } : {}),
  });
  /** Where the markets came from, once each: "Spansh and Ardent Insight". */
  const sources = (legs: RouteLeg[]) => [...new Set(legs.map((l) => l.source))].join(" and ");
</script>

<Workbench min={720}>
  {#snippet inputs()}
  <div class="query">
    {#if lead}{@render lead()}{/if}
    <div class="form-grid">
      <OriginInput {origin} onchange={(o) => (origin = o)} />
      <Choice
        class="kinds"
        label="Route kind"
        value={kind}
        options={[
          { value: "single", label: "Single hop", tip: "The most profitable hop from a station" },
          { value: "loop", label: "Loop", tip: "A loop that brings cargo back" },
        ]}
        onchange={(v) => (kind = v)}
      />
      <div class="pick">
        <span class="type-label lbl">Station</span>
        <Input class="box" aria-label="Start station" placeholder={origin == null && docked ? docked : "Nearest the star"} bind:value={station} />
      </div>
      <div class="fields">
        <label><span class="type-label">Cargo</span><Input aria-label="Cargo capacity" type="number" min="1" value={capacity ?? ""} oninput={(e) => (capacity = num(e.currentTarget.value))}>{#snippet end()}t{/snippet}</Input></label>
        <label><span class="type-label">Credits</span><Input aria-label="Credits" type="number" min="1" value={cash ?? ""} oninput={(e) => (cash = num(e.currentTarget.value))}>{#snippet end()}cr{/snippet}</Input></label>
        <label><span class="type-label">Max hop</span><Input aria-label="Max hop distance" type="number" min="1" step="0.1" value={hop ?? ""} oninput={(e) => (hop = num(e.currentTarget.value))}>{#snippet end()}ly{/snippet}</Input></label>
      </div>
    </div>
    <div class="tools">
      {#if filters}<SearchFilters {filters} {shipPad} offers={[...OFFERS]} hints={HINTS} summary={line} onchange={(f) => void changeFilter(f)} />{/if}
      <Button kind="primary" class="go" disabled={!ready || busy} onclick={() => void find()}>{busy ? "Finding…" : "Find route"}</Button>
    </div>
    {#if defaultsErr}<p class="warn">{defaultsErr}</p>{/if}
    {#if filterErr}<p class="warn">{filterErr}</p>{/if}
  </div>
  {/snippet}

  <div class="out">
    {#if busy}
      <p class="type-label">Asking Spansh… a route can take a minute.</p>
    {:else if err}
      <Failure text={err} onaction={() => void find()} />
    {:else if res}
      {@const r = res.route}
      {#if r.legs.length === 0}
        <p class="type-body">No profitable route within these limits. Try a longer hop, more credits or older market data.</p>
      {:else}
        {@const stops = [r.legs[0]!.from, ...r.legs.map((l) => l.to)]}
        <div class="totals">
          <p class="hour"><span class="type-value-xl main" {@attach tip(estimate(r))}>{credits(r.per_hour)}</span> <span class="type-label">cr an hour, estimated</span></p>
          <p class="type-label">{credits(r.profit)} cr {r.kind === "loop" ? "a round" : "a hop"} · {minutes(r.minutes)} {r.kind === "loop" ? "a round" : "there and back"}</p>
          <p class="type-label">{sources(r.legs)}, {ago(new Date(r.fetched_at * 1000).toISOString())}</p>
        </div>
        {#if r.warning}<p class="warn" role="alert">{r.warning}</p>{/if}
        <ol class="trip" aria-label="Stops and the legs between them">
          {#each stops as s, i (i)}
            {@const unfit = padTooSmall(s.pad, need)}
            <li class="stop" class:first={i === 0} class:last={i === stops.length - 1}>
              <span class="sp"><span class="node"></span></span>
              <div class="place">
                <span class="type-name name">{s.station}</span>
                <span class="where">
                  <span class="type-label sys">{s.system}</span><SystemChip name={s.system} icon />
                  {#if s.pad}<PadLetter pad={s.pad} {unfit} />{/if}
                  {#if s.arrival_ls != null}<span class="type-label">{arrivalLabel(s.arrival_ls)}</span>{/if}
                </span>
              </div>
            </li>
            {#if i < r.legs.length}
              {@const l = r.legs[i]!}
              <li class="leg">
                <span class="sp"></span>
                <div class="cargo">
                  {#each l.cargo as c (c.name)}
                    <p><span class="type-figures">{credits(c.tonnes)} t {c.name}</span> <span class="type-label">buy {credits(c.buy)} · sell {credits(c.sell)}</span></p>
                  {:else}
                    <p class="type-label">Fly back empty</p>
                  {/each}
                </div>
                <div class="figs">
                  {#if l.cargo.length}
                    <span class="type-value-l">{credits(l.profit)}<span class="type-unit">cr</span></span>
                    <span class="type-label">{credits(l.profit_per_t)} cr/t</span>
                  {/if}
                  <span class="type-label">{jumps(l.jumps, false)} · {l.ly.toFixed(1)} ly</span>
                </div>
              </li>
            {/if}
          {/each}
        </ol>
      {/if}
    {:else}
      <p class="type-label">Press Find route to see the {kind === "loop" ? "loop" : "hop"}. Spansh can take a minute.</p>
    {/if}
  </div>
</Workbench>

<style>
  /* The inputs: one column of label and cell, the kind first, Find route last. */
  .query { display: flex; flex-direction: column; gap: var(--sp-3); }
  .query :global(.origin) { margin: 0; min-width: 0; }
  .form-grid { display: flex; flex-direction: column; gap: var(--sp-3); }
  .pick { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; }
  .lbl { min-width: 4.5rem; }
  .pick :global(.box) { flex: 1; }
  .fields { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--sp-3); min-width: 0; }
  label { display: grid; grid-template-columns: 4.5rem minmax(0, 1fr); align-items: center; gap: var(--sp-2); min-width: 0; }
  .tools { display: flex; flex-wrap: wrap; align-items: flex-start; gap: var(--sp-3) var(--sp-4); }
  .tools :global(.search-filters) { flex: 1 1 auto; }
  .tools :global(.go) { margin-left: auto; }
  .warn { color: var(--accent); font-weight: 500; margin: 0 0 var(--sp-3); }
  .warn::before { content: "! "; font-weight: 600; }
  p { margin: 0; }

  /* The answer: the route's totals, then its stops joined by legs, top to bottom. */
  .out { min-width: 0; }
  .totals { display: flex; flex-direction: column; gap: var(--sp-1); margin-bottom: var(--sp-6); }
  .hour { display: flex; align-items: baseline; flex-wrap: wrap; gap: var(--sp-2); margin-bottom: var(--sp-1); }
  /* Two columns shared by every row (subgrid): the spine with a node at each stop, and the stop
     or the leg; a leg's figures are its own right-aligned column (The Tabular Rule). */
  .trip { display: grid; grid-template-columns: var(--sp-6) minmax(0, 24rem) auto; justify-content: start; column-gap: var(--sp-3); margin: 0; padding: 0; list-style: none; }
  .trip > li { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; }
  .sp { position: relative; display: grid; place-items: center; }
  .sp::before { content: ""; position: absolute; top: 0; bottom: 0; left: calc(50% - 1px); border-left: 2px solid var(--line); }
  .first .sp::before { top: 50%; }
  .last .sp::before { bottom: 50%; }
  .node { position: relative; padding: var(--sp-1); border: 2px solid var(--label-grey); border-radius: 50%; background: var(--bg); }
  .place { grid-column: 2 / -1; display: flex; flex-direction: column; gap: var(--gap); min-width: 0; padding: var(--sp-2) 0; }
  .name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .first .name { color: var(--text-hi); }
  .where { display: flex; align-items: center; flex-wrap: wrap; gap: var(--sp-2); min-width: 0; }
  .where :global(.chip) { margin-left: calc(-1 * var(--sp-2)); }
  .cargo { display: flex; flex-direction: column; gap: var(--sp-1); min-width: 0; padding: var(--sp-3) 0 var(--sp-4); }
  .figs { display: flex; flex-direction: column; align-items: flex-end; gap: var(--gap); text-align: right; padding: var(--sp-3) 0 var(--sp-4) var(--sp-6); }
</style>
