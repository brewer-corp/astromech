<script lang="ts">
  // Spansh and Inara buttons for a system, or for a station in it. Opens in the browser through
  // the core; shows its own error so a failed open is not silent.
  import { openUrl } from "../core";
  import { inaraStation, inaraSystem, spanshStation, spanshSystem } from "../links";

  let { system, station = null, marketId = null }: { system: string; station?: string | null; marketId?: number | null } = $props();
  let err = $state<string | null>(null);

  // A station link needs a name (Inara searches by it) or a market id (Spansh opens it directly).
  const spansh = $derived(station || marketId != null ? spanshStation({ marketId, name: station ?? "" }) : spanshSystem({ name: system }));
  const inara = $derived(station ? inaraStation(station) : inaraSystem(system));
  const what = $derived(station ?? (marketId != null ? `the station in ${system}` : system));

  async function open(url: string) {
    err = null;
    try {
      await openUrl(url);
    } catch (e) {
      err = `Couldn't open the link: ${String(e)}`;
    }
  }
</script>

<span class="ext">
  <button type="button" title="Open {what} on Spansh" onclick={() => void open(spansh)}>Spansh</button>
  {#if station || !marketId}<button type="button" title="Open {what} on Inara" onclick={() => void open(inara)}>Inara</button>{/if}
  {#if err}<span class="err">{err}</span>{/if}
</span>

<style>
  .ext { display: inline-flex; gap: 4px; align-items: baseline; }
  button { all: unset; cursor: pointer; padding: 0 6px; border: 1px solid var(--accent-dim); background: var(--cell); color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  button:hover, button:focus-visible { color: var(--text-hi); outline: 1px solid var(--accent); outline-offset: -1px; }
  .err { color: var(--warn, var(--text-lo)); font-size: var(--fs-s); }
</style>
