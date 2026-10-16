<script lang="ts">
  // Where my power wants work: nearby acquisition, reinforcement and undermining systems
  // (Spansh), on the search kit.
  import { loadTargets, targetDetail, TARGETS, type Target } from "../../lib/powerplay";
  import type { Filters, Origin, Results } from "../../lib/search";
  import SearchKit from "../../lib/ui/SearchKit.svelte";

  const RANGES = [20, 40, 80];
  let target = $state<Target>("acquire");
  let within = $state(40);
  const kind = $derived(TARGETS.find((t) => t.id === target));

  async function run(origin: Origin, filters: Filters): Promise<Results> {
    const w = within;
    const r = await loadTargets(target, origin, filters, w);
    return {
      origin: r.here.system,
      range: r.range,
      source: "Spansh",
      fetched: r.fetched,
      warning: r.warning,
      rows: r.hits.map((h) => ({ key: h.id64, system: h.name, ly: h.ly, jumps: h.jumps, exact: h.exact, updated_at: h.updated_at, detail: targetDetail(h, r.power) })),
    };
  }
</script>

<div class="acts">
  {#each TARGETS as t (t.id)}
    <button class:on={target === t.id} onclick={() => (target = t.id)}>{t.title}</button>
  {/each}
  <label>Within
    <select bind:value={within}>
      {#each RANGES as r (r)}<option value={r}>{r} ly</option>{/each}
    </select>
  </label>
</div>
<!-- The kit re-runs when the lookup changes, and when `within` does: `run` reads it before awaiting. -->
<SearchKit lookup={`powerplay.${target}`} offers={["age"]} {run} title="Nearest" busyText={kind?.busy ?? "Searching…"} />

<style>
  .acts { display: flex; flex-wrap: wrap; align-items: center; gap: 22px; text-transform: uppercase; letter-spacing: var(--caps); }
  button { all: unset; cursor: pointer; color: var(--text-lo); }
  button:hover { color: var(--text); }
  .on { color: var(--accent); }
  label { display: flex; align-items: center; gap: 8px; color: var(--text-lo); font-size: var(--fs-s); }
  select { all: unset; height: 26px; padding: 0 8px; background: var(--cell); color: var(--accent); cursor: pointer; text-transform: none; letter-spacing: normal; }
</style>
