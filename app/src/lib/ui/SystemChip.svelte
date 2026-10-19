<script lang="ts">
  // A system name with a one-click copy mark, for pasting into the galaxy map.
  // Uses the webview clipboard API; the core writes the clipboard only for the route follower.
  import Glyph from "./Glyph.svelte";

  let { name, icon = false }: { name: string; /** Copy mark only; the name is already shown beside it. */ icon?: boolean } = $props();
  let copied = $state(false);
  let timer: ReturnType<typeof setTimeout> | undefined;

  async function copy() {
    try {
      await navigator.clipboard.writeText(name);
    } catch {
      return;
    }
    copied = true;
    clearTimeout(timer);
    timer = setTimeout(() => (copied = false), 1200);
  }
</script>

<span class="chip" class:icon>
  {#if !icon}<span class="name">{name}</span>{/if}
  <button type="button" class:done={copied} aria-label={copied ? `Copied ${name}` : `Copy ${name}`} title={copied ? "Copied" : "Copy system name"} onclick={() => void copy()}>
    <Glyph name={copied ? "copied" : "copy"} />
  </button>
</span>

<style>
  .chip { display: inline-flex; align-items: stretch; vertical-align: baseline; max-width: 100%; min-width: 0; }
  .name { padding: 0 2px 0 0; color: var(--text); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  button { all: unset; cursor: pointer; box-sizing: border-box; display: inline-flex; align-items: center; justify-content: center; width: 28px; height: 28px; color: var(--text-lo); flex: none; }
  button:hover, button:focus-visible { color: var(--accent); }
  .done { color: var(--accent); }
</style>
