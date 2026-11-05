<script lang="ts">
  // The nearest station with a service, on the search kit. Traders and brokers come by type.
  // Type a service, module or ship in Find; #/services/<id> keeps the service on back and reload.
  import { parseServiceRoute, serviceRoute } from "../../lib/find";
  import type { Filters, Origin } from "../../lib/search";
  import { SERVICES, stations, toResults, type ServiceId } from "../../lib/stations";
  import FindWant from "../../lib/ui/FindWant.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";

  const DEFAULT: ServiceId = "interstellar_factors";
  let service = $state<ServiceId>(parseServiceRoute(location.hash) ?? DEFAULT);
  const label = $derived(SERVICES.find((s) => s.id === service)?.label ?? service);

  $effect(() => {
    const onHash = () => (service = parseServiceRoute(location.hash) ?? DEFAULT);
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  $effect(() => {
    const want = `#/${serviceRoute(service)}`;
    if (location.hash !== want && (location.hash === "#/services" || parseServiceRoute(location.hash))) {
      history.replaceState(null, "", want);
    }
  });

  // A new function per service, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const s = service;
    return async (origin: Origin, filters: Filters) => toResults(await stations({ kind: "service", service: s }, origin, filters));
  });
</script>

<section>
  <SearchKit lookup="stations.service" offers={["pad", "carriers", "odyssey", "age", "arrival"]} {run} title="Nearest" busyText="Asking Spansh…">
    {#snippet lead()}
      <FindWant current={label} />
    {/snippet}
  </SearchKit>
</section>

<style>
  /* The route strip took over from the Galaxy rail; the page keeps its spacing and DESIGN.md's list width. */
  section { padding: 28px 0 72px; min-width: 0; max-width: var(--list-w); }
</style>
