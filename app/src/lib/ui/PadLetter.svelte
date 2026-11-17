<script lang="ts">
  // DESIGN.md's pad letter: a station's largest pad as a small outlined letter in Label size,
  // Prose Grey. One too small for the ship is Red Low, outline and letter, and struck through,
  // with the reason as its accessible name (the list of colours that carry a meaning: the
  // strike is the cue that is not colour).
  import type { Pad } from "../search";

  let { pad, unfit = false }: { pad: Pad; /** Too small for the commander's ship. */ unfit?: boolean } = $props();

  const NAMES = { S: "Small", M: "Medium", L: "Large" } as const;
</script>

<span class="type-label pad" class:unfit role="img" aria-label={unfit ? `${NAMES[pad]} pad: too small for your ship` : `${NAMES[pad]} pad`}>{pad}</span>

<style>
  .pad { display: inline-flex; align-items: center; flex: none; height: 1.125rem; padding: 0 var(--sp-1); border: 1px solid var(--line); color: var(--prose); }
  .pad.unfit { color: var(--red-low); border-color: var(--red-low); text-decoration: line-through; }
</style>
