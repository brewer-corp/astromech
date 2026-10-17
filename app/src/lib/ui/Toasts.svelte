<script lang="ts">
  // Callout toasts from the core (V1-DESIGN.md: callouts speak, toast or notify). Each new one
  // shows for a few seconds in the corner.
  import { untrack } from "svelte";
  import { freshToasts, type CoreStatus, type Toast } from "../core";

  let { status }: { status: CoreStatus | null } = $props();
  const SHOW_MS = 8000;
  let lastSeen: number | null = null;
  let shown = $state<Toast[]>([]);

  $effect(() => {
    if (!status) return;
    const { show, last } = freshToasts(status.toasts ?? [], lastSeen);
    lastSeen = last;
    const key = (t: Toast) => `${t.time}-${t.id}`;
    untrack(() => {
      for (const t of show) {
        shown = [...shown, t];
        setTimeout(() => (shown = shown.filter((s) => key(s) !== key(t))), SHOW_MS);
      }
    });
  });
</script>

{#if shown.length}
  <div class="toasts" role="status" aria-live="polite">
    {#each shown as t (`${t.time}-${t.id}`)}
      <p>{t.text}</p>
    {/each}
  </div>
{/if}

<style>
  .toasts { position: fixed; right: 40px; bottom: 32px; display: grid; gap: 8px; max-width: 360px; pointer-events: none; }
  p { margin: 0; padding: 10px 14px; background: var(--row-hover); color: var(--text-hi); border-left: 2px solid var(--accent); font-size: var(--fs-s); }
</style>
