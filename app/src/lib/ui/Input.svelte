<script lang="ts">
  // DESIGN.md's one input: an ember cell, 34 px, Body text in Orange, the placeholder in
  // Placeholder (Orange at 75 %), no border or underline at rest, a 1 px orange outline on focus.
  // `rows` makes it a multi-line box. `end` sits inside the cell at its right (a clear ×, the
  // "Ctrl+K" hint, a unit); `children` is anchored under the cell, for a suggestion popover.
  // An error puts an alarm "!" and plain words directly under it. Name it with `aria-label` or
  // a <label> around it; the placeholder is never the only label.
  import type { Snippet } from "svelte";
  import type { HTMLInputAttributes } from "svelte/elements";

  let {
    value = $bindable(),
    rows,
    error,
    end,
    children,
    class: cls = "",
    input = $bindable(),
    ...rest
  }: Omit<HTMLInputAttributes, "value" | "children"> & {
    value?: string | number | null;
    /** A multi-line box of this many rows. */
    rows?: number;
    error?: string | null;
    end?: Snippet;
    children?: Snippet;
    class?: string;
    /** The <input> or <textarea>, for focus(). */
    input?: HTMLInputElement | HTMLTextAreaElement;
  } = $props();
</script>

<span class="field {cls}">
  <span class="cell" class:multi={rows}>
    {#if rows}
      <textarea bind:this={input} bind:value {rows} {...rest as Record<string, unknown>}></textarea>
    {:else}
      <input bind:this={input} bind:value {...rest} />
    {/if}
    {#if end}<span class="end">{@render end()}</span>{/if}
  </span>
  {#if children}{@render children()}{/if}
  {#if error}<span class="error" role="alert"><span class="bang">!</span>{error}</span>{/if}
</span>

<style>
  .field { position: relative; display: flex; flex-direction: column; min-width: 0; }
  .cell { display: flex; align-items: center; min-height: var(--row); background: var(--cell); color: var(--accent); }
  .cell:hover { background: var(--cell-hover); }
  .cell:focus-within { outline: 1px solid var(--accent); outline-offset: 2px; }
  input, textarea { all: unset; box-sizing: border-box; flex: 1; min-width: 0; width: 100%; padding: 0 var(--sp-3); font-size: var(--fs); color: var(--accent); }
  input { height: var(--row); }
  textarea { padding: var(--sp-2) var(--sp-3); line-height: 1.45; white-space: pre-wrap; resize: vertical; }
  .multi { align-items: stretch; }
  input::placeholder, textarea::placeholder { color: var(--placeholder); opacity: 1; }
  /* The cell draws focus; the field inside doesn't draw a second outline. */
  input:focus-visible, textarea:focus-visible { outline: none; }
  .end { display: flex; align-items: center; gap: var(--sp-2); padding-right: var(--sp-3); flex: none; font-size: var(--fs-s); }
  .error { display: flex; gap: var(--sp-1); margin-top: var(--sp-1); font-size: var(--fs-s); color: var(--alarm); }
  .bang { font-weight: 600; }
</style>
