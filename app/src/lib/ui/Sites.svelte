<script lang="ts">
  // Raw-material sites near me: pick a kind (crystalline shards, brain trees, ...), then the
  // search kit lists the nearest bodies with jumps, distance, arrival distance and data age.
  import { SITE_KINDS, siteKind, siteResults, sites } from "../sites";
  import type { Filters, Origin, Results } from "../search";
  import SearchKit from "./SearchKit.svelte";

  let { kind = $bindable("crystalline_shards") }: { kind?: string } = $props();

  const current = $derived(siteKind(kind));

  async function run(origin: Origin, filters: Filters): Promise<Results> {
    return siteResults(await sites(current.id, origin, filters));
  }
</script>

<div class="sites">
  <div class="kinds" role="tablist" aria-label="Kind of site">
    {#each SITE_KINDS as k (k.id)}
      <button role="tab" aria-selected={k.id === current.id} class:on={k.id === current.id} onclick={() => (kind = k.id)}>{k.label}</button>
    {/each}
  </div>
  <p class="lo">{current.yields}</p>
  <SearchKit lookup={`sites.${current.id}`} offers={["age", "arrival"]} {run} title={`Nearest ${current.noun}`} busyText="Looking up sites…" />
</div>

<style>
  .sites { margin-top: 18px; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .kinds { display: flex; flex-wrap: wrap; gap: 18px; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); margin-bottom: 6px; }
  .kinds button { color: var(--text-lo); }
  .kinds button:hover { color: var(--text); }
  .kinds .on { color: var(--accent); }
  .lo { color: var(--text-lo); font-size: var(--fs-s); margin: 0 0 4px; }
</style>
