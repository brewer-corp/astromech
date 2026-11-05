<script lang="ts">
  // Nearest systems for one faction-state lookup (Famine, an HGE pool), on the search kit; the
  // Faction states finder (modules/places) is the full page for these.
  import { nearby, nearbyResults } from "../galaxy";
  import type { Filters, Origin, Results } from "../search";
  import SearchKit from "./SearchKit.svelte";

  let { query }: { query: string } = $props();

  async function run(origin: Origin, filters: Filters): Promise<Results> {
    return nearbyResults(await nearby(query, origin, filters));
  }
</script>

<SearchKit lookup={`faction.${query}`} offers={["age"]} {run} noun="systems" busyText="Looking up live faction states…" />
