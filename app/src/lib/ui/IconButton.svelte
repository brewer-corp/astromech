<script lang="ts">
  // DESIGN.md's icon button: a 28 px square holding one icon, Orange Mid at rest, Orange Lit on
  // hover, with a tooltip and an accessible label always. `label` names the action ("Add
  // Diaguandri as a stop") and is the tooltip unless `tip` says more. A row that holds icon
  // buttons lights them while it is hovered (`.ibtn` on the row's :hover).
  import type { HTMLButtonAttributes } from "svelte/elements";
  import Glyph, { type GlyphName } from "./Glyph.svelte";

  let {
    icon,
    label,
    tip,
    type = "button",
    class: cls = "",
    ...rest
  }: Omit<HTMLButtonAttributes, "children"> & { icon: GlyphName; label: string; tip?: string; class?: string } = $props();
</script>

<button class="ibtn {cls}" {type} aria-label={label} title={tip ?? label} {...rest}><Glyph name={icon} /></button>

<style>
  .ibtn { all: unset; box-sizing: border-box; cursor: pointer; display: inline-flex; align-items: center; justify-content: center; flex: none; width: 28px; height: 28px; color: var(--orange-mid); }
  .ibtn:hover { color: var(--orange-lit); }
  .ibtn:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .ibtn:disabled { color: var(--orange-dim); cursor: default; }
</style>
