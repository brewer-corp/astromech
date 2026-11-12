<script lang="ts">
  // Where my power wants work: nearby acquisition, reinforcement and undermining systems
  // (Spansh), on the search kit.
  import Choice from "../../lib/ui/Choice.svelte";
  import Dropdown from "../../lib/ui/Dropdown.svelte";
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
      place: "system",
      columns: [{ key: "power", head: "Powerplay", text: true }],
      rows: r.hits.map((h) => ({ key: h.id64, system: h.name, ly: h.ly, jumps: h.jumps, exact: h.exact, updated_at: h.updated_at, facts: { power: targetDetail(h, r.power) } })),
    };
  }
</script>

<div class="acts">
  <!-- Which work is a choice in the section, not a tab row (DESIGN.md Choice). -->
  <Choice label="Work" options={TARGETS.map((t) => ({ value: t.id, label: t.title }))} value={target} onchange={(v) => (target = v)} />
</div>
<!-- The kit re-runs when the lookup changes, and when `within` does: `run` reads it before awaiting.
     The radius is among the filters, and the summary always names it ("Within 40 ly of Sol"). -->
<SearchKit lookup={`powerplay.${target}`} offers={["age"]} {run} title={`Within ${within} ly`} joiner="of" noun="systems" busyText={kind?.busy ?? "Searching…"}>
  {#snippet more()}
    <label>Within
      <Dropdown bind:value={within}>
        {#each RANGES as r (r)}<option value={r}>{r} ly</option>{/each}
      </Dropdown>
    </label>
  {/snippet}
</SearchKit>

<style>
  .acts { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-4); }
  label { display: flex; align-items: center; gap: 8px; color: var(--orange-mid); font-size: var(--fs-s); }
</style>
