<script lang="ts">
  // DESIGN.md's Hexagons: an engineer rank or a blueprint grade as a row of the engineer
  // hexagon (app/public/engineers/access.svg, used as a mask so it takes the colour), one per
  // step, the reached ones lit. The colour is by the list in DESIGN.md: a reached rank White
  // (`rank`), a grade the commander can get Orange (`grade`), a grade not reached Orange Mid
  // (`plan`); the unlit steps Mark Off. `label` ("Rank 3 of 5") is its accessible name; without
  // one it is hidden from assistive tech, for a mark whose words sit beside it. `to` lights the steps
  // after `n` up to it in Orange Mid: a grade the engineer reaches that the commander can't get yet.
  let {
    n,
    of = 5,
    to = 0,
    tone = "rank",
    label,
  }: { n: number; of?: number; to?: number; tone?: "rank" | "grade" | "plan"; label?: string } = $props();
</script>

<span class="hexes {tone}" role={label ? "img" : undefined} aria-label={label} aria-hidden={label ? undefined : "true"}>
  {#each Array.from({ length: of }, (_, i) => i) as i (i)}<span class="hex" class:on={i < n} class:to={i >= n && i < to}></span>{/each}
</span>

<style>
  .hexes { display: inline-flex; align-items: center; gap: var(--gap); flex: none; vertical-align: middle; }
  .hex { width: 0.75rem; height: 0.875rem; background: var(--mark-off); -webkit-mask: url("/engineers/access.svg") center / contain no-repeat; mask: url("/engineers/access.svg") center / contain no-repeat; }
  .rank .hex.on { background: var(--text-hi); }
  .grade .hex.on { background: var(--accent); }
  .plan .hex.on, .hex.to { background: var(--orange-mid); }
</style>
