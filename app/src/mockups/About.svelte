<script lang="ts">
  import type { Snippet } from "svelte";
  // "About this setting" panel, as in the game: small orange label, white title, plain text.
  let { label = "About this setting", title, children }: { label?: string; title: string; children: Snippet } = $props();
</script>

<aside>
  <div class="lbl">{label}</div>
  <h2>{title}</h2>
  <div class="txt">{@render children()}</div>
</aside>

<style>
  aside { padding: 20px 0 20px 28px; border-left: 2px solid var(--accent); min-height: 0; }
  .lbl { color: var(--accent); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  h2 { margin: 6px 0 14px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-xl); text-transform: uppercase; letter-spacing: 0.03em; line-height: 1.15; }
  .txt { color: var(--text); line-height: 1.5; }
  .txt :global(p) { margin: 0 0 12px; }
  .txt :global(.k) { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); margin: 18px 0 4px; }
  .txt :global(.act) { color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); display: block; margin: 6px 0; }
</style>
