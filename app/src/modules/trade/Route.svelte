<script lang="ts">
  // A trade route from here (or a typed system): the best single hop, or a loop that brings the
  // best cargo back, for the hold, credits, pad and hop distance. The numbers start from the
  // current ship and the journal and can be changed; the filters are remembered as
  // `trade.route`. Spansh works the route out as a job, so it runs only on "Find route".
  import { changeFilters, openFilters, type FilterStore } from "../../lib/filterstate";
  import { ago, jumps } from "../../lib/galaxy";
  import { arrivalLabel, DEFAULT_FILTERS, loadFilters, padLabel, saveFilters, type Filters, type Origin, type Pad } from "../../lib/search";
  import { credits, DEFAULT_MAX_AGE_DAYS, ROUTE_ESTIMATE, tradeRoute, tradeRouteDefaults, type RouteKind, type RouteSearch } from "../../lib/trade";
  import OriginInput from "../../lib/ui/OriginInput.svelte";
  import SearchFilters from "../../lib/ui/SearchFilters.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";

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
  /** The current ship's pad as of the last search (the shell resolves it on every search). */
  let shipPad = $state<Pad | null | undefined>(undefined);
  let filterErr = $state<string | null>(null);
  let res = $state<RouteSearch | null>(null);
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
      .catch(() => {
        hop ??= FALLBACK_HOP_LY;
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
      const ask = { kind, station: start, capacity: Math.floor(capacity), credits: Math.floor(cash), max_hop_ly: hop };
      const r = await tradeRoute(ask, origin, filters);
      if (n === asked) res = r;
    } catch (e) {
      if (n === asked) {
        res = null;
        err = String(e);
      }
    } finally {
      if (n === asked) busy = false;
    }
  }

  const num = (v: string): number | null => (v.trim() === "" ? null : Number(v));
  const minutes = (m: number) => (m < 1 ? "under a minute" : `${Math.round(m)} min`);
</script>

<div class="route">
  <div class="kinds" role="group" aria-label="Route kind">
    <button class:on={kind === "single"} aria-pressed={kind === "single"} onclick={() => (kind = "single")}>Single hop</button>
    <button class:on={kind === "loop"} aria-pressed={kind === "loop"} onclick={() => (kind = "loop")}>Loop</button>
  </div>
  <OriginInput {origin} onchange={(o) => (origin = o)} />
  <div class="fields">
    <label><span>Station</span><input aria-label="Start station" placeholder={origin == null && docked ? docked : "Nearest the star"} bind:value={station} /></label>
    <label><span>Cargo</span><input aria-label="Cargo capacity" type="number" min="1" value={capacity ?? ""} oninput={(e) => (capacity = num(e.currentTarget.value))} /><em>t</em></label>
    <label><span>Credits</span><input aria-label="Credits" type="number" min="1" value={cash ?? ""} oninput={(e) => (cash = num(e.currentTarget.value))} /><em>cr</em></label>
    <label><span>Max hop</span><input aria-label="Max hop distance" type="number" min="1" step="0.1" value={hop ?? ""} oninput={(e) => (hop = num(e.currentTarget.value))} /><em>ly</em></label>
  </div>
  {#if filters}<SearchFilters {filters} {shipPad} offers={["pad", "age", "arrival", "carriers"]} hints={{ age: DEFAULT_MAX_AGE_DAYS }} onchange={(f) => void changeFilter(f)} />{/if}
  {#if filterErr}<p class="lo">{filterErr}</p>{/if}
  <button class="go" disabled={!ready || busy} onclick={() => void find()}>{busy ? "Finding…" : "Find route"}</button>

  {#if busy}
    <p class="lo">Asking Spansh… a route can take a minute.</p>
  {:else if err}
    <p class="lo err">{err}</p>
  {:else if res}
    <h4>{res.route.kind === "loop" ? "Loop" : "Single hop"} from&nbsp;<SystemChip name={res.here.system} /></h4>
    {#if res.route.warning}<p class="warn" role="alert">{res.route.warning}</p>{/if}
    {#if res.route.legs.length === 0}
      <p class="lo">No profitable route within these limits. Try a longer hop, more credits or older market data.</p>
    {:else}
      <ol>
        {#each res.route.legs as l, i (i)}
          <li>
            <span class="sys">{l.from.station} · <SystemChip name={l.from.system} /> → {l.to.station} · <SystemChip name={l.to.system} /></span>
            {#each l.cargo as c (c.name)}
              <span>{credits(c.tonnes)} t {c.name}: buy {credits(c.buy)} cr, sell {credits(c.sell)} cr</span>
            {:else}
              <span class="lo">Nothing worth carrying back: fly back empty.</span>
            {/each}
            <span class="q">{l.cargo.length ? `${credits(l.profit_per_t)} cr/t · ${credits(l.profit)} cr · ` : ""}{jumps(l.jumps, false)} · {l.ly.toFixed(1)} ly</span>
            <span class="lo">{[padLabel(l.to.pad), arrivalLabel(l.to.arrival_ls), l.source].filter(Boolean).join(" · ")}</span>
          </li>
        {/each}
      </ol>
      <p class="total">
        {credits(res.route.profit)} cr a {res.route.kind === "loop" ? "round" : "trip"} · about {credits(res.route.per_hour)} cr/h <span class="lo">(estimate)</span>
      </p>
      <p class="lo small">
        Spansh gives no profit per hour, so it's estimated: {minutes(res.route.minutes)} a round trip{res.route.kind === "single" ? " (flying back empty)" : ""} at {res.route.range.toFixed(1)} ly per jump, {ROUTE_ESTIMATE}. Spansh, {ago(new Date(res.route.fetched_at * 1000).toISOString())}.
      </p>
    {/if}
  {/if}
</div>

<style>
  .route { margin-top: 6px; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .kinds { display: flex; gap: 18px; margin: 0 0 10px; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .kinds button { color: var(--text-lo); }
  .kinds button:hover { color: var(--text); }
  .kinds .on { color: var(--accent); }
  .fields { display: flex; flex-direction: column; gap: 4px; margin: 8px 0 4px; font-size: var(--fs-s); }
  label { display: flex; align-items: center; gap: 8px; }
  label > span { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; }
  label em { font-style: normal; color: var(--text-lo); min-width: 1.5em; }
  input { all: unset; flex: 1; height: 26px; padding: 0 8px; background: var(--cell); color: var(--accent); font-family: var(--mono); }
  input::placeholder { color: var(--text-lo); }
  .go { margin: 4px 0 12px; padding: 6px 14px; background: var(--cell); color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .go:disabled { color: var(--text-lo); cursor: default; }
  h4 { margin: 6px 0 6px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  ol { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 10px; }
  li { display: flex; flex-direction: column; gap: 1px; font-size: var(--fs-s); line-height: 1.35; }
  .sys { color: var(--text); }
  .q { color: var(--accent); font-family: var(--mono); }
  .total { margin: 12px 0 4px; color: var(--accent); font-family: var(--mono); font-size: var(--fs-s); }
  .lo { color: var(--text-lo); }
  .warn { color: var(--accent); margin: 0 0 8px; }
  .small { font-size: var(--fs-s); }
  p { margin: 0 0 6px; }
</style>
