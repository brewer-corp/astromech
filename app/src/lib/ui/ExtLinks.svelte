<script lang="ts">
  // Spansh and Inara buttons for a system, or for a station in it. Opens in the browser through
  // the core; shows its own error so a failed open is not silent.
  import { tip } from "./tip";
  import Button from "./Button.svelte";
  import Glyph from "./Glyph.svelte";
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
  <Button onclick={() => void open(spansh)} {@attach tip(`Open ${what} on Spansh`)}>Spansh <Glyph name="external" /></Button>
  {#if station || !marketId}<Button onclick={() => void open(inara)} {@attach tip(`Open ${what} on Inara`)}>Inara <Glyph name="external" /></Button>{/if}
  {#if err}<span class="err" role="alert"><b>!</b> {err}</span>{/if}
</span>

<style>
  .ext { display: inline-flex; gap: var(--sp-1); align-items: center; }
  .err { color: var(--alarm); font-size: var(--fs-s); }
  b { font-weight: 600; }
</style>
