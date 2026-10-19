<script lang="ts">
  // The nearest shipyards selling a ship, on the search kit. Stations whose pads are too small
  // for the ship are left out by the shell, whatever pad is picked.
  // #/shipyard/<ship> picks the ship (the command palette and Find box link there), so back and
  // reload keep it.
  import { parseShipRoute, shipRoute } from "../../lib/outfitting";
  import type { Filters, Origin } from "../../lib/search";
  import { stations, toResults } from "../../lib/stations";
  import FindWant from "../../lib/ui/FindWant.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";

  const DEFAULT = "Krait MkII";
  let ship = $state(parseShipRoute(location.hash) ?? DEFAULT);

  $effect(() => {
    const onHash = () => (ship = parseShipRoute(location.hash) ?? DEFAULT);
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  $effect(() => {
    const want = `#/${shipRoute(ship)}`;
    if (location.hash !== want && (location.hash === "#/shipyard" || parseShipRoute(location.hash))) {
      history.replaceState(null, "", want);
    }
  });

  // A new function per ship, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const s = ship;
    return async (origin: Origin, filters: Filters) => toResults(await stations({ kind: "ship", ship: s }, origin, filters));
  });
</script>

<section>
  <SearchKit lookup="stations.ship" offers={["pad", "carriers", "odyssey", "age", "arrival"]} {run} title="Nearest" busyText="Asking Spansh…">
    {#snippet lead()}
      <FindWant current={ship} />
    {/snippet}
  </SearchKit>
</section>

<style>
  section { padding: 8px 0 32px; min-width: 0; }
</style>
