<script lang="ts">
  // The Mining hotspots finder (decision astromech-bi94): rings with a hotspot of a mineral,
  // most hotspots first, within the radius of here, the route's next stop or a typed system. The
  // main input is the mineral; the ring type and the least reserve are choices. The core asks
  // Spansh for the highest hotspot counts first and picks the ring type itself; the summary says
  // when its requests ran out before the whole radius was ranked.
  // #/places/mining-hotspots/<ore>[?ring=&reserve=] keeps all three on back and reload.
  import { ORES, RING_TYPES, hotspotResults, hotspots, oreNamed, wantNotes, type Reserve, type RingType, type Want } from "../../lib/hotspots";
  import { hotspotRoute, parseHotspotRoute, rememberAnswer, showing, type Finder } from "../../lib/places";
  import type { Filters, Origin } from "../../lib/search";
  import Choice from "../../lib/ui/Choice.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import FinderPage from "./FinderPage.svelte";
  import MainInput from "./MainInput.svelte";

  let { finder }: { finder: Finder } = $props();

  /** The search radius when none is picked (astromech_market::mining::DEFAULT_RADIUS_LY). */
  const RADIUS = 100;
  const DEFAULT: Want = { ore: "painite", ring_type: null, reserve: null };
  let want = $state<Want>(parseHotspotRoute(location.hash) ?? DEFAULT);
  const name = $derived(oreNamed(want.ore)?.name ?? want.ore);
  const items = ORES.map((o) => ({ label: o.name }));
  const ANY = "any";
  const rings = [...RING_TYPES.map((t) => ({ value: t, label: t })), { value: ANY, label: "Any", ariaLabel: "Any ring type" }];
  const reserves = [
    { value: "pristine", label: "Pristine", title: "Pristine reserves only" },
    { value: "major", label: "Major", title: "Major or pristine reserves" },
    { value: ANY, label: "Any", ariaLabel: "Any reserve" },
  ];

  /** This finder is the page shown (a hash change may be leaving it). */
  const here = () => showing("mining-hotspots");

  $effect(() => {
    const onHash = () => {
      if (here()) want = parseHotspotRoute(location.hash) ?? DEFAULT;
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  // The address always names the mineral and choices, so back and the tile reopen them.
  $effect(() => {
    const to = `#/${hotspotRoute(want)}`;
    if (here() && location.hash !== to) history.replaceState(null, "", to);
  });

  /** A new mineral or choice: searched at once (a second click before the hash change lands keeps the first). */
  function go(w: Want) {
    want = w;
    location.hash = `#/${hotspotRoute(w)}`;
  }

  // A new function per search, so the kit searches again when the mineral or a choice changes.
  const run = $derived.by(() => {
    const w = $state.snapshot(want);
    return async (origin: Origin, filters: Filters) => hotspotResults(await hotspots(w, origin, filters));
  });
</script>

<FinderPage {finder}>
  {#snippet input()}
    <MainInput label={finder.input} current={name} {items} placeholder="Type a mineral" onpick={(n) => go({ ...want, ore: ORES.find((o) => o.name === n)?.symbol ?? want.ore })} />
  {/snippet}
  <SearchKit
    lookup="mining.hotspots"
    offers={["distance"]}
    hints={{ distance: RADIUS }}
    around
    notes={wantNotes(want)}
    {run}
    title="Nearest"
    noun="rings"
    busyText="Asking Spansh…"
    onanswer={(top) => rememberAnswer(finder.id, top && { figure: top.figure, unit: top.figure === "1" ? "hotspot" : "hotspots", context: name, route: hotspotRoute(want) })}
  >
    {#snippet lead()}
      <div class="choices">
        <span class="label">Ring</span>
        <Choice label="Ring type" value={want.ring_type ?? ANY} options={rings} onchange={(v) => go({ ...want, ring_type: v === ANY ? null : (v as RingType) })} />
        <span class="label">Reserve</span>
        <Choice label="Reserve" value={want.reserve ?? ANY} options={reserves} onchange={(v) => go({ ...want, reserve: v === ANY ? null : (v as Reserve) })} />
      </div>
    {/snippet}
  </SearchKit>
</FinderPage>

<style>
  .choices { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-3); }
  .label { font-size: var(--fs-s); color: var(--orange-mid); }
  .label:not(:first-child) { margin-left: var(--sp-4); }
</style>
