<script lang="ts">
  // DESIGN.md icons: one stroke, 16 px grid, 1 px, currentColor, no fill, square ends.
  // copy matches lib/ui/Glyph.svelte; the others are proposed additions to it (astromech-pey5).
  let { name }: { name: "copy" | "add-stop" | "destination" | "external" | "chevron" } = $props();
</script>

<svg viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">
  {#if name === "copy"}
    <rect x="5.5" y="2.5" width="7" height="9" /><path d="M3.5 5.5h6v8h-6z" />
  {:else if name === "add-stop"}
    <circle cx="4.5" cy="3.5" r="1.5" /><circle cx="4.5" cy="12.5" r="1.5" /><path d="M4.5 5v6M10.5 8h5M13 5.5v5" />
  {:else if name === "destination"}
    <path d="M3.5 15V1.5" /><path d="M3.5 2h9l-2.5 3 2.5 3h-9" />
  {:else if name === "external"}
    <path d="M7 3.5H2.5v10h10V9" /><path d="M9.5 2.5h4v4M13.5 2.5L7.5 8.5" />
  {:else}
    <path d="M4 6l4 4 4-4" />
  {/if}
</svg>

<style>
  svg { fill: none; stroke: currentColor; stroke-width: 1; stroke-linecap: square; flex: none; display: block; }
</style>
