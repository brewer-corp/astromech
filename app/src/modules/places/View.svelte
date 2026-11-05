<script lang="ts">
  // Places (decision astromech-bi94, DESIGN.md Places home): #/places is the home page of finder
  // tiles; #/places/<finder>[/<input>] is one finder's page. Each finder reads its own input
  // from the address, so back and reload keep it.
  import { parsePlaces, type Finder, type FinderId } from "../../lib/places";
  import type { Component } from "svelte";
  import Home from "./Home.svelte";
  import Outfitting from "./Outfitting.svelte";
  import Services from "./Services.svelte";
  import Shipyard from "./Shipyard.svelte";
  import Traders from "./Traders.svelte";

  /** Every finder in lib/places.ts FINDERS has its page here (the type checks it). */
  const PAGES: Record<FinderId, Component<{ finder: Finder }>> = {
    services: Services,
    outfitting: Outfitting,
    shipyard: Shipyard,
    "material-traders": Traders,
  };

  let finder = $state(parsePlaces(location.hash)?.finder ?? null);

  $effect(() => {
    const onHash = () => (finder = parsePlaces(location.hash)?.finder ?? null);
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  const Page = $derived(finder ? PAGES[finder.id] : null);
</script>

{#if finder && Page}
  <Page {finder} />
{:else}
  <Home />
{/if}
