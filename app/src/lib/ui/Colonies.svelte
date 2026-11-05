<script lang="ts">
  // Colonisation candidates near me: unclaimed systems within 15 ly of a populated one (where a
  // claim can be made), optionally only those with rings, an Earth-like world or a landable
  // body, listed by the search kit with each system's bodies and the populated system in range.
  import { candidateResults, candidates, DEFAULT_RADIUS_LY, INTERESTS, LOOKUP, type Interest } from "../colonies";
  import type { Filters, Origin, Results } from "../search";
  import Choice from "./Choice.svelte";
  import SearchKit from "./SearchKit.svelte";

  let { interest = $bindable("any") }: { interest?: Interest } = $props();

  const run = $derived.by(() => {
    const i = interest;
    return async (origin: Origin, filters: Filters): Promise<Results> => candidateResults(await candidates(i, origin, filters));
  });
</script>

<div class="colonies">
  <h3 class="section">Colonisation candidates</h3>
  <Choice class="kinds" label="Bodies of interest" options={INTERESTS.map((k) => ({ value: k.id, label: k.label }))} value={interest} onchange={(v) => (interest = v)} />
  <p class="lo">Unclaimed systems within 15 ly of a populated one: claim them at that system's Colonisation Contact. Spansh's data, so a claim made since may not show yet.</p>
  <SearchKit lookup={LOOKUP} offers={["distance", "age"]} hints={{ distance: DEFAULT_RADIUS_LY }} {run} title="Nearest" noun="candidates" busyText="Asking Spansh…" />
</div>

<style>
  .colonies { margin-top: 32px; }
  h3 { margin: 0 0 var(--sp-3); }
  .colonies > :global(.kinds) { margin-bottom: var(--sp-2); }
  .lo { color: var(--prose); font-size: var(--fs-s); margin: 0 0 4px; }
</style>
