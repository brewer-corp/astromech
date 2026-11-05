<script lang="ts">
  // The Services finder: the nearest stations with a service, on the search kit. The main input
  // is the service; #/places/services/<id> keeps it on back and reload.
  import { parseServiceRoute, rememberAnswer, serviceRoute, showing, type Finder } from "../../lib/places";
  import type { Filters, Origin } from "../../lib/search";
  import { SERVICES, stations, toResults, type ServiceId } from "../../lib/stations";
  import Dropdown from "../../lib/ui/Dropdown.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import FinderPage from "./FinderPage.svelte";

  let { finder }: { finder: Finder } = $props();

  const DEFAULT: ServiceId = "interstellar_factors";
  let service = $state<ServiceId>(parseServiceRoute(location.hash) ?? DEFAULT);
  const label = $derived(SERVICES.find((s) => s.id === service)?.label ?? service);

  /** This finder is the page shown (a hash change may be leaving it). */
  const here = () => showing("services");

  $effect(() => {
    const onHash = () => {
      if (here()) service = parseServiceRoute(location.hash) ?? DEFAULT;
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  // The address always names the service, so back and the tile reopen it.
  $effect(() => {
    const want = `#/${serviceRoute(service)}`;
    if (here() && location.hash !== want) history.replaceState(null, "", want);
  });

  function pick(id: ServiceId) {
    location.hash = `#/${serviceRoute(id)}`;
  }

  // A new function per service, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const s = service;
    return async (origin: Origin, filters: Filters) => toResults(await stations({ kind: "service", service: s }, origin, filters));
  });
</script>

<FinderPage {finder}>
  {#snippet input()}
    <Dropdown aria-label={finder.input} value={service} onchange={(e) => pick(e.currentTarget.value as ServiceId)}>
      {#each SERVICES as s (s.id)}<option value={s.id}>{s.label}</option>{/each}
    </Dropdown>
  {/snippet}
  <SearchKit
    lookup="stations.service"
    offers={["pad", "carriers", "odyssey", "age", "arrival"]}
    {run}
    title="Nearest"
    noun="stations"
    busyText="Asking Spansh…"
    onanswer={(top) => rememberAnswer(finder.id, top && { ...top, context: label, route: serviceRoute(service) })}
  />
</FinderPage>
