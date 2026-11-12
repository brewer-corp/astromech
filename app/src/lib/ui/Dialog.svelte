<script lang="ts">
  // A dialog over the page, as the calibration dialog draws it: the window's Void between two
  // orange rules, a white title, then the content. It is a native modal <dialog>, so the
  // keyboard stays inside it and Escape closes it (`onclose`); the page behind is shaded.
  import type { Snippet } from "svelte";

  let { title, onclose, children }: { title: string; onclose: () => void; children: Snippet } = $props();

  let dlg = $state<HTMLDialogElement>();

  $effect(() => {
    const d = dlg;
    if (d && !d.open) d.showModal();
    return () => d?.close();
  });
</script>

<dialog
  bind:this={dlg}
  aria-modal="true"
  aria-label={title}
  oncancel={(e) => {
    e.preventDefault();
    onclose();
  }}
>
  <h2 class="type-title">{title}</h2>
  {@render children()}
</dialog>

<style>
  dialog { box-sizing: border-box; width: min(640px, calc(var(--win-w) * 100 - 32px)); margin: auto; padding: var(--sp-6) 28px; border: 0; border-top: 2px solid var(--accent); border-bottom: 2px solid var(--accent); background: var(--bg); color: var(--accent); }
  dialog::backdrop { background: rgb(0 0 0 / 0.7); }
  h2 { margin: 0 0 var(--sp-4); }
</style>
