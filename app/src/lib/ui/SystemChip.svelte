<script lang="ts">
  // A system name with a one-click copy button, for pasting into the galaxy map.
  // Uses the webview clipboard API; the core writes the clipboard only for the route follower.
  let { name }: { name: string } = $props();
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

<span class="chip">
  <span class="name">{name}</span>
  <button type="button" class:done={copied} aria-label={`Copy ${name}`} title="Copy system name" onclick={() => void copy()}>
    {copied ? "Copied" : "Copy"}
  </button>
</span>

<style>
  .chip { display: inline-flex; align-items: stretch; vertical-align: baseline; border: 1px solid var(--accent-dim); color: var(--text); text-transform: none; letter-spacing: normal; font-weight: 400; }
  .name { padding: 0 6px; }
  button { all: unset; cursor: pointer; box-sizing: border-box; padding: 0 6px; background: var(--cell); color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  button:hover, button:focus-visible { color: var(--text-hi); outline: 1px solid var(--accent); outline-offset: -1px; }
  .done { color: var(--accent); }
</style>
