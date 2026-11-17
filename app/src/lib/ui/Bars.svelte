<script lang="ts">
  // DESIGN.md's demand bars: the market's three bars rising left to right, the lit ones in the
  // demand's colour (Red Low short, Orange Mid tight, the text's own grey plenty) and the rest Mark
  // Off. With nothing to measure against (`depth` null) all three are lit in grey: no warning.
  // A mark beside the figure it qualifies; the row says the words.
  import { BARS, type Depth } from "../demand";

  let { depth }: { depth: Depth | null } = $props();
  const lit = $derived(depth ? BARS[depth] : 3);
</script>

<svg class="bars" viewBox="0 0 14 12" aria-hidden="true">
  {#each [0, 1, 2] as i (i)}<rect x={i * 5} y={8 - i * 4} width="3" height={4 + i * 4} class:on={i < lit} />{/each}
</svg>

<style>
  .bars { width: 0.875rem; height: 0.75rem; flex: none; }
  rect { fill: var(--mark-off); }
  rect.on { fill: currentColor; }
</style>
