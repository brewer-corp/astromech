<script lang="ts">
  // The Sell and Buy finders: where one commodity fetches the most (stations buying it, highest
  // price first) or costs the least (stations selling it, lowest first), near the origin or
  // anywhere in the galaxy, from Ardent Insight. The main input is the commodity; the range is
  // a choice. #/places/<sell|buy>/<symbol>[?range=anywhere] keeps both on back and reload.
  // Each side and range keeps the filters Trade remembered for it (`trade.sell`,
  // `trade.sell.anywhere`, ...), which voice questions use too.
  import { marketRoute, parseMarketRoute, rememberAnswer, showing, type Finder, type MarketWant } from "../../lib/places";
  import type { Filters, Origin, Results } from "../../lib/search";
  import { commodities, DEFAULT_DISTANCE_LY, DEFAULT_MAX_AGE_DAYS, toResults, tradeAnywhere, tradeSearch, type Side } from "../../lib/trade";
  import Choice from "../../lib/ui/Choice.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import FinderPage from "./FinderPage.svelte";
  import MainInput from "./MainInput.svelte";

  let { finder }: { finder: Finder } = $props();

  // A Places page is made for one finder; Sell and Buy are two pages of this one.
  // svelte-ignore state_referenced_locally
  const side: Side = finder.id === "buy" ? "buy" : "sell";
  const fallback = (): MarketWant => ({ side, symbol: "painite", anywhere: false });
  const read = () => {
    const w = parseMarketRoute(location.hash);
    return w?.side === side ? w : fallback();
  };
  let want = $state<MarketWant>(read());
  const name = $derived(commodities.find((c) => c.symbol === want.symbol)?.name ?? want.symbol);
  const items = commodities.map((c) => ({ label: c.name, detail: c.category, keywords: c.aliases?.join(" ") }));

  /** This finder is the page shown (a hash change may be leaving it). */
  const here = () => showing(side);

  $effect(() => {
    const onHash = () => {
      if (here()) want = read();
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  // The address always names the commodity and range, so back and the tile reopen them.
  $effect(() => {
    const to = `#/${marketRoute(want)}`;
    if (here() && location.hash !== to) history.replaceState(null, "", to);
  });

  function go(w: MarketWant) {
    want = w;
    location.hash = `#/${marketRoute(w)}`;
  }

  // A new function per commodity and range, so the kit searches again when either changes.
  const run = $derived.by(() => {
    const { symbol, anywhere } = want;
    if (anywhere) return async (origin: Origin, filters: Filters): Promise<Results> => toResults(await tradeAnywhere(side, symbol, origin, filters), true);
    return async (origin: Origin, filters: Filters): Promise<Results> => toResults(await tradeSearch(side, [symbol], origin, filters));
  });
</script>

<FinderPage {finder}>
  {#snippet input()}
    <MainInput
      label={finder.input}
      current={name}
      {items}
      placeholder="Type a commodity"
      onpick={(n) => go({ ...want, symbol: commodities.find((c) => c.name === n)?.symbol ?? want.symbol })}
    />
  {/snippet}
  <SearchKit
    lookup={`trade.${side}${want.anywhere ? ".anywhere" : ""}`}
    offers={want.anywhere ? ["volume", "pad", "age", "arrival", "carriers", "odyssey"] : ["distance", "volume", "pad", "age", "arrival", "carriers", "odyssey"]}
    hints={{ volume: side === "sell" ? "Demand" : "Stock", distance: DEFAULT_DISTANCE_LY, age: DEFAULT_MAX_AGE_DAYS }}
    {run}
    title={`${side === "sell" ? "Best prices" : "Cheapest"}${want.anywhere ? " anywhere" : ""}`}
    joiner={want.anywhere ? "· distances from" : "near"}
    noun={side === "buy" ? "sellers" : "buyers"}
    busyText="Asking Ardent Insight…"
    onanswer={(top) => rememberAnswer(finder.id, top && { ...top, context: name, route: marketRoute(want) })}
  >
    {#snippet lead()}
      <div class="choices">
        <span class="label">Range</span>
        <Choice
          label="Search range"
          value={want.anywhere ? "anywhere" : "near"}
          options={[
            { value: "near", label: "Near", title: `Within a distance of the origin` },
            { value: "anywhere", label: "Anywhere", title: "Ardent's best 100 prices in the galaxy that pass the filters" },
          ]}
          onchange={(v) => go({ ...want, anywhere: v === "anywhere" })}
        />
      </div>
    {/snippet}
  </SearchKit>
</FinderPage>

<style>
  .choices { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-3); }
  .label { font-size: var(--fs-s); color: var(--orange-mid); }
</style>
