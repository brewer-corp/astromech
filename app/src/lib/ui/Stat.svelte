<script lang="ts">
  // DESIGN.md's stat tile: a read-only Hull tile with one main value. Its label in Label, the value
  // in Value XL, Orange Lit, with its unit after it in Label, Orange Mid ("1.2 M cr"), and at most
  // one line of context below in Label. `go` marks a tile the commander should act on now: an
  // Orange outline and the value in Orange. A Tip attaches as on any element:
  // `<Stat label="Earned" value="1.2" unit="M cr" {@attach tip("…")} />`.
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  let {
    label,
    value,
    unit = "",
    go = false,
    children,
    ...rest
  }: HTMLAttributes<HTMLDivElement> & { label: string; value: string | number; unit?: string; go?: boolean; children?: Snippet } = $props();
</script>

<div class="stat" class:go {...rest}>
  <span class="type-label">{label}</span>
  <!-- A real space before the unit, so "9 t" reads as one phrase to a screen reader. -->
  <span><span class="type-value-xl v">{value}</span>{#if unit}<span class="type-label">{` ${unit}`}</span>{/if}</span>
  {#if children}<span class="type-label">{@render children()}</span>{/if}
</div>

<style>
  .stat { display: flex; flex-direction: column; gap: var(--sp-2); min-width: 176px; padding: 10px var(--sp-3); background: var(--surface); }
  .go { outline: 1px solid var(--accent); }
  .go .v { color: var(--accent); }
</style>
