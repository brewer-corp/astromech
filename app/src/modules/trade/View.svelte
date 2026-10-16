<script lang="ts">
  // Where to sell or buy one commodity near you (or a typed system), best price first, with
  // demand or stock, pad, arrival distance and data age. Each side remembers its own filters,
  // which voice questions use too.
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import type { Filters, Origin, Results } from "../../lib/search";
  import { DEFAULT_DISTANCE_LY, findCommodity, toResults, tradeSearch, type Commodity, type Side } from "../../lib/trade";
  import CommodityInput from "./CommodityInput.svelte";

  let side = $state<Side>("sell");
  let commodity = $state<Commodity>(findCommodity("painite")!);

  const run = $derived.by(() => {
    const s = side;
    const c = commodity.symbol;
    return async (origin: Origin, filters: Filters): Promise<Results> => {
      return toResults(await tradeSearch(s, [c], origin, filters));
    };
  });
</script>

<section>
  <div class="acts">
    <button class:on={side === "sell"} onclick={() => (side = "sell")}>Sell</button>
    <button class:on={side === "buy"} onclick={() => (side = "buy")}>Buy</button>
  </div>
  <p class="sub">
    {side === "sell" ? "Stations buying" : "Stations selling"} {commodity.name}, {side === "sell" ? "highest" : "lowest"} price first. Ask by voice too: "where can I {side} {commodity.name.toLowerCase()}".
  </p>
  <div class="main">
    <CommodityInput value={commodity} onchange={(c) => (commodity = c)} />
    <SearchKit
      lookup={`trade.${side}`}
      offers={["distance", "volume", "pad", "age", "arrival", "carriers", "odyssey"]}
      hints={{ volume: side === "sell" ? "Demand" : "Stock", distance: DEFAULT_DISTANCE_LY }}
      {run}
      title={side === "sell" ? "Best prices" : "Cheapest"}
      joiner="near"
      busyText="Asking Ardent Insight…"
    />
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
  .main { max-width: 40rem; }
</style>
