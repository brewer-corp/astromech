<script lang="ts">
  // DESIGN.md's badge: a fact with no column of its own ("Scoopable", "Permit", "Odyssey only").
  // Label type, mixed case, Orange Mid, a 1 px Seam outline, 20 px high, never filled (a fill would
  // read as a control or a selection).
  import type { Snippet } from "svelte";

  let { title, children }: { title?: string; children: Snippet } = $props();
</script>

<span class="badge" {title}>{@render children()}</span>

<style>
  .badge { display: inline-flex; align-items: center; flex: none; height: 20px; padding: 0 6px; border: 1px solid var(--line); color: var(--orange-mid); font-size: var(--fs-s); font-weight: 400; line-height: 1; letter-spacing: normal; text-transform: none; white-space: nowrap; vertical-align: middle; }
</style>
