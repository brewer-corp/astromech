<script lang="ts">
  // The nearest station with a service, on the search kit. Traders and brokers come by type.
  import type { Filters, Origin } from "../../lib/search";
  import { SERVICES, stations, toResults, type ServiceId } from "../../lib/stations";
  import SearchKit from "../../lib/ui/SearchKit.svelte";

  let service = $state<ServiceId>("interstellar_factors");

  // A new function per service, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const s = service;
    return async (origin: Origin, filters: Filters) => toResults(await stations({ kind: "service", service: s }, origin, filters));
  });
</script>

<section>
  <label class="pick">
    <span>Service</span>
    <select bind:value={service}>
      {#each SERVICES as s (s.id)}<option value={s.id}>{s.label}</option>{/each}
    </select>
  </label>
  <SearchKit lookup="stations.service" offers={["pad", "carriers", "odyssey", "age", "arrival"]} {run} title="Nearest" busyText="Asking Spansh…" />
</section>

<style>
  section { padding: 22px 0 28px; max-width: 46rem; }
  .pick { display: flex; align-items: center; gap: 8px; font-size: var(--fs-s); }
  .pick > span { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; }
  select { all: unset; flex: 1; height: 30px; padding: 0 8px; background: var(--cell); color: var(--accent); cursor: pointer; }
</style>
