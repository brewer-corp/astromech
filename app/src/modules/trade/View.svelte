<script lang="ts">
  // Where to sell or buy one commodity near you (or a typed system), best price first, with
  // demand or stock, pad, arrival distance and data age; and where to sell the cargo in the
  // hold, as a whole load and per commodity. Each mode remembers its own filters, which voice
  // questions use too.
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
    tradeCargo,
    tradeSearch,
    type CargoPlan,
    type Commodity,
    type Side,
  } from "../../lib/trade";
  import CommodityInput from "./CommodityInput.svelte";

  type Mode = Side | "cargo";
  let mode = $state<Mode>("sell");
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
    return async (origin: Origin, filters: Filters): Promise<Results> => toResults(await tradeSearch(m, [c], origin, filters));
  });

  const hold = $derived(plan ? plan.load.map((l) => `${credits(l.tonnes)} t ${l.name}`).join(", ") : "");
  const kept = $derived(plan ? plan.kept.map((k) => `${credits(k.tonnes)} t ${k.name} (${k.reason})`).join(", ") : "");
</script>

<section>
  <div class="acts">
    <button class:on={mode === "sell"} onclick={() => (mode = "sell")}>Sell</button>
    <button class:on={mode === "buy"} onclick={() => (mode = "buy")}>Buy</button>
    <button class:on={mode === "cargo"} onclick={() => (mode = "cargo")}>My cargo</button>
  </div>
  <div class="main">
    {#if mode === "cargo"}
      <p class="sub">
        Stations that pay the most for everything in your hold, each commodity capped by the station's demand. Ask by voice too: "where should I sell my cargo".
      </p>
      {#if plan}
        <p class="hold">{hold ? `In the hold: ${hold}.` : "Nothing in the hold to sell."}{#if kept}<span class="lo">{` Left out: ${kept}.`}</span>{/if}</p>
      {/if}
      <SearchKit
        lookup="trade.cargo"
        offers={["distance", "pad", "age", "arrival", "carriers", "odyssey"]}
        hints={{ distance: DEFAULT_DISTANCE_LY, age: DEFAULT_MAX_AGE_DAYS }}
        {run}
        title="Best for the whole load"
        joiner="near"
        busyText="Asking Ardent Insight…"
      />
      {#if plan && plan.per_commodity.length}
        <h4>Best for each on its own</h4>
        <ul class="each">
          {#each plan.per_commodity as c (c.commodity)}
            <li>
              <span>{credits(c.carried)} t {c.name}</span>
              {#if c.best}
                <span class="sys">{c.best.station} · <SystemChip name={c.best.system} /></span>
                <span class="q">{credits(c.best.price)} cr · {credits(c.best.price * c.carried)} cr in all · {jumps(c.best.jumps, c.best.exact)} · {c.best.ly.toFixed(1)} ly</span>
                <span class="lo">demand {credits(c.best.volume)} · reported {ago(c.best.updated_at)}</span>
              {:else}
                <span class="lo">No market with demand for all of it.</span>
              {/if}
            </li>
          {/each}
        </ul>
      {/if}
    {:else}
      <p class="sub">
        {mode === "sell" ? "Stations buying" : "Stations selling"} {commodity.name}, {mode === "sell" ? "highest" : "lowest"} price first. Ask by voice too: "where can I {mode} {commodity.name.toLowerCase()}".
      </p>
      <CommodityInput value={commodity} onchange={(c) => (commodity = c)} />
      <SearchKit
        lookup={`trade.${mode}`}
        offers={["distance", "volume", "pad", "age", "arrival", "carriers", "odyssey"]}
        hints={{ volume: mode === "sell" ? "Demand" : "Stock", distance: DEFAULT_DISTANCE_LY, age: DEFAULT_MAX_AGE_DAYS }}
        {run}
        title={mode === "sell" ? "Best prices" : "Cheapest"}
        joiner="near"
        busyText="Asking Ardent Insight…"
      />
    {/if}
  </div>
</section>

<style>
  section { padding: 8px 0 28px; min-height: 0; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .acts { display: flex; flex-wrap: wrap; gap: 26px; padding: 14px 0 6px; text-transform: uppercase; letter-spacing: var(--caps); }
  .acts button { color: var(--text-lo); }
  .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0 0 14px; }
  .hold { font-size: var(--fs-s); margin: 0 0 8px; color: var(--text); }
  .main { max-width: 40rem; }
  h4 { margin: 18px 0 6px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .each { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 8px; }
  .each li { display: flex; flex-direction: column; gap: 1px; font-size: var(--fs-s); line-height: 1.35; }
  .q { color: var(--accent); font-family: var(--mono); }
  .lo { color: var(--text-lo); }
</style>
