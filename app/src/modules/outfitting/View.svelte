<script lang="ts">
  // The nearest stations selling a module (by name, class, rating and mount), on the search kit.
  // #/outfitting/<module>[?class=&rating=&mount=] picks the module (the command palette links
  // there), so back and reload keep it.
  import { choices, moduleRoute, parseModuleRoute, settle } from "../../lib/outfitting";
  import type { Filters, Origin } from "../../lib/search";
  import { stations, toResults, type ModuleWant, type Mount } from "../../lib/stations";
  import FindWant from "../../lib/ui/FindWant.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import Choice, { type ChoiceOpt } from "../../lib/ui/Choice.svelte";

  const DEFAULT: ModuleWant = { name: "Frame Shift Drive", class: 5, rating: "A", mount: null };
  let want = $state<ModuleWant>(parseModuleRoute(location.hash) ?? DEFAULT);
  const opts = $derived(choices(want));

  $effect(() => {
    const onHash = () => (want = parseModuleRoute(location.hash) ?? DEFAULT);
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  function pick(next: Partial<ModuleWant>) {
    want = settle({ ...want, ...next });
    location.hash = `#/${moduleRoute(want)}`;
  }

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

<section>
  <SearchKit lookup="stations.module" offers={["pad", "carriers", "odyssey", "age", "arrival"]} {run} title="Nearest" busyText="Asking Spansh…">
    {#snippet lead()}
      <FindWant current={want.name} />
      <div class="want">
        <div class="field"><span>Class</span><Choice label="Class" value={want.class != null ? String(want.class) : ""} options={classOpts} onchange={(v) => pick({ class: v ? Number(v) : null })} /></div>
        <div class="field"><span>Rating</span><Choice label="Rating" value={want.rating ?? ""} options={ratingOpts} onchange={(v) => pick({ rating: v || null })} /></div>
        {#if opts.mounts.length}
          <div class="field"><span>Mount</span><Choice label="Mount" value={want.mount ?? ""} options={mountOpts} onchange={(v) => pick({ mount: (v || null) as Mount | null })} /></div>
        {/if}
      </div>
    {/snippet}
  </SearchKit>
</section>

<style>
  /* The route strip took over from the Galaxy rail; the page keeps its spacing and DESIGN.md's list width. */
  section { padding: 28px 0 72px; min-width: 0; max-width: var(--list-w); }
  .want { display: flex; flex-direction: column; gap: 10px; }
  .field { display: flex; align-items: center; gap: 10px; min-width: 0; }
  .field > span { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); flex: none; min-width: 5.5em; }
</style>
