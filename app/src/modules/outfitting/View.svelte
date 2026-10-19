<script lang="ts">
  // The nearest stations selling a module (by name, class, rating and mount), on the search kit.
  // #/outfitting/<module>[?class=&rating=&mount=] picks the module (the command palette links
  // there), so back and reload keep it.
  import { choices, moduleRoute, parseModuleRoute, settle } from "../../lib/outfitting";
  import type { Filters, Origin } from "../../lib/search";
  import { stations, toResults, type ModuleWant, type Mount } from "../../lib/stations";
  import FindWant from "../../lib/ui/FindWant.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import Seg from "../../lib/ui/Seg.svelte";

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

  const classOpts = $derived([{ value: "", label: "Any" }, ...opts.classes.map((c) => ({ value: String(c), label: String(c) }))]);
  const ratingOpts = $derived([{ value: "", label: "Any" }, ...opts.ratings.map((r) => ({ value: r, label: r }))]);
  const mountOpts = $derived([{ value: "", label: "Any" }, ...opts.mounts.map((m) => ({ value: m, label: m }))]);
</script>

<section>
  <SearchKit lookup="stations.module" offers={["pad", "carriers", "odyssey", "age", "arrival"]} {run} title="Nearest" busyText="Asking Spansh…">
    {#snippet lead()}
      <FindWant current={want.name} />
      <div class="want">
        <Seg label="Class" value={want.class != null ? String(want.class) : ""} options={classOpts} onchange={(v) => pick({ class: v ? Number(v) : null })} />
        <Seg label="Rating" value={want.rating ?? ""} options={ratingOpts} onchange={(v) => pick({ rating: v || null })} />
        {#if opts.mounts.length}
          <Seg label="Mount" value={want.mount ?? ""} options={mountOpts} onchange={(v) => pick({ mount: (v || null) as Mount | null })} />
        {/if}
      </div>
    {/snippet}
  </SearchKit>
</section>

<style>
  section { padding: 8px 0 32px; min-width: 0; }
  .want { display: flex; flex-direction: column; gap: 10px; }
</style>
