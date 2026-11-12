<script lang="ts">
  // DESIGN.md's Workbench: the inputs on the left, the result on the right, which changes as the
  // inputs do. The inputs keep one column; the result keeps to a readable width and spare width
  // stays empty. Where the result can't keep `min` beside the inputs, it goes below them.
  import type { Snippet } from "svelte";
  import { GUTTER } from "./layout";

  let { inputs, children, min = 560 }: { inputs: Snippet; children: Snippet; /** The narrowest the result works at. */ min?: number } = $props();

  /** The inputs' column. */
  const INPUTS = 400;
  let width = $state(0);
  const stacked = $derived(width > 0 && width < INPUTS + GUTTER + min);
</script>

<div class="bench" class:stacked bind:clientWidth={width}>
  <div class="inputs">{@render inputs()}</div>
  <div class="result">{@render children()}</div>
</div>

<style>
  .bench { display: grid; grid-template-columns: 400px minmax(0, 1000px); align-items: start; column-gap: var(--sp-12); padding: var(--sp-6) 0 var(--sp-12); }
  .bench.stacked { grid-template-columns: minmax(0, 1fr); row-gap: var(--sp-8); }
  .inputs, .result { min-width: 0; }
</style>
