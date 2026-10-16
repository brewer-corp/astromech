<script lang="ts">
  // The nearest shipyards selling a ship, on the search kit. Stations whose pads are too small
  // for the ship are left out by the shell, whatever pad is picked.
  // #/shipyard/<ship> picks the ship (the command palette links there), so back and reload keep it.
  import { SHIPS, parseShipRoute, shipRoute } from "../../lib/outfitting";
  import type { Filters, Origin } from "../../lib/search";
  import { stations, toResults } from "../../lib/stations";
  import SearchKit from "../../lib/ui/SearchKit.svelte";

  const DEFAULT = "Krait MkII";
  let ship = $state(parseShipRoute(location.hash) ?? DEFAULT);

  $effect(() => {
    const onHash = () => (ship = parseShipRoute(location.hash) ?? DEFAULT);
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  // A new function per ship, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const s = ship;
    return async (origin: Origin, filters: Filters) => toResults(await stations({ kind: "ship", ship: s }, origin, filters));
  });
</script>

<section>
  <label class="pick">
    <span>Ship</span>
    <select value={ship} onchange={(e) => (location.hash = `#/${shipRoute(e.currentTarget.value)}`)}>
      {#each SHIPS as s (s.symbol)}<option value={s.name}>{s.name}</option>{/each}
    </select>
  </label>
  <SearchKit lookup="stations.ship" offers={["pad", "carriers", "odyssey", "age", "arrival"]} {run} title="Nearest" busyText="Asking Spansh…" />
</section>

<style>
  section { padding: 22px 0 28px; max-width: 46rem; }
  .pick { display: flex; align-items: center; gap: 8px; font-size: var(--fs-s); }
  .pick > span { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; }
  select { all: unset; flex: 1; height: 30px; padding: 0 8px; background: var(--cell); color: var(--accent); cursor: pointer; }
</style>
