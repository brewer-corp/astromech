<script lang="ts">
  // Import a route, from the Route page's ⋯ menu: a pasted Spansh CSV export (neutron, road to
  // riches, carrier…) or one system name per line, parsed by the core, and an optional name.
  // Follow this route follows it and closes the dialog; a failure stays in the dialog as
  // DESIGN.md's hard error, with the text kept.
  import { importRoute, type RouteView } from "../../lib/route";
  import Button from "../../lib/ui/Button.svelte";
  import Dialog from "../../lib/ui/Dialog.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import Input from "../../lib/ui/Input.svelte";

  let { onclose, onview }: { onclose: () => void; onview: (v: RouteView) => void } = $props();

  let name = $state("");
  let text = $state("");
  let busy = $state(false);
  let err = $state<string | null>(null);

  async function follow() {
    busy = true;
    err = null;
    try {
      onview(await importRoute(name.trim() || "Imported route", text));
    } catch (e) {
      err = String(e instanceof Error ? e.message : e);
    } finally {
      busy = false;
    }
  }
</script>

<Dialog title="Import a route" {onclose}>
  <div class="body">
    <Input aria-label="Route name" placeholder="Name (optional)" bind:value={name} />
    <Input rows={8} aria-label="Route" placeholder="Paste a Spansh CSV (neutron, road to riches, carrier…) or one system name per line" bind:value={text} />
    {#if err}<Failure text={err} onaction={() => void follow()} />{/if}
    <div class="acts">
      <Button onclick={onclose}>Cancel</Button>
      <Button kind="primary" onclick={() => void follow()} disabled={!text.trim() || busy}>Follow this route</Button>
    </div>
  </div>
</Dialog>

<style>
  .body { display: flex; flex-direction: column; gap: var(--sp-2); }
  .acts { display: flex; justify-content: flex-end; gap: var(--sp-1); margin-top: var(--sp-2); }
</style>
