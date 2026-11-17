<script lang="ts">
  import type { Snippet } from "svelte";
  // "About this setting" panel, as in the game: a small label, white title, prose in grey.
  let { label = "About this setting", title, children }: { label?: string; title: string; children: Snippet } = $props();
</script>

<aside>
  <div class="lbl">{label}</div>
  <h2>{title}</h2>
  <div class="txt">{@render children()}</div>
</aside>

<style>
  aside { padding: 1.25rem 0 1.25rem 1.75rem; border-left: 1px solid var(--line); min-height: 0; }
  .lbl { color: var(--label-grey); font-size: var(--fs-s); }
  h2 { margin: 0.375rem 0 0.875rem; color: var(--text-hi); font-weight: 500; font-size: var(--fs-xl); line-height: 1.15; }
  .txt { color: var(--prose); line-height: 1.5; }
  .txt :global(p) { margin: 0 0 var(--sp-3); }
  .txt :global(.k) { color: var(--label-grey); font-size: var(--fs-s); margin: 1.125rem 0 var(--sp-1); }
  /* Action words (Buttons with class "act") stack under the text, lined up with it. */
  .txt :global(.act) { display: flex; width: fit-content; margin: var(--sp-1) 0 0 calc(-1 * var(--sp-2)); }
</style>
