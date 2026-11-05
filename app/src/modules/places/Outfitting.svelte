<script lang="ts">
  // The Outfitting finder: the nearest stations selling a module, on the search kit. The main
  // input is the module; its class, rating and mount narrow it. #/places/outfitting/<module>
  // [?class=&rating=&mount=] picks the module (the command palette links there), so back and
  // reload keep it.
  import { CATEGORIES, MODULES, choices, moduleRoute, parseModuleRoute, settle } from "../../lib/outfitting";
  import { parsePlaces, rememberAnswer, type Finder } from "../../lib/places";
  import type { Filters, Origin } from "../../lib/search";
  import { stations, toResults, type ModuleWant, type Mount } from "../../lib/stations";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import Choice, { type ChoiceOpt } from "../../lib/ui/Choice.svelte";
  import FinderPage from "./FinderPage.svelte";
  import MainInput from "./MainInput.svelte";

  let { finder }: { finder: Finder } = $props();

  const DEFAULT: ModuleWant = { name: "Frame Shift Drive", class: 5, rating: "A", mount: null };
  let want = $state<ModuleWant>(parseModuleRoute(location.hash) ?? DEFAULT);
  const opts = $derived(choices(want));
  const items = MODULES.map((m) => ({ label: m.name, detail: CATEGORIES.find((c) => c.id === m.category)?.title }));

  /** This finder is the page shown (a hash change may be leaving it). */
  const here = () => parsePlaces(location.hash)?.finder?.id === finder.id;

  $effect(() => {
    const onHash = () => {
      if (here()) want = parseModuleRoute(location.hash) ?? DEFAULT;
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  // The address always names the module, so back and the tile reopen it.
  $effect(() => {
    const to = `#/${moduleRoute(want)}`;
    if (here() && location.hash !== to) history.replaceState(null, "", to);
  });

  function pick(next: Partial<ModuleWant>) {
    want = settle({ ...want, ...next });
    location.hash = `#/${moduleRoute(want)}`;
  }

  /** "5A Gimballed Pulse Laser": the size and mount picked, then the module. */
  const named = $derived([`${want.class ?? ""}${want.rating ?? ""}`, want.mount ?? "", want.name].filter(Boolean).join(" "));

  // A new function per module, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const w = { kind: "module" as const, ...want };
    return async (origin: Origin, filters: Filters) => toResults(await stations(w, origin, filters));
  });

  /** "Any" and the values, each named with its field ("Class 5") for screen readers. */
  const withAny = (field: string, values: string[]): ChoiceOpt[] =>
    ["", ...values].map((v) => ({ value: v, label: v || "Any", ariaLabel: `${field} ${v || "Any"}` }));
  const classOpts = $derived(withAny("Class", opts.classes.map(String)));
  const ratingOpts = $derived(withAny("Rating", opts.ratings));
  const mountOpts = $derived(withAny("Mount", opts.mounts));
</script>

<FinderPage {finder}>
  {#snippet input()}
    <MainInput label={finder.input} current={want.name} {items} placeholder="Type a module" onpick={(name) => pick({ name, class: null, rating: null, mount: null })} />
  {/snippet}
  <SearchKit
    lookup="stations.module"
    offers={["pad", "carriers", "odyssey", "age", "arrival"]}
    {run}
    title="Nearest"
    noun="stations"
    busyText="Asking Spansh…"
    onanswer={(top) => rememberAnswer(finder.id, top && { ...top, context: named, route: moduleRoute(want) })}
  >
    {#snippet lead()}
      <div class="want">
        <div class="field"><span>Class</span><Choice label="Class" value={want.class != null ? String(want.class) : ""} options={classOpts} onchange={(v) => pick({ class: v ? Number(v) : null })} /></div>
        <div class="field"><span>Rating</span><Choice label="Rating" value={want.rating ?? ""} options={ratingOpts} onchange={(v) => pick({ rating: v || null })} /></div>
        {#if opts.mounts.length}
          <div class="field"><span>Mount</span><Choice label="Mount" value={want.mount ?? ""} options={mountOpts} onchange={(v) => pick({ mount: (v || null) as Mount | null })} /></div>
        {/if}
      </div>
    {/snippet}
  </SearchKit>
</FinderPage>

<style>
  .want { display: flex; flex-direction: column; gap: 10px; }
  .field { display: flex; align-items: center; gap: 10px; min-width: 0; }
  .field > span { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); flex: none; min-width: 5.5em; }
</style>
