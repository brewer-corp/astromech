<script lang="ts">
  // The Sell my cargo finder: the stations that pay the most for everything in the hold, each
  // commodity capped by demand, then the best station for each commodity on its own. It asks
  // nothing: the hold comes from the journal, listed above the results with what was left out.
  // Its filters are the ones Trade remembered as `trade.cargo`, which the voice question uses too.
  import { ago, jumps } from "../../lib/galaxy";
  import { placesRoute, rememberAnswer, type Finder } from "../../lib/places";
  import type { Filters, Origin, Results } from "../../lib/search";
  import { cargoResults, credits, DEFAULT_DISTANCE_LY, DEFAULT_MAX_AGE_DAYS, tradeCargo, type CargoPlan } from "../../lib/trade";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import FinderPage from "./FinderPage.svelte";

  let { finder }: { finder: Finder } = $props();

  /** The last cargo plan, for the hold line and the per-commodity list under the whole-load stations. */
  let plan = $state<CargoPlan | null>(null);

  async function run(origin: Origin, filters: Filters): Promise<Results> {
    const r = await tradeCargo(origin, filters);
    plan = r.plan;
    return cargoResults(r);
  }

  const held = $derived(plan ? plan.load.reduce((t, l) => t + l.tonnes, 0) : 0);
</script>

<FinderPage {finder}>
  <SearchKit
    lookup="trade.cargo"
    offers={["distance", "pad", "age", "arrival", "carriers", "odyssey"]}
    hints={{ distance: DEFAULT_DISTANCE_LY, age: DEFAULT_MAX_AGE_DAYS }}
    {run}
    title="Best for the whole load"
    joiner="near"
    noun="buyers"
    busyText="Asking Ardent Insight…"
    onanswer={(top) => rememberAnswer(finder.id, top && { ...top, context: `${credits(held)} t in the hold`, route: placesRoute("sell-cargo") })}
  >
    {#snippet lead()}
      {#if plan}
        <div class="hold" role="group" aria-label={finder.input}>
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
    {/snippet}
  </SearchKit>
  {#if plan && plan.per_commodity.length}
    <div class="each-wrap">
      <h2>Best for each on its own</h2>
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
</FinderPage>

<style>
  .hold { display: flex; flex-wrap: wrap; align-items: baseline; gap: 8px 18px; }
  .pair { display: inline-flex; align-items: baseline; gap: 6px; white-space: nowrap; }
  .k { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .q { font-family: var(--mono); color: var(--accent); }
  .left .q { color: var(--text); }
  .lo { color: var(--text-lo); }
  .each-wrap { padding: 8px 8px 28px; }
  h2 { margin: 18px 0 6px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .each { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; }
  .each li { display: grid; grid-template-columns: minmax(10rem, 1fr) minmax(12rem, 1.4fr) minmax(10rem, 1fr) auto; gap: 8px 18px; align-items: start; padding: 14px 8px; border-bottom: 1px solid #1a1c1e; font-size: var(--fs-s); line-height: 1.35; }
  .who { display: flex; flex-direction: column; gap: 2px; color: var(--text-hi); font-size: var(--fs); font-weight: 500; }
  .place { color: var(--text); }
  .facts { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
  @media (max-width: 900px) {
    .each li { grid-template-columns: minmax(0, 1fr); }
  }
</style>
