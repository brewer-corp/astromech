<script lang="ts">
  // A Board's tile grid: columns of 200–240 px with 8 px gaps (DESIGN.md Page layouts). `cols`
  // fixes the column count (a grid of grades), with `heads` a leading column for row heads;
  // without it, as many columns as fit. At the 1100 x 700 minimum a wide fixed grid's tiles may
  // narrow to 160 px rather than run off the page. `wide` makes each tile a card two tracks wide
  // (408–488 px), for a card that holds lines of its own (Biology's body cards).
  import type { Snippet } from "svelte";

  let {
    children,
    cols,
    heads = false,
    label,
    list = false,
    wide = false,
  }: {
    children: Snippet;
    cols?: number;
    heads?: boolean;
    label?: string;
    /** Its tiles are list items (role listitem). */
    list?: boolean;
    /** Each tile is two tracks and the gap between them wide. */
    wide?: boolean;
  } = $props();
</script>

<div class="tiles" class:fixed={cols != null} class:heads class:wide style:--n={cols} role={list ? "list" : label ? "group" : undefined} aria-label={label}>{@render children()}</div>

<style>
  .tiles { display: grid; gap: var(--sp-2); grid-template-columns: repeat(auto-fill, minmax(12.5rem, 15rem)); align-content: start; }
  .wide { grid-template-columns: repeat(auto-fill, minmax(25.5rem, 30.5rem)); }
  .fixed { grid-template-columns: repeat(var(--n), minmax(10rem, 15rem)); }
  .fixed.heads { grid-template-columns: max-content repeat(var(--n), minmax(10rem, 15rem)); }
</style>
