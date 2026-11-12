<script lang="ts">
  // The Raw sites finder: the nearest bodies with a kind of raw-material site (crystalline
  // shards, brain trees, crashed ships, geological sites, the known grade 4 farms), with jumps,
  // distance, arrival distance and data age. The main input is the kind;
  // #/places/raw-sites/<kind> keeps it on back and reload. Each kind keeps the filters it was
  // remembered under on Materials (`sites.<kind>`), which its voice query uses too.
  import { parseSiteRoute, rememberAnswer, showing, siteRoute, type Finder } from "../../lib/places";
  import type { Filters, Origin } from "../../lib/search";
  import { SITE_KINDS, siteKind, siteResults, sites } from "../../lib/sites";
  import Dropdown from "../../lib/ui/Dropdown.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import FinderPage from "./FinderPage.svelte";

  let { finder }: { finder: Finder } = $props();

  const DEFAULT = "crystalline_shards";
  let kind = $state(parseSiteRoute(location.hash) ?? DEFAULT);
  const current = $derived(siteKind(kind));

  /** This finder is the page shown (a hash change may be leaving it). */
  const here = () => showing("raw-sites");

  $effect(() => {
    const onHash = () => {
      if (here()) kind = parseSiteRoute(location.hash) ?? DEFAULT;
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  // The address always names the kind, so back and the tile reopen it.
  $effect(() => {
    const want = `#/${siteRoute(kind)}`;
    if (here() && location.hash !== want) history.replaceState(null, "", want);
  });

  // A new function per kind, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const k = kind;
    return async (origin: Origin, filters: Filters) => siteResults(await sites(k, origin, filters));
  });
</script>

<FinderPage {finder}>
  {#snippet input()}
    <Dropdown aria-label={finder.input} value={kind} onchange={(e) => (location.hash = `#/${siteRoute(e.currentTarget.value)}`)}>
      {#each SITE_KINDS as k (k.id)}<option value={k.id}>{k.label}</option>{/each}
    </Dropdown>
  {/snippet}
  <SearchKit
    lookup={`sites.${current.id}`}
    offers={["age", "arrival"]}
    {run}
    title={`Nearest ${current.noun}`}
    noun="sites"
    busyText="Looking up sites…"
    onanswer={(top) => rememberAnswer(finder.id, top && { ...top, context: current.label, route: siteRoute(current.id) })}
  >
    {#snippet lead()}<p class="about">{current.yields}</p>{/snippet}
  </SearchKit>
</FinderPage>

<style>
  .about { margin: 0; max-width: var(--measure); font-size: var(--fs-s); color: var(--orange-mid); }
</style>
