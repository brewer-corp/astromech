<script lang="ts">
  // The Tip: an instant custom tooltip that replaces native title tooltips. A raised hull panel
  // with a label-orange border and a pointer, page-size text. Explanations live here instead of
  // in footnote paragraphs. In the mockups it is drawn open, placed against its positioned parent;
  // `end` hangs it to the left of the anchor with the pointer at its right end.
  import type { Snippet } from "svelte";

  let { side = "below", end = false, pos = "", width = 360, children }: { side?: "below" | "above" | "right"; end?: boolean; pos?: string; width?: number; children: Snippet } =
    $props();
</script>

<div class="tip {side}" class:end role="tooltip" style="max-width: {width}px; {pos}">
  {@render children()}
</div>

<style>
  .tip {
    position: absolute; z-index: 20; width: max-content; padding: var(--sp-3) var(--sp-4);
    background: var(--surface-hi); border: 1px solid var(--orange-mid);
    font-size: var(--fs); font-weight: 500; line-height: 1.45; color: var(--prose); white-space: normal; text-align: left;
    letter-spacing: normal; text-transform: none;
  }
  .tip :global(b) { color: var(--accent); font-weight: 600; }
  /* The pointer: an outer triangle in the border colour, an inner one in the panel's. */
  .tip::before, .tip::after { content: ""; position: absolute; border: solid transparent; }
  .below { top: calc(100% + 10px); left: -12px; }
  .below::before { left: 18px; bottom: 100%; border-width: 0 9px 9px; border-bottom-color: var(--orange-mid); }
  .below::after { left: 19px; bottom: 100%; border-width: 0 8px 8px; border-bottom-color: var(--surface-hi); }
  .above { bottom: calc(100% + 10px); left: -12px; }
  .above::before { left: 18px; top: 100%; border-width: 9px 9px 0; border-top-color: var(--orange-mid); }
  .above::after { left: 19px; top: 100%; border-width: 8px 8px 0; border-top-color: var(--surface-hi); }
  /* end: the box hangs left from its anchor, with the pointer near its right end. */
  .below.end, .above.end { left: auto; right: -12px; }
  .below.end::before, .above.end::before { left: auto; right: 18px; }
  .below.end::after, .above.end::after { left: auto; right: 19px; }
  .right { left: calc(100% + 12px); top: -10px; }
  .right::before { top: 14px; right: 100%; border-width: 9px 9px 9px 0; border-right-color: var(--orange-mid); }
  .right::after { top: 15px; right: 100%; border-width: 8px 8px 8px 0; border-right-color: var(--surface-hi); }
</style>
