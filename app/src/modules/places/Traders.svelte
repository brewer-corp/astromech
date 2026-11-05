<script lang="ts">
  // The Material traders finder: the Services lookup on the raw, manufactured and encoded trader
  // services (or any of them). The main input is the trader kind;
  // #/places/material-traders/<kind> keeps it on back and reload.
  import { TRADERS, parsePlaces, parseTraderRoute, rememberAnswer, traderRoute, type Finder, type TraderKind } from "../../lib/places";
  import type { Filters, Origin } from "../../lib/search";
  import { stations, toResults } from "../../lib/stations";
  import Choice from "../../lib/ui/Choice.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import FinderPage from "./FinderPage.svelte";

  let { finder }: { finder: Finder } = $props();

  const DEFAULT: TraderKind = "raw";
  let kind = $state<TraderKind>(parseTraderRoute(location.hash) ?? DEFAULT);
  const trader = $derived(TRADERS.find((t) => t.kind === kind) ?? TRADERS[0]);
  const options = TRADERS.map((t) => ({ value: t.kind, label: t.label, ariaLabel: t.name }));

  /** This finder is the page shown (a hash change may be leaving it). */
  const here = () => parsePlaces(location.hash)?.finder?.id === finder.id;

  $effect(() => {
    const onHash = () => {
      if (here()) kind = parseTraderRoute(location.hash) ?? DEFAULT;
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  // The address always names the kind, so back and the tile reopen it.
  $effect(() => {
    const want = `#/${traderRoute(kind)}`;
    if (here() && location.hash !== want) history.replaceState(null, "", want);
  });

  // A new function per kind, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const service = trader.service;
    return async (origin: Origin, filters: Filters) => toResults(await stations({ kind: "service", service }, origin, filters));
  });
</script>

<FinderPage {finder}>
  {#snippet input()}
    <Choice label={finder.input} value={kind} {options} onchange={(k) => (location.hash = `#/${traderRoute(k as TraderKind)}`)} />
  {/snippet}
  <SearchKit
    lookup="stations.service"
    offers={["pad", "carriers", "odyssey", "age", "arrival"]}
    {run}
    title="Nearest"
    noun="traders"
    busyText="Asking Spansh…"
    onanswer={(top) => rememberAnswer(finder.id, top && { ...top, context: trader.name, route: traderRoute(kind) })}
  />
</FinderPage>
