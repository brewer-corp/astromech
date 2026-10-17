<script lang="ts">
  // Colonisation candidates near me: unclaimed systems within 15 ly of a populated one (where a
  // claim can be made), optionally only those with rings, an Earth-like world or a landable
  // body, listed by the search kit with each system's bodies and the populated system in range.
  import { candidateResults, candidates, DEFAULT_RADIUS_LY, INTERESTS, LOOKUP, type Interest } from "../colonies";
  import type { Filters, Origin, Results } from "../search";
  import SearchKit from "./SearchKit.svelte";

  let { interest = $bindable("any") }: { interest?: Interest } = $props();

  const run = $derived.by(() => {
    const i = interest;
    return async (origin: Origin, filters: Filters): Promise<Results> => candidateResults(await candidates(i, origin, filters));
  });
</script>

<div class="colonies">
  <h3>Colonisation candidates</h3>
  <div class="kinds" role="tablist" aria-label="Bodies of interest">
    {#each INTERESTS as k (k.id)}
      <button role="tab" aria-selected={k.id === interest} class:on={k.id === interest} onclick={() => (interest = k.id)}>{k.label}</button>
    {/each}
  </div>
  <p class="lo">Unclaimed systems within 15 ly of a populated one: claim them at that system's Colonisation Contact. Spansh's data, so a claim made since may not show yet.</p>
  <SearchKit lookup={LOOKUP} offers={["distance", "age"]} hints={{ distance: DEFAULT_RADIUS_LY }} {run} title="Nearest" busyText="Asking Spansh…" />
</div>

<style>
  .colonies { margin-top: 32px; }
  h3 { margin: 0 0 8px; color: var(--text-hi); font-weight: 500; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .kinds { display: flex; flex-wrap: wrap; gap: 18px; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); margin-bottom: 6px; }
  .kinds button { color: var(--text-lo); }
  .kinds button:hover { color: var(--text); }
  .kinds .on { color: var(--accent); }
  .lo { color: var(--text-lo); font-size: var(--fs-s); margin: 0 0 4px; }
</style>
