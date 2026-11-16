<script lang="ts">
  // Engineer rank and blueprint grade as Elite draws them: a row of hexagons, the reached ones
  // lit. The mark is the app's own (app/public/engineers/access.svg, CMDR Robert Phillips'
  // engineer icon from EDAssets, credited in NOTICE.md), used as a mask so it takes currentColor,
  // as modules/unlocks/View.svelte does.
  let { n, of = 5, to = 0, size = 12, label }: { n: number; of?: number; to?: number; size?: number; label?: string } = $props();
</script>

<span class="hexes" role="img" aria-label={label ?? `${n} of ${of}`} style="--s: {size}px">
  {#each Array.from({ length: of }, (_, i) => i) as i (i)}<span class="hex" class:on={i < n} class:plan={i >= n && i < to}></span>{/each}
</span>

<style>
  .hexes { display: inline-flex; align-items: center; gap: 3px; flex: none; }
  .hex { width: var(--s); height: calc(var(--s) * 1.15); background: currentColor; opacity: 1;
    -webkit-mask: url("/engineers/access.svg") center / contain no-repeat; mask: url("/engineers/access.svg") center / contain no-repeat; }
  .hex:not(.on) { color: #4f4a45; }
  /* A planned grade, not yet reached: Orange Mid. */
  .hex.plan { color: #d97a34; }
</style>
