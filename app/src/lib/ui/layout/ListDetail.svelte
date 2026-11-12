<script lang="ts">
  // DESIGN.md's List with detail: a list (rows or a tree) and the side panel, which describes the
  // selected item. The page passes the panel, or a list inside fills it (useDetail). The list
  // keeps the width of its columns; spare width stays empty.
  import type { Snippet } from "svelte";
  import { provideDetail } from "./listDetail.svelte";
  import Split from "./Split.svelte";

  let {
    children,
    panel = null,
    selected = null,
    min = 480,
  }: {
    children: Snippet;
    /** The selected item's details, when the page holds the selection. */
    panel?: Snippet | null;
    selected?: unknown;
    /** The narrowest the list works at beside the panel, unless the list inside says. */
    min?: number;
  } = $props();

  const detail = provideDetail();
</script>

<Split panel={panel ?? detail.panel} selected={selected ?? detail.selected} min={detail.min ?? min}>{@render children()}</Split>
