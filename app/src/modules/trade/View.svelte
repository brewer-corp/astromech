<script lang="ts">
  // Where to sell or buy one commodity near you (or a typed system) or anywhere in the galaxy,
  // best price first, with demand or stock, pad, arrival distance and data age; where to sell
  // the cargo in the hold, as a whole load and per commodity; and a single-hop or loop trade
  // route from here. Each mode remembers its own filters, which voice questions use too.
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import { ago, jumps } from "../../lib/galaxy";
  import type { Filters, Origin, Results } from "../../lib/search";
  import {
    cargoResults,
    credits,
    DEFAULT_DISTANCE_LY,
    DEFAULT_MAX_AGE_DAYS,
    findCommodity,
    toResults,
    tradeAnywhere,
    tradeCargo,
    tradeSearch,
    type CargoPlan,
    type Commodity,
    type Side,
  } from "../../lib/trade";
  import CommodityInput from "./CommodityInput.svelte";
  import Route from "./Route.svelte";

  type Mode = Side | "cargo" | "route";
  let mode = $state<Mode>("sell");
  /** Sell and Buy: near the origin, or anywhere in the galaxy. */
  let anywhere = $state(false);
  let commodity = $state<Commodity>(findCommodity("painite")!);
  /** The last cargo plan, for the per-commodity list under the whole-load stations. */
  let plan = $state<CargoPlan | null>(null);

  const run = $derived.by(() => {
    const m = mode;
    const c = commodity.symbol;
    if (m === "cargo") {
      return async (origin: Origin, filters: Filters): Promise<Results> => {
        const r = await tradeCargo(origin, filters);
        plan = r.plan;
        return cargoResults(r);
      };
    }
    // The route mode has its own search (Route.svelte); only the market lookups get here.
    const side: Side = m === "buy" ? "buy" : "sell";
    if (anywhere) return async (origin: Origin, filters: Filters): Promise<Results> => toResults(await tradeAnywhere(side, c, origin, filters), true);
    return async (origin: Origin, filters: Filters): Promise<Results> => toResults(await tradeSearch(side, [c], origin, filters));
  });

  const sub = $derived.by(() => {
    if (mode === "cargo") return "Stations that pay the most for everything in the hold, each commodity capped by demand.";
    if (mode === "route") return "The most profitable hop from a station, or a loop that brings cargo back.";
    const who = mode === "sell" ? "Stations buying" : "Stations selling";
    const order = mode === "sell" ? "highest" : "lowest";
    if (anywhere) return `${who} ${commodity.name} anywhere in the galaxy, ${order} price first. Ardent's best 100 that pass the filters.`;
    return `${who} ${commodity.name}, ${order} price first.`;
  });
</script>

<section>
  {#if mode === "route"}
    <div class="query route-head">
      <div class="acts">
        <div class="tabs" role="tablist" aria-label="Trade">
          <button type="button" role="tab" aria-selected={false} onclick={() => (mode = "sell")}>Sell</button>
          <button type="button" role="tab" aria-selected={false} onclick={() => (mode = "buy")}>Buy</button>
          <button type="button" role="tab" aria-selected={false} onclick={() => (mode = "cargo")}>My cargo</button>
          <button type="button" role="tab" aria-selected={true} class="on" onclick={() => (mode = "route")}>Route</button>
        </div>
      </div>
      <p class="sub">{sub}</p>
    </div>
    <Route />
  {:else}
    <SearchKit
      lookup={mode === "cargo" ? "trade.cargo" : anywhere ? `trade.${mode}.anywhere` : `trade.${mode}`}
      offers={mode === "cargo"
        ? ["distance", "pad", "age", "arrival", "carriers", "odyssey"]
        : anywhere
          ? ["volume", "pad", "age", "arrival", "carriers", "odyssey"]
          : ["distance", "volume", "pad", "age", "arrival", "carriers", "odyssey"]}
      hints={{ volume: mode === "sell" ? "Demand" : "Stock", distance: DEFAULT_DISTANCE_LY, age: DEFAULT_MAX_AGE_DAYS }}
      {run}
      title={mode === "cargo" ? "Best for the whole load" : `${mode === "sell" ? "Best prices" : "Cheapest"}${anywhere ? " anywhere" : ""}`}
      joiner={mode === "cargo" ? "near" : anywhere ? "· distances from" : "near"}
      busyText="Asking Ardent Insight…"
    >
      {#snippet header()}
        <div class="acts">
          <div class="tabs" role="tablist" aria-label="Trade">
            <button type="button" role="tab" aria-selected={mode === "sell"} class:on={mode === "sell"} onclick={() => (mode = "sell")}>Sell</button>
            <button type="button" role="tab" aria-selected={mode === "buy"} class:on={mode === "buy"} onclick={() => (mode = "buy")}>Buy</button>
            <button type="button" role="tab" aria-selected={mode === "cargo"} class:on={mode === "cargo"} onclick={() => (mode = "cargo")}>My cargo</button>
            <button type="button" role="tab" aria-selected={false} onclick={() => (mode = "route")}>Route</button>
          </div>
          {#if mode === "sell" || mode === "buy"}
            <span class="scope" role="group" aria-label="Search range">
              <button type="button" class:on={!anywhere} aria-pressed={!anywhere} onclick={() => (anywhere = false)}>Near</button>
              <button type="button" class:on={anywhere} aria-pressed={anywhere} onclick={() => (anywhere = true)}>Anywhere</button>
            </span>
          {/if}
        </div>
        <p class="sub">{sub}</p>
      {/snippet}
      {#snippet lead()}
        {#if mode === "cargo"}
          {#if plan}
            <div class="hold" role="group" aria-label="Hold">
              {#if plan.load.length}
                {#each plan.load as l (l.symbol)}
                  <span class="pair"><span class="k">{l.name}</span> <span class="q">{credits(l.tonnes)} t</span></span>
                {/each}
              {:else}
                <span class="lo">Nothing in the hold to sell.</span>
              {/if}
              {#each plan.kept as k, i (`${k.name}:${k.reason}:${i}`)}
                <span class="pair left"><span class="k">{k.name}</span> <span class="q">{credits(k.tonnes)} t</span> <span class="lo">{k.reason}</span></span>
              {/each}
            </div>
          {/if}
        {:else}
          <CommodityInput value={commodity} onchange={(c) => (commodity = c)} />
        {/if}
      {/snippet}
    </SearchKit>
    {#if mode === "cargo" && plan && plan.per_commodity.length}
      <div class="each-wrap">
        <h4>Best for each on its own</h4>
        <ul class="each">
          {#each plan.per_commodity as c (c.commodity)}
            <li>
              <span class="who">
                {credits(c.carried)} t {c.name}
                {#if c.partial && !c.best}<span class="lo">Takes {credits(Math.min(c.partial.volume, c.carried))} of {credits(c.carried)} t</span>{/if}
              </span>
              {#if c.best}
                <span class="place">{c.best.station} · <SystemChip name={c.best.system} /></span>
                <span class="facts">
                  <span class="q">{credits(c.best.price)} cr</span>
                  <span class="q">{credits(c.best.price * c.carried)} cr in all</span>
                  <span class="q">{jumps(c.best.jumps, c.best.exact)} · {c.best.ly.toFixed(1)} ly</span>
                </span>
                <span class="lo">demand {credits(c.best.volume)} · reported {ago(c.best.updated_at)}</span>
              {:else if c.partial}
                <span class="place">{c.partial.station} · <SystemChip name={c.partial.system} /></span>
                <span class="facts">
                  <span class="q">{credits(c.partial.price)} cr</span>
                  <span class="q">{credits(c.partial.price * Math.min(c.partial.volume, c.carried))} cr for that</span>
                  <span class="q">{jumps(c.partial.jumps, c.partial.exact)} · {c.partial.ly.toFixed(1)} ly</span>
                </span>
                <span class="lo">demand {credits(c.partial.volume)} · reported {ago(c.partial.updated_at)}</span>
              {:else}
                <span class="lo">No market with demand for any of it.</span>
              {/if}
            </li>
          {/each}
        </ul>
      </div>
    {/if}
  {/if}
</section>

<style>
  section { padding: 0; min-width: 0; min-height: 0; display: flex; flex-direction: column; height: 100%; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .query { display: flex; flex-direction: column; gap: 12px; padding: 18px 20px 12px; background: var(--bg-list); }
  .route-head { padding-bottom: 0; }
  .acts { display: flex; flex-wrap: wrap; gap: 26px; align-items: baseline; text-transform: uppercase; letter-spacing: var(--caps); }
  .tabs { display: flex; flex-wrap: wrap; gap: 26px; align-items: baseline; }
  .acts button { color: var(--text-lo); }
  .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .scope { display: flex; gap: 16px; margin-left: auto; font-size: var(--fs-s); }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0; }
  .hold { display: flex; flex-wrap: wrap; align-items: baseline; gap: 8px 18px; }
  .pair { display: inline-flex; align-items: baseline; gap: 6px; white-space: nowrap; }
  .k { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .q { font-family: var(--mono); color: var(--accent); }
  .left .q { color: var(--text); }
  .lo { color: var(--text-lo); }
  .each-wrap { padding: 8px 8px 28px; }
  h4 { margin: 18px 0 6px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .each { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; }
  .each li { display: grid; grid-template-columns: minmax(10rem, 1fr) minmax(12rem, 1.4fr) minmax(10rem, 1fr) auto; gap: 8px 18px; align-items: start; padding: 14px 8px; border-bottom: 1px solid #1a1c1e; font-size: var(--fs-s); line-height: 1.35; }
  .who { display: flex; flex-direction: column; gap: 2px; color: var(--text-hi); font-size: var(--fs); font-weight: 500; }
  .place { color: var(--text); }
  .facts { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
  @media (max-width: 900px) {
    .each li { grid-template-columns: minmax(0, 1fr); }
    .scope { margin-left: 0; }
  }
</style>
