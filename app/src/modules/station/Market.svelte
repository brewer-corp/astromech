<script lang="ts">
  // The market docked at against nearby prices, read as answers (DESIGN.md Lookup result row):
  // each commodity in the hold, what it fetches here over what the best station nearby pays, the
  // gain per tonne and where; then the goods for sale here the same way, ranked by profit. Asked
  // once per dock (Ardent Insight, a request per commodity, at most a dozen); CHECK AGAIN asks
  // anew. The hold's first gain is the page's main value, in Value XL, Orange Lit: the largest
  // figure on the page (astromech-vru4). The hold is in load order, so its other gains are plain
  // Value L, while For sale here is ranked by its profit, Orange at the row's Value L.
  import Button from "../../lib/ui/Button.svelte";
  import { ago } from "../../lib/galaxy";
  import { when } from "../../lib/logbook";
  import { figure, isStale, jumpsFigure, jumpsUnit, ageCell, STALE_DAYS } from "../../lib/search";
  import { compareMarket, credits, DEFAULT_DISTANCE_LY, DEFAULT_MAX_AGE_DAYS, gainText, type MarketCompare, type MarketLine } from "../../lib/trade";
  import { failure } from "../../lib/ui/errors";
  import Failure from "../../lib/ui/Failure.svelte";
  import StationKind from "../../lib/ui/StationKind.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import { tip } from "../../lib/ui/tip";
  import { untrack } from "svelte";

  let { marketId }: { marketId: number | null } = $props();

  let res = $state<MarketCompare | null>(null);
  let err = $state<string | null>(null);
  let busy = $state(false);
  let asked = 0;

  function check() {
    const n = ++asked;
    busy = true;
    err = null;
    compareMarket()
      .then((r) => {
        if (n === asked) res = r;
      })
      .catch((e: unknown) => {
        if (n !== asked) return;
        res = null;
        err = failure("compare the market", e);
      })
      .finally(() => {
        if (n === asked) busy = false;
      });
  }

  // A new dock asks again; the page's 5-second refresh of the same dock doesn't (a derived
  // number only changes when the value does).
  const dock = $derived(marketId);
  $effect(() => {
    void dock;
    untrack(check);
  });

  /** What Best nearby is, in its group head's Tip. */
  const BEST = `The best price a station in another system pays, within ${DEFAULT_DISTANCE_LY} ly and ${DEFAULT_MAX_AGE_DAYS} days unless changed in Commodities › Sell`;
  const NOT_LOOKED_UP = "A dozen are looked up per dock, the biggest loads first";

  /** Staleness is judged as of when Ardent answered (DESIGN.md States); its age is told from now. */
  const asOf = $derived(res?.fetched_at ? res.fetched_at * 1000 : Date.now());
  const stale = (l: MarketLine) => !!l.best && isStale(l.best, STALE_DAYS, asOf);
  const age = (l: MarketLine) => {
    const c = ageCell(l.best?.updated_at ?? null, Date.now());
    return `${c.value}${c.unit ? ` ${c.unit}` : ""} old`;
  };
  const fetched = $derived(res?.fetched_at ? new Date(res.fetched_at * 1000).toISOString() : null);
</script>

<div class="market">
  {#if !err}<p class="head">
    {#if res}<span class="type-label" {@attach tip("The last time you opened its commodity market: open it again to refresh")}>Market as of {when(res.seen)} UTC, {ago(res.seen)}</span>{/if}
    {#if fetched}<span class="type-label">· Ardent Insight, asked {ago(fetched)}</span>{/if}
    <Button onclick={check} disabled={busy}>{busy ? "Asking…" : "Check again"}</Button>
  </p>{/if}
  {#if err}
    <Failure text={err} action="Check again" onaction={check} />
  {:else if busy && !res}
    <p class="type-label">Asking Ardent Insight…</p>
  {:else if res}
    <div class="lists" class:busy>
      {#if res.carried.length}
        <div class="list" role="table" aria-label="Your cargo" aria-busy={busy}>
          <div class="row heads type-label" role="row">
            <span role="columnheader">Your cargo</span>
            <span class="r" role="columnheader">Here · <span {@attach tip(BEST)}>best nearby</span></span>
            <span class="r" role="columnheader">Gain</span>
            <span role="columnheader">Station · system</span>
            <span class="r" role="columnheader">Trip</span>
          </div>
          {#each res.carried as l, i (l.commodity)}
            <div class="row result" class:stale={stale(l)} role="row">
              <span class="g" role="cell"><span class="type-name ell">{l.name}</span><span class="type-label">{credits(l.carried)} t</span></span>
              <span class="g r" role="cell">
                {#if l.sell_here === null}<span class="type-label">not bought</span>{:else}<span class="type-figures">{credits(l.sell_here)}</span>{/if}
                <span class="type-label">{l.best ? credits(l.best.price) : "–"}</span>
              </span>
              <span class="g r" role="cell">
                {#if l.gain === null}<span class="type-figures none">–</span>{:else}<span class="gain"><span class={i === 0 ? "type-value-xl main" : "type-value-l"}>{gainText(l.gain)}</span><span class="type-unit">cr/t</span></span>{/if}
              </span>
              {@render place(l, !l.checked ? "Not looked up" : "No station nearby takes all of it", !l.checked)}
            </div>
          {/each}
        </div>
      {:else}
        <p class="type-label line">Nothing in the hold to sell.</p>
      {/if}
      {#if res.kept.length}
        <p class="type-label line">Left out: {res.kept.map((k) => `${credits(k.tonnes)} t ${k.name} (${k.reason})`).join(", ")}.</p>
      {/if}

      {#if res.for_sale.length}
        <div class="list sale" role="table" aria-label="For sale here" aria-busy={busy}>
          <div class="row heads type-label" role="row">
            <span role="columnheader">For sale here</span>
            <span class="r" role="columnheader">Price · <span {@attach tip(BEST)}>best nearby</span></span>
            <span class="r" role="columnheader">Profit</span>
            <span role="columnheader">Station · system</span>
            <span class="r" role="columnheader">Trip</span>
          </div>
          {#each res.for_sale as l (l.commodity)}
            <div class="row result" class:stale={stale(l)} role="row">
              <span class="g" role="cell"><span class="type-name ell">{l.name}</span><span class="type-label">{credits(l.stock)} t</span></span>
              <span class="g r" role="cell">
                <span class="type-figures">{credits(l.buy_here ?? 0)}</span>
                <span class="type-label">{l.best ? credits(l.best.price) : "–"}</span>
              </span>
              <span class="g r" role="cell">
                {#if l.gain === null}<span class="type-figures none">–</span>{:else}<span class="gain"><span class="type-value-l ranked">{gainText(l.gain)}</span><span class="type-unit">cr/t</span></span>{/if}
              </span>
              {@render place(l, "No station nearby buys it", false)}
            </div>
          {/each}
        </div>
      {:else}
        <p class="type-label line">Nothing for sale here.</p>
      {/if}
      {#if res.unchecked.length}
        <p class="type-label line" {@attach tip("A dozen are looked up per dock, those furthest below their average price first")}>Not looked up: {res.unchecked.join(", ")}.</p>
      {/if}
    </div>
  {/if}
</div>

<!-- Where the best price is: the station over its system, then the trip (jumps over light years
     and the report's age). With none, what the row lacks in words across both groups. -->
{#snippet place(l: MarketLine, none: string, why: boolean)}
  {#if l.best}
    {@const j = jumpsFigure(l.best)}
    <span class="g place" role="cell">
      <span class="l1"><StationKind type={l.best.station_type} /><span class="type-name ell">{l.best.station}</span></span>
      <span class="l1 in"><span class="type-label ell">{l.best.system}</span><SystemChip name={l.best.system} icon /></span>
    </span>
    <span class="g r" role="cell">
      <span class="type-figures">{j}<span class="type-unit">{jumpsUnit(j)}</span></span>
      <span class="type-label">{figure(l.best.ly, 1)} ly · <span class="age" class:old={stale(l)}>{age(l)}</span></span>
    </span>
  {:else}
    <span class="g wide" role="cell"><span class="type-label" {@attach tip(why ? NOT_LOOKED_UP : null)}>{none}</span></span>
  {/if}
{/snippet}

<style>
  p { margin: 0; }
  .market { display: flex; flex-direction: column; gap: var(--sp-3); min-width: 0; }
  /* The market's age, then CHECK AGAIN right after it (DESIGN.md The Next To It Rule). */
  .head { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-2); }

  /* Both lists on one set of group tracks, so the hold and the goods for sale line up
     (DESIGN.md The Aligned Columns Rule). Only the place gives way, with an ellipsis. */
  .lists { display: grid; grid-template-columns: max-content max-content max-content minmax(0, max-content) max-content; column-gap: var(--sp-6); row-gap: var(--sp-3); width: fit-content; max-width: 100%; }
  .list, .line { grid-column: 1 / -1; }
  .list { display: grid; grid-template-columns: subgrid; }
  .sale { margin-top: var(--sp-3); }
  .row { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: center; padding: 0 var(--sp-3); white-space: nowrap; }
  .result { height: var(--row-result); }
  .result:hover { background: var(--row-hover); }
  .heads { height: var(--sp-8); border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); }
  .r { text-align: right; }
  .g { display: flex; flex-direction: column; justify-content: center; min-width: 0; overflow: hidden; }
  .g.r { align-items: flex-end; }
  .wide { grid-column: 4 / -1; }
  .l1 { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; }
  .ell { overflow: hidden; text-overflow: ellipsis; }
  .place :global(.kind) { flex: none; color: var(--label-grey); }
  .place :global(.chip) { flex: none; }
  .place :global(.chip .ibtn) { height: 1.25rem; }
  /* The system under the station, indented to the name past its type icon. */
  .in { padding-left: var(--sp-6); gap: 0; }
  .gain { display: flex; align-items: baseline; }
  /* For sale here is ranked by its profit: that column is Orange (DESIGN.md The Orange Means Act
     Rule), at the row's Value L, smaller than the hold's main value above it. */
  .ranked { color: var(--accent); }
  .none { color: var(--grey-dim); }
  /* A report older than 30 days as Ardent answered: its age in Orange, the ranked value a step down. */
  .age.old { color: var(--accent); }
  .stale .ranked, .stale .main { color: var(--orange-mid); }
  /* Asking again keeps the old rows Grey Dim until the answer replaces them (DESIGN.md States, Loading). */
  .busy .result, .busy .result :global(*) { color: var(--grey-dim); }
</style>
