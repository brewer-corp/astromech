<script lang="ts">
  // DESIGN.md's meter: a Mark Off track filled from the left in Prose Grey, as wide as what holds
  // it. No label inside; the figure beside it says how much. `rest` draws the unfilled part in
  // Label Grey instead, for a bar split between two figures (reinforcement against undermining),
  // where the figures under it say which side is which. It is hidden from screen readers: the
  // figure beside it carries the words, and any Tip goes on that figure.
  import type { HTMLAttributes } from "svelte/elements";

  let {
    value,
    thin = false,
    rest = false,
    ...attrs
  }: HTMLAttributes<HTMLSpanElement> & {
    /** The filled share, 0 to 1; outside that it is clamped. */
    value: number;
    /** 2 px rather than 6 px. */
    thin?: boolean;
    /** The unfilled part is Label Grey: one bar of two parts. */
    rest?: boolean;
  } = $props();

  const at = $derived(`${Math.max(0, Math.min(1, Number.isFinite(value) ? value : 0)) * 100}%`);
</script>

<span class="meter" class:thin class:rest style:--at={at} aria-hidden="true" {...attrs}></span>

<style>
  .meter { display: block; height: 0.375rem; background: linear-gradient(90deg, var(--prose) var(--at), var(--mark-off) var(--at)); }
  .thin { height: 0.125rem; }
  .rest { background: linear-gradient(90deg, var(--prose) var(--at), var(--label-grey) var(--at)); }
</style>
