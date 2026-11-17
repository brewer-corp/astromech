<script lang="ts">
  // A species' samples on Exploration > Biology as three short meters, one lit per sample taken
  // (DESIGN.md Meter: Prose Grey lit, Mark Off unlit), its words ("1 of 3") as the accessible
  // name. `small` draws them on a genus line; the sampling block draws them full size.
  let { count, small = false }: { count: number; small?: boolean } = $props();
  const lit = $derived(Math.max(0, Math.min(3, count)));
</script>

<span class="marks" class:small role="img" aria-label="{lit} of 3">
  {#each [0, 1, 2] as i (i)}<span class="mark" class:on={i < lit}></span>{/each}
</span>

<style>
  .marks { display: inline-flex; align-items: center; gap: var(--sp-1); }
  .mark { width: var(--sp-8); height: 0.375rem; background: var(--mark-off); }
  .small .mark { width: var(--sp-2); height: var(--sp-1); }
  .on { background: var(--prose); }
</style>
