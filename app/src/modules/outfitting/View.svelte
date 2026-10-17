<script lang="ts">
  // The nearest stations selling a module (by name, class, rating and mount), on the search kit.
  // #/outfitting/<module>[?class=&rating=&mount=] picks the module (the command palette links
  // there), so back and reload keep it.
  import { CATEGORIES, MODULES, choices, moduleRoute, parseModuleRoute, settle } from "../../lib/outfitting";
  import type { Filters, Origin } from "../../lib/search";
  import { stations, toResults, type ModuleWant, type Mount } from "../../lib/stations";
  import SearchKit from "../../lib/ui/SearchKit.svelte";

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

  const num = (v: string): number | null => (v ? Number(v) : null);

  // A new function per module, so the kit searches again when it changes.
  const run = $derived.by(() => {
    const w = { kind: "module" as const, ...want };
    return async (origin: Origin, filters: Filters) => toResults(await stations(w, origin, filters));
  });
</script>

<section>
  <div class="pick">
    <label>
      <span>Module</span>
      <select value={want.name} onchange={(e) => pick({ name: e.currentTarget.value })}>
        {#each CATEGORIES as c (c.id)}
          <optgroup label={c.title}>
            {#each MODULES.filter((m) => m.category === c.id) as m (m.name)}<option value={m.name}>{m.name}</option>{/each}
          </optgroup>
        {/each}
      </select>
    </label>
    <label>
      <span>Class</span>
      <select value={want.class ?? ""} onchange={(e) => pick({ class: num(e.currentTarget.value) })}>
        <option value="">Any</option>
        {#each opts.classes as c (c)}<option value={c}>{c}</option>{/each}
      </select>
    </label>
    <label>
      <span>Rating</span>
      <select value={want.rating ?? ""} onchange={(e) => pick({ rating: e.currentTarget.value || null })}>
        <option value="">Any</option>
        {#each opts.ratings as r (r)}<option value={r}>{r}</option>{/each}
      </select>
    </label>
    {#if opts.mounts.length}
      <label>
        <span>Mount</span>
        <select value={want.mount ?? ""} onchange={(e) => pick({ mount: (e.currentTarget.value || null) as Mount | null })}>
          <option value="">Any</option>
          {#each opts.mounts as m (m)}<option value={m}>{m}</option>{/each}
        </select>
      </label>
    {/if}
  </div>
  <SearchKit lookup="stations.module" offers={["pad", "carriers", "odyssey", "age", "arrival"]} {run} title="Nearest" busyText="Asking Spansh…" />
</section>

<style>
  section { padding: 22px 0 28px; max-width: 46rem; }
  .pick { display: flex; flex-direction: column; gap: 4px; }
  label { display: flex; align-items: center; gap: 8px; font-size: var(--fs-s); }
  label > span { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; }
  select { all: unset; flex: 1; height: 30px; padding: 0 8px; background: var(--cell); color: var(--accent); cursor: pointer; }
  optgroup, option { background: var(--bg, #111); color: var(--text); }
</style>
