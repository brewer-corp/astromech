<script lang="ts">
  // DESIGN.md's no-route rail: with no route, at or above the dock width, a 48 px column of Void
  // with a Seam rule on its right and two icon buttons, one that takes focus to the header's Find
  // or plot and one that opens the Import a route dialog. It reserves no more width.
  import IconButton from "../../lib/ui/IconButton.svelte";
  import ImportDialog from "./ImportDialog.svelte";
  import { focusFindOrPlot } from "./PlotBar.svelte";
  import { useFollow } from "./follow.svelte";

  const { follow } = useFollow();
  let importing = $state(false);
</script>

<nav class="rail" aria-label="No route">
  <IconButton icon="add-stop" label="Find or plot" tip="No route. Plot one in Find or plot (Ctrl+K)" onclick={focusFindOrPlot} />
  <IconButton icon="import" label="Import a route" tip="Import a route: a Spansh CSV or a list of systems" onclick={() => (importing = true)} />
</nav>

{#if importing}
  <ImportDialog onclose={() => (importing = false)} onview={(v) => { importing = false; follow.view = v; }} />
{/if}

<style>
  .rail { box-sizing: border-box; display: flex; flex-direction: column; align-items: center; gap: var(--sp-2); width: var(--rail); height: 100%; padding-top: var(--sp-4); border-right: 1px solid var(--line); }
</style>
