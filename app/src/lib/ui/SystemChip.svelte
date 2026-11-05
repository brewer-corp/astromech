<script lang="ts">
  // A system name with a one-click copy mark, for pasting into the galaxy map.
  // Uses the webview clipboard API; the core writes the clipboard only for the route follower.
  import IconButton from "./IconButton.svelte";

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
  <IconButton icon={copied ? "copied" : "copy"} label={copied ? `Copied ${name}` : `Copy ${name}`} tip={copied ? "Copied" : "Copy system name"} onclick={() => void copy()} />
</span>

<style>
  .chip { display: inline-flex; align-items: stretch; vertical-align: baseline; max-width: 100%; min-width: 0; }
  .name { padding: 0 2px 0 0; color: var(--text); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
