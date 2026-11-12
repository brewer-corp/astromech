<script lang="ts">
  // Content and its 320 px side panel after a 1 px orange line (DESIGN.md Side panel), shared by
  // Board and ListDetail. The panel sits right after the content. Where the content can't keep `min` beside the panel (the 1100 x 700
  // minimum), the panel moves below it; text never shrinks to fit. A new `selected` brings a
  // panel that sits below the content into view, since it may be off screen.
  import type { Snippet } from "svelte";
  import { stacks } from "./layout";

  let {
    children,
    panel,
    selected = null,
    min,
    label = "Details",
  }: {
    children: Snippet;
    /** The side panel's contents; none, no panel. */
    panel?: Snippet | null;
    /** What the panel describes, by key. */
    selected?: unknown;
    /** The narrowest the content works at beside the panel. */
    min: number;
    /** The panel's accessible name. */
    label?: string;
  } = $props();

  let width = $state(0);
  let aside = $state<HTMLElement>();
  const stacked = $derived(stacks(width, min));

  /** The selection last brought into view, so it is scrolled to once, not on every resize. */
  let shown: unknown = null;
  $effect(() => {
    // The panel may only now be mounted (the first selection in a list), so this waits on it.
    if (selected == null || !aside || !stacked) return void (selected == null && (shown = null));
    if (selected === shown) return;
    shown = selected;
    aside.scrollIntoView({ block: "nearest" });
  });
</script>

<div class="split" class:side={!!panel && !stacked} bind:clientWidth={width}>
  <div class="content">{@render children()}</div>
  {#if panel}
    <aside class:stacked bind:this={aside} aria-label={label}>{@render panel()}</aside>
  {/if}
</div>

<style>
  .split { display: grid; grid-template-columns: minmax(0, 1fr); align-items: start; column-gap: var(--sp-12); padding: var(--sp-6) 0 var(--sp-12); }
  /* The panel follows the content, as wide as its tiles or columns; spare width stays empty after
     the panel, so on a big window the panel is never a screen away from what it describes. */
  .split.side { grid-template-columns: minmax(0, max-content) calc(var(--panel) + var(--sp-6) + 1px); justify-content: start; }
  .content { min-width: 0; }
  aside { position: sticky; top: var(--sp-6); min-width: 0; padding-left: var(--sp-6); border-left: 1px solid var(--accent); }
  aside.stacked { position: static; margin-top: var(--sp-8); padding: var(--sp-6) 0 0; border-left: 0; border-top: 1px solid var(--accent); }
</style>
