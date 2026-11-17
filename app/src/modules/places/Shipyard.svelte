<script lang="ts">
  // The Shipyard finder: the nearest shipyards selling a ship, on the search kit. Stations whose
  // pads are too small for the ship are left out by the core, whatever pad is picked. The main
  // input is the ship; #/places/shipyard/<ship> picks it (Find or plot links there), so
  // back and reload keep it.
  import { SHIPS, parseShipRoute, shipRoute } from "../../lib/outfitting";
  import { rememberAnswer, type Finder } from "../../lib/places";
  import type { Filters, Origin } from "../../lib/search";
  import { stations, toResults } from "../../lib/stations";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import { followAddress } from "../../lib/finderAddress.svelte";
  import FinderPage from "./FinderPage.svelte";
  import MainInput from "./MainInput.svelte";

  let { finder }: { finder: Finder } = $props();

  const DEFAULT = "Krait MkII";
  let ship = $state(parseShipRoute(location.hash) ?? DEFAULT);
  const items = SHIPS.map((s) => ({ label: s.name }));

  // The address always names the ship, so back and the tile reopen it.
  followAddress("shipyard", { read: () => parseShipRoute(location.hash) ?? DEFAULT, route: shipRoute, get: () => ship, set: (q) => (ship = q) });

  // A new function per ship, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const s = ship;
    return async (origin: Origin, filters: Filters) => toResults(await stations({ kind: "ship", ship: s }, origin, filters));
  });
</script>

<FinderPage {finder}>
  {#snippet input()}
    <MainInput label={finder.input} current={ship} {items} placeholder="Type a ship" onpick={(name) => (location.hash = `#/${shipRoute(name)}`)} />
  {/snippet}
  <SearchKit
    lookup="stations.ship"
    offers={["pad", "carriers", "odyssey", "age", "arrival"]}
    {run}
    title="Nearest"
    noun="stations"
    busyText="Asking Spansh…"
    onanswer={(top) => rememberAnswer(finder.id, top && { ...top, context: ship, route: shipRoute(ship) })}
  />
</FinderPage>
