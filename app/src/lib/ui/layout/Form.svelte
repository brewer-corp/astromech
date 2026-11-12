<script lang="ts">
  // DESIGN.md's Form: Settings rows in one column, a 200 px label and a cell up to 700 px, 900 px
  // in all. Rows lay out on --form-label: `grid-template-columns: var(--form-label) minmax(0, 1fr)`.
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
</script>

<div class="form">{@render children()}</div>

<style>
  .form { --form-label: 200px; max-width: var(--form-w); padding: var(--sp-6) 0 var(--sp-12); }
</style>
