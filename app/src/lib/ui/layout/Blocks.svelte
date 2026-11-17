<script lang="ts">
  // A Board's row of answer blocks (DESIGN.md Page layouts): each block one answer with its own
  // main value, read at a glance. Four across where four fit, else two, else one: never three
  // over one, so the blocks always read as a grid. A page hands it its blocks and sets no width.
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
</script>

<div class="room"><div class="blocks">{@render children()}</div></div>

<style>
  .room { container: blocks / inline-size; }
  .blocks { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--sp-8); align-items: start; }
  /* A block needs about 18rem (its value, the place it names and two icon buttons). */
  @container blocks (min-width: 38rem) {
    .blocks { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  }
  @container blocks (min-width: 78rem) {
    .blocks { grid-template-columns: repeat(4, minmax(0, 1fr)); }
  }
</style>
