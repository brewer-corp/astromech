<script lang="ts">
  // The ship and suit trees side by side while the width the layout gives them holds both, else
  // the suit tree below the ship tree. Decided from ListDetail's measured width rather than by
  // wrapping, so the list is only as wide as what it shows and the panel sits right after it.
  import type { Snippet } from "svelte";
  import { useDetail } from "../../lib/ui/layout/listDetail.svelte";

  let { children, both }: { children: Snippet; /** px at 100 percent the two trees need side by side. */ both: number } = $props();
  const detail = useDetail();
  const stacked = $derived((detail?.room ?? 0) > 0 && detail!.room < both);
  // A selection that opens the panel can stack the trees, which moves the node just clicked:
  // bring it back into view.
  let el = $state<HTMLElement>();
  let was = false;
  $effect(() => {
    if (stacked && !was) el?.querySelector<HTMLElement>(".node[aria-pressed=true]")?.scrollIntoView({ block: "nearest" });
    was = stacked;
  });
</script>

<div class="trees" class:stacked bind:this={el}>{@render children()}</div>

<style>
  .trees { display: flex; align-items: start; gap: var(--sp-8) var(--sp-12); }
  .trees.stacked { flex-direction: column; }
</style>
