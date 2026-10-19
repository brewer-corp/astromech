<script lang="ts">
  // A short set of options as rank cells (pad size, class, mount). A select is the wrong control
  // when every choice fits in one row. `mark` is a HUD glyph on that cell (the ship you are flying).
  import Glyph, { type GlyphName } from "./Glyph.svelte";

  export type SegOpt = { value: string; label: string; title?: string; mark?: GlyphName };

  let {
    label,
    value,
    options,
    onchange,
  }: {
    label: string;
    value: string;
    options: SegOpt[];
    onchange: (v: string) => void;
  } = $props();
</script>

<div class="seg" role="group" aria-label={label}>
  <span class="lbl">{label}</span>
  <div class="opts">
    {#each options as o (o.value)}
      <button
        type="button"
        class:on={value === o.value}
        aria-pressed={value === o.value}
        aria-label={o.mark ? `${label} ${o.label}, current ship` : `${label} ${o.label}`}
        title={o.title}
        onclick={() => onchange(o.value)}
      >{#if o.mark}<Glyph name={o.mark} />{/if}{o.label}</button>
    {/each}
  </div>
</div>

<style>
  .seg { display: flex; align-items: center; gap: 10px; min-width: 0; }
  .lbl { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); flex: none; min-width: 5.5em; }
  .opts { display: flex; flex-wrap: wrap; gap: var(--gap); min-width: 0; }
  button { all: unset; cursor: pointer; box-sizing: border-box; display: inline-flex; align-items: center; justify-content: center; gap: 4px; min-width: 2.2em; height: 30px; padding: 0 10px; background: var(--cell-opt); color: var(--text); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; text-align: center; }
  button:hover { color: var(--text-hi); }
  button.on { background: var(--cell); color: var(--accent); }
</style>
