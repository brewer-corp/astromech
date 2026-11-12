<script lang="ts">
  // DESIGN.md's Board: blocks or tiles across the width, each with its own main value; where a
  // tile can be selected, the side panel describes it. Tiles go in a Tiles grid. Spare width
  // stays empty; it never stretches a tile.
  import type { Snippet } from "svelte";
  import Split from "./Split.svelte";

  let {
    children,
    panel = null,
    selected = null,
    min = 640,
  }: {
    children: Snippet;
    /** What the selected tile is; none, no panel. */
    panel?: Snippet | null;
    selected?: unknown;
    /** The narrowest the board works at beside the panel: tilesMin() of its widest grid. */
    min?: number;
  } = $props();
</script>

<Split {panel} {selected} {min}>{@render children()}</Split>
