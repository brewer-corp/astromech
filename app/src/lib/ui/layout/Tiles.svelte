<script lang="ts">
  // A Board's tile grid: columns of 200–240 px with 8 px gaps (DESIGN.md Page layouts). `cols`
  // fixes the column count (a grid of grades), with `heads` a leading column for row heads;
  // without it, as many columns as fit. At the 1100 x 700 minimum a wide fixed grid's tiles may
  // narrow to 160 px rather than run off the page.
  import type { Snippet } from "svelte";

  let { children, cols, heads = false, label }: { children: Snippet; cols?: number; heads?: boolean; label?: string } = $props();
</script>

<div class="tiles" class:fixed={cols != null} class:heads style:--n={cols} role={label ? "group" : undefined} aria-label={label}>{@render children()}</div>

<style>
  .tiles { display: grid; gap: var(--sp-2); grid-template-columns: repeat(auto-fill, minmax(200px, 240px)); align-content: start; }
  .fixed { grid-template-columns: repeat(var(--n), minmax(160px, 240px)); }
  .fixed.heads { grid-template-columns: max-content repeat(var(--n), minmax(160px, 240px)); }
</style>
