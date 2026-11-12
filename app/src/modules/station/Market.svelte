<script lang="ts">
  // The market docked at against nearby prices: what each commodity in the hold fetches here and
  // at the best station nearby, and what the goods for sale here sell for nearby. Asked once per
  // dock (Ardent Insight, a request per commodity, at most a dozen); "Check again" asks anew.
  import Button from "../../lib/ui/Button.svelte";
  import { ago, jumps } from "../../lib/galaxy";
  import { when } from "../../lib/logbook";
  import { compareMarket, credits, DEFAULT_DISTANCE_LY, DEFAULT_MAX_AGE_DAYS, gainText, type MarketCompare, type MarketLine } from "../../lib/trade";
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
        err = String(e);
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

  /** What Best nearby is, in its column head's Tip. */
  const BEST = `The best price a station in another system pays, within ${DEFAULT_DISTANCE_LY} ly and ${DEFAULT_MAX_AGE_DAYS} days unless changed in Commodities › Sell`;

  const where = (l: MarketLine) => (l.best ? `${jumps(l.best.jumps, l.best.exact)} · ${l.best.ly.toFixed(1)} ly · demand ${credits(l.best.volume)} · ${ago(l.best.updated_at)}` : "");
</script>

<div class="head">
  <p class="sub">
    {#if res}<span {@attach tip("The last time you opened its commodity market: open it again to refresh")}>Market as of {when(res.seen)} UTC ({ago(res.seen)})</span>{/if}
    {#if res?.fetched_at}· Ardent Insight, asked {ago(new Date(res.fetched_at * 1000).toISOString())}{/if}
  </p>
  <Button onclick={check} disabled={busy}>{busy ? "Asking…" : "Check again"}</Button>
</div>
{#if err}
  <p class="err">{err}</p>
{:else if busy && !res}
  <p class="lo">Asking Ardent Insight…</p>
{:else if res}
  {#if res.carried.length}
    <table aria-label="Your cargo">
      <thead><tr><th>Your cargo</th><th class="num">Here</th><th class="num" {@attach tip(BEST)}>Best nearby</th><th class="num">Difference</th><th>Where</th></tr></thead>
      <tbody>
        {#each res.carried as l (l.commodity)}
          <tr>
            <td class="name">{credits(l.carried)} t {l.name}</td>
            <td class="num">{l.sell_here === null ? "not bought" : credits(l.sell_here)}</td>
            <td class="num">{l.best ? credits(l.best.price) : "–"}</td>
            <td class="num" class:up={(l.gain ?? 0) > 0}>{l.gain === null ? "" : `${gainText(l.gain)}/t`}</td>
            <td>
              {#if l.best}<span>{l.best.station} <SystemChip name={l.best.system} /></span> <span class="lo">{where(l)}</span>{:else if !l.checked}<span class="lo" {@attach tip("A dozen are looked up per dock, the biggest loads first")}>Not looked up</span>{:else}<span class="lo">No station nearby takes all of it.</span>{/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {:else}
    <p class="lo">Nothing in the hold to sell.</p>
  {/if}
  {#if res.kept.length}
    <p class="lo">Left out: {res.kept.map((k) => `${credits(k.tonnes)} t ${k.name} (${k.reason})`).join(", ")}.</p>
  {/if}

  {#if res.for_sale.length}
    <table aria-label="For sale here">
      <thead><tr><th>For sale here</th><th class="num">Price</th><th class="num" {@attach tip(BEST)}>Best nearby</th><th class="num">Profit</th><th>Where</th></tr></thead>
      <tbody>
        {#each res.for_sale as l (l.commodity)}
          <tr>
            <td class="name">{l.name} <span class="lo">{credits(l.stock)} t</span></td>
            <td class="num">{credits(l.buy_here ?? 0)}</td>
            <td class="num">{l.best ? credits(l.best.price) : "–"}</td>
            <td class="num" class:up={(l.gain ?? 0) > 0}>{l.gain === null ? "" : `${gainText(l.gain)}/t`}</td>
            <td>
              {#if l.best}<span>{l.best.station} <SystemChip name={l.best.system} /></span> <span class="lo">{where(l)}</span>{:else}<span class="lo">No station nearby buys it.</span>{/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {:else}
    <p class="lo">Nothing for sale here.</p>
  {/if}
  {#if res.unchecked.length}
    <p class="lo" {@attach tip("A dozen are looked up per dock, those furthest below their average price first")}>Not looked up: {res.unchecked.join(", ")}.</p>
  {/if}
{/if}

<style>
  .head { display: flex; align-items: baseline; gap: 18px; max-width: 60rem; }
  .sub { color: var(--orange-mid); font-size: var(--fs-s); margin: 8px 0; flex: 1; }
  .err { color: var(--accent); }
  .lo { color: var(--orange-mid); }
  .name { white-space: nowrap; }
  .up { color: var(--accent); }
  table { border-collapse: collapse; width: 100%; max-width: 60rem; margin-top: 6px; }
  th { text-align: left; color: var(--orange-mid); font-size: var(--fs-s); font-weight: 400; padding: 0 12px 6px 0; }
  td { padding: 5px 12px 5px 0; border-top: 1px solid var(--line); vertical-align: baseline; }
  td .lo { font-size: var(--fs-s); }
  .num { text-align: right; white-space: nowrap; }
</style>
