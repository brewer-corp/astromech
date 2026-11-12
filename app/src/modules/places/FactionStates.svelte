<script lang="ts">
  // The Faction states finder: the nearest systems with a faction in a state, for wake scanning
  // (Famine) or a High Grade Emissions pool (an allegiance, Boom, War, Civil Unrest, Outbreak),
  // from Spansh with the states seen on the commander's own visits. HGE pools are ranked by
  // population and distance, the rest by jumps. The main input is the lookup;
  // #/places/faction-states/<id> keeps it on back and reload. Each keeps the filters it was
  // remembered under on Materials (`faction.<id>`), which its voice query uses too.
  import { FACTION_STATES, nearby, nearbyResults } from "../../lib/galaxy";
  import { factionRoute, parseFactionRoute, rememberAnswer, showing, type Finder } from "../../lib/places";
  import type { Filters, Origin } from "../../lib/search";
  import Dropdown from "../../lib/ui/Dropdown.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import FinderPage from "./FinderPage.svelte";

  let { finder }: { finder: Finder } = $props();

  const DEFAULT = "proto";
  let query = $state(parseFactionRoute(location.hash) ?? DEFAULT);
  const current = $derived(FACTION_STATES.find((q) => q.id === query) ?? FACTION_STATES[0]);

  /** This finder is the page shown (a hash change may be leaving it). */
  const here = () => showing("faction-states");

  $effect(() => {
    const onHash = () => {
      if (here()) query = parseFactionRoute(location.hash) ?? DEFAULT;
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  // The address always names the lookup, so back and the tile reopen it.
  $effect(() => {
    const want = `#/${factionRoute(query)}`;
    if (here() && location.hash !== want) history.replaceState(null, "", want);
  });

  // A new function per lookup, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const q = query;
    return async (origin: Origin, filters: Filters) => nearbyResults(await nearby(q, origin, filters));
  });
</script>

<FinderPage {finder}>
  {#snippet input()}
    <Dropdown aria-label={finder.input} value={query} onchange={(e) => (location.hash = `#/${factionRoute(e.currentTarget.value)}`)}>
      {#each FACTION_STATES as q (q.id)}<option value={q.id}>{q.name}</option>{/each}
    </Dropdown>
  {/snippet}
  <SearchKit
    lookup={`faction.${current.id}`}
    offers={["age"]}
    {run}
    noun="systems"
    busyText="Looking up live faction states…"
    onanswer={(top) => rememberAnswer(finder.id, top && { ...top, context: current.name, route: factionRoute(current.id) })}
  >
    {#snippet lead()}<p class="about">{current.about}</p>{/snippet}
  </SearchKit>
</FinderPage>

<style>
  .about { margin: 0; max-width: var(--measure); font-size: var(--fs-s); color: var(--orange-mid); }
</style>
