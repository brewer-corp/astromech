<script lang="ts" module>
  import type { GlyphName } from "./Glyph.svelte";

  /** One option. `mark` is a glyph before its label (the ship you are flying on its pad size);
   * `tip` explains it in a Tip. */
  export type ChoiceOpt = { value: string; label: string; tip?: string; mark?: GlyphName; ariaLabel?: string };
</script>

<script lang="ts" generics="V extends string | string[]">
  // DESIGN.md's choice: a row of ember cells, 2 px apart, in Label size. The chosen option is the
  // filled one (solid orange, On Orange text), never drawn inverted. A string value is a single
  // choice; an array is a multiple choice, where each chosen option is filled. Both look the same.
  // Use it when every option fits on one row; a longer list is a Dropdown.
  import Glyph from "./Glyph.svelte";
  import { tip } from "./tip";

  let {
    label,
    options,
    value,
    onchange,
    class: cls = "",
  }: {
    /** The group's accessible name. */
    label: string;
    options: ChoiceOpt[];
    value: V;
    onchange: (v: V) => void;
    class?: string;
  } = $props();

  const on = (v: string) => (Array.isArray(value) ? value.includes(v) : value === v);

  function pick(v: string) {
    if (Array.isArray(value)) onchange((on(v) ? value.filter((x) => x !== v) : [...value, v]) as V);
    else onchange(v as V);
  }
</script>

<span class="choices {cls}" role="group" aria-label={label}>
  {#each options as o (o.value)}
    <button type="button" class:on={on(o.value)} aria-pressed={on(o.value)} aria-label={o.ariaLabel} onclick={() => pick(o.value)} {@attach tip(o.tip)}
      >{#if o.mark}<Glyph name={o.mark} />{/if}{o.label}</button>
  {/each}
</span>

<style>
  .choices { display: inline-flex; flex-wrap: wrap; gap: var(--gap); min-width: 0; }
  button { all: unset; box-sizing: border-box; cursor: pointer; display: inline-flex; align-items: center; justify-content: center; gap: var(--sp-1); min-width: var(--row); height: var(--row); padding: 0 var(--sp-3); background: var(--cell); color: var(--accent); font-size: var(--fs-s); white-space: nowrap; }
  button:hover { background: var(--cell-hover); }
  button:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  button.on { background: var(--accent); color: var(--on-accent); }
  button.on:focus-visible { outline-color: var(--on-accent); outline-offset: -3px; }
</style>
