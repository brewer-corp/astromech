<script lang="ts">
  // DESIGN.md's meter: a Mark Off track filled from the left in Prose Grey, as wide as what holds
  // it. No label inside; the figure beside it says how much. `rest` draws the unfilled part in
  // Label Grey instead, for a bar split between two figures (reinforcement against undermining),
  // where the figures under it say which side is which. `coming` draws a second part after the
  // fill in Orange Mid, for what is on its way and not yet counted (a colonisation delivery: aboard
  // and not yet delivered, DESIGN.md's list of colours that carry a meaning). It is hidden from screen readers: the
  // figure beside it carries the words, and any Tip goes on that figure.
  import type { HTMLAttributes } from "svelte/elements";

  let {
    value,
    thin = false,
    rest = false,
    coming = 0,
    ...attrs
  }: HTMLAttributes<HTMLSpanElement> & {
    /** The filled share, 0 to 1; outside that it is clamped. */
    value: number;
    /** 2 px rather than 6 px. */
    thin?: boolean;
    /** The unfilled part is Label Grey: one bar of two parts. */
    rest?: boolean;
    /** A share, 0 to 1, drawn after the fill in Orange Mid: on its way, not yet counted. */
    coming?: number;
  } = $props();

  const share = (v: number) => Math.max(0, Math.min(1, Number.isFinite(v) ? v : 0));
  const at = $derived(`${share(value) * 100}%`);
  const to = $derived(`${share(share(value) + share(coming)) * 100}%`);
</script>

<span class="meter" class:thin class:rest style:--at={at} style:--to={to} aria-hidden="true" {...attrs}></span>

<style>
  .meter { display: block; height: 0.375rem; background: linear-gradient(90deg, var(--prose) var(--at), var(--orange-mid) var(--at) var(--to), var(--mark-off) var(--to)); }
  .thin { height: 0.125rem; }
  .rest { background: linear-gradient(90deg, var(--prose) var(--at), var(--label-grey) var(--at)); }
</style>
