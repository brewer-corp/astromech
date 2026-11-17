<script lang="ts">
  // DESIGN.md's Hexagons: an engineer rank or a blueprint grade as a row of the engineer
  // hexagon (app/public/engineers/access.svg, used as a mask so it takes the colour), one per
  // step. The states, by DESIGN.md's "Engineer rank and blueprint grade" row:
  // - reached (`n`: a rank held, a grade fitted, a grade the held rank gets today): the full mark, White;
  // - `plan`: the steps after `n` up to it, what a plan adds: the full mark, Orange;
  // - `offers`: the steps after `n` up to it, a grade the engineer offers that is neither reached
  //   nor planned: the ring alone without its centre (access-ring.svg), Label Grey;
  // - the rest, unreached: the full mark, Mark Off.
  // A row takes `plan` or `offers`, not both. `label` ("Rank 3 of 5") is its accessible name;
  // without one it is hidden from assistive tech, for a mark whose words sit beside it.
  let {
    n,
    of = 5,
    plan = 0,
    offers = 0,
    label,
  }: { n: number; of?: number; plan?: number; offers?: number; label?: string } = $props();
</script>

<span class="hexes" role={label ? "img" : undefined} aria-label={label} aria-hidden={label ? undefined : "true"}>
  {#each Array.from({ length: of }, (_, i) => i) as i (i)}<span
      class="hex"
      class:on={i < n}
      class:plan={i >= n && i < plan}
      class:offer={i >= n && i < offers}
    ></span>{/each}
</span>

<style>
  .hexes { display: inline-flex; align-items: center; gap: var(--gap); flex: none; vertical-align: middle; }
  .hex { width: 0.75rem; height: 0.875rem; background: var(--mark-off); -webkit-mask: url("/engineers/access.svg") center / contain no-repeat; mask: url("/engineers/access.svg") center / contain no-repeat; }
  .hex.on { background: var(--text-hi); }
  .hex.plan { background: var(--accent); }
  .hex.offer { background: var(--label-grey); -webkit-mask-image: url("/engineers/access-ring.svg"); mask-image: url("/engineers/access-ring.svg"); }
</style>
