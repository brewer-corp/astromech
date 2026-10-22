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
  import { failure } from "../../lib/ui/errors";

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
  /** The ship's cargo, credits and range could not be read: the fields start empty and the hop at the default. */
  let defaultsErr = $state<string | null>(null);
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
  <div class="query">
    <div class="form-grid">
      <OriginInput {origin} onchange={(o) => (origin = o)} />
      <div class="kinds" role="group" aria-label="Route kind">
        <button type="button" class:on={kind === "single"} aria-pressed={kind === "single"} onclick={() => (kind = "single")}>Single hop</button>
        <button type="button" class:on={kind === "loop"} aria-pressed={kind === "loop"} onclick={() => (kind = "loop")}>Loop</button>
      </div>
      <div class="pick">
        <span class="lbl">Station</span>
        <div class="box">
          <input aria-label="Start station" placeholder={origin == null && docked ? docked : "Nearest the star"} bind:value={station} />
        </div>
      </div>
      <div class="fields">
      <label><span>Cargo</span><input aria-label="Cargo capacity" type="number" min="1" value={capacity ?? ""} oninput={(e) => (capacity = num(e.currentTarget.value))} /><em>t</em></label>
      <label><span>Credits</span><input aria-label="Credits" type="number" min="1" value={cash ?? ""} oninput={(e) => (cash = num(e.currentTarget.value))} /><em>cr</em></label>
      <label><span>Max hop</span><input aria-label="Max hop distance" type="number" min="1" step="0.1" value={hop ?? ""} oninput={(e) => (hop = num(e.currentTarget.value))} /><em>ly</em></label>
      </div>
    </div>
    <div class="tools">
      {#if filters}<SearchFilters {filters} {shipPad} offers={["pad", "age", "arrival", "carriers"]} hints={{ age: DEFAULT_MAX_AGE_DAYS }} onchange={(f) => void changeFilter(f)} />{/if}
      <button type="button" class="go" disabled={!ready || busy} onclick={() => void find()}>{busy ? "Finding…" : "Find route"}</button>
    </div>
    {#if defaultsErr}<p class="lo">{defaultsErr}</p>{/if}
    {#if filterErr}<p class="lo">{filterErr}</p>{/if}
  </div>

  <div class="out">
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
              <span class="place">
                <span class="leg">
                  <span class="st">{l.from.station}</span>
                  <span class="to" aria-hidden="true">→</span>
                  <span class="st">{l.to.station}</span>
                </span>
                <span class="via">
                  <SystemChip name={l.from.system} />
                  <span class="to" aria-hidden="true">→</span>
                  <SystemChip name={l.to.system} />
                </span>
              </span>
              <span class="cargo">
                {#each l.cargo as c (c.name)}
                  <span class="who">{credits(c.tonnes)} t {c.name}: buy {credits(c.buy)} cr, sell {credits(c.sell)} cr</span>
                {:else}
                  <span class="lo">Nothing worth carrying back: fly back empty.</span>
                {/each}
              </span>
              <span class="facts">
                {#if l.cargo.length}<span class="q">{credits(l.profit)} cr</span>{/if}
                <span class="dist">{l.cargo.length ? `${credits(l.profit_per_t)} cr/t · ` : ""}{jumps(l.jumps, false)} · {l.ly.toFixed(1)} ly</span>
              </span>
              <span class="meta">{[padLabel(l.to.pad), arrivalLabel(l.to.arrival_ls), l.source].filter(Boolean).join(" · ")}</span>
            </li>
          {/each}
        </ol>
        <div class="total">
          <span class="pair"><span class="k">{res.route.kind === "loop" ? "A round" : "A trip"}</span> <span class="q">{credits(res.route.profit)} cr</span></span>
          <span class="pair"><span class="k">About</span> <span class="q">{credits(res.route.per_hour)} cr/h</span></span>
        </div>
        <p class="lo small" title={`Spansh gives no profit per hour. Estimate: ${ROUTE_ESTIMATE}.`}>
          Estimated: {minutes(res.route.minutes)} a round trip{res.route.kind === "single" ? " (flying back empty)" : ""} at {res.route.range.toFixed(1)} ly per jump · Spansh, {ago(new Date(res.route.fetched_at * 1000).toISOString())}.
        </p>
      {/if}
    {:else}
      <p class="lo">Press Find route to see the {kind === "loop" ? "loop" : "hop"}. Spansh can take a minute.</p>
    {/if}
  </div>
</div>

<style>
  .route { display: flex; flex-direction: column; min-width: 0; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .query { display: flex; flex-direction: column; gap: 12px; padding: 12px 20px 20px; background: var(--bg-list); border-bottom: 1px solid var(--accent-dim); }
  .query :global(.origin) { margin: 0; min-width: 0; grid-column: 1; }
  .form-grid { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 12px 18px; align-items: center; }
  .kinds { display: flex; gap: 16px; grid-column: 2; grid-row: 1; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .kinds button { color: var(--text-lo); }
  .kinds button:hover { color: var(--text); }
  .kinds .on { color: var(--accent); }
  .pick { display: flex; align-items: center; gap: 8px; font-size: var(--fs); grid-column: 1; min-width: 0; }
  .lbl { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; font-size: var(--fs-s); }
  .box { position: relative; flex: 1; min-width: 0; display: flex; align-items: center; background: var(--cell-opt); border-bottom: 2px solid var(--accent); }
  .box input { all: unset; flex: 1; min-width: 0; width: 100%; height: 32px; padding: 0 10px; color: var(--text-hi); }
  .box input::placeholder { color: var(--text-lo); }
  .fields { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 10px 16px; grid-column: 1; min-width: 0; }
  label { display: grid; grid-template-columns: 5.5em minmax(0, 1fr) auto; align-items: center; gap: 8px; font-size: var(--fs-s); min-width: 0; }
  label > span { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); }
  label em { font-style: normal; color: var(--text-lo); min-width: 1.4em; }
  label input { all: unset; min-width: 0; height: 26px; padding: 0 8px; background: var(--cell); color: var(--accent); font-family: var(--mono); }
  label input::placeholder { color: var(--text-lo); }
  .tools { display: flex; flex-wrap: wrap; align-items: center; gap: 10px 18px; }
  .tools :global(.filters) { flex: 1 1 auto; }
  .go { margin-left: auto; height: var(--row); padding: 0 14px; background: var(--cell); color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); font-weight: 600; }
  .go:disabled { background: var(--cell-empty); color: var(--text-lo); cursor: default; }
  .go:not(:disabled):hover { color: var(--text-hi); }
  .out { padding: 24px 8px 28px; }
  h4 { margin: 0 0 16px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-l); letter-spacing: 0.04em; text-transform: uppercase; }
  ol { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; }
  li { display: grid; grid-template-columns: minmax(14rem, 1.5fr) minmax(12rem, 1.3fr) minmax(10rem, 1fr) auto; gap: 10px 18px; align-items: start; padding: 16px 8px; border-bottom: 1px solid #1a1c1e; font-size: var(--fs); line-height: 1.3; }
  .place, .cargo, .facts { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
  .leg, .via { display: flex; flex-wrap: wrap; align-items: baseline; gap: 6px 8px; min-width: 0; }
  .st { color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  .to { color: var(--text-lo); }
  .who { color: var(--text); }
  .q { color: var(--accent); font-family: var(--mono); }
  .dist { color: var(--accent); font-family: var(--mono); font-size: var(--fs-s); }
  .meta { color: var(--text-lo); font-size: var(--fs-s); white-space: nowrap; }
  .total { display: flex; flex-wrap: wrap; gap: 10px 28px; margin: 16px 8px 8px; padding: 14px 16px; background: var(--cell-empty); }
  .pair { display: inline-flex; align-items: baseline; gap: 8px; }
  .k { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .total .q { font-size: var(--fs-l); }
  .lo { color: var(--text-lo); }
  .warn { color: var(--accent); margin: 0 0 12px; }
  .small { font-size: var(--fs-s); margin: 0 8px; }
  p { margin: 0 0 8px; }
  @media (max-width: 1100px) {
    .fields { grid-template-columns: minmax(0, 1fr); }
  }
  @media (max-width: 900px) {
    .form-grid { grid-template-columns: minmax(0, 1fr); }
    .kinds { grid-column: 1; grid-row: auto; }
    li { grid-template-columns: minmax(0, 1fr); }
    .meta { white-space: normal; }
  }
</style>
