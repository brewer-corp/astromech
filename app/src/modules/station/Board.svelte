<script lang="ts">
  // The services board (DESIGN.md Board): one Seam-outlined tile per service in the game's
  // station menu, in its order. A service the station has is lit (Name, Options Grey); one it
  // lacks is shown, not left out, in Grey Dim. The tiles are facts, not controls. A service the
  // station reports that the menu doesn't list (dock, contacts, shop, carrier services) isn't drawn.
  import Tiles from "../../lib/ui/layout/Tiles.svelte";
  import { MENU_SERVICES, serviceName } from "../../lib/station";

  let { services }: { services: string[] } = $props();
  const has = $derived(new Set(services));
</script>

<section class="board" aria-labelledby="services-head">
  <h3 class="section" id="services-head">Services</h3>
  <Tiles label="Services" list>
    {#each MENU_SERVICES as s (s)}
      {@const lit = has.has(s)}
      <div class="tile type-name" class:lit role="listitem" aria-label={lit ? serviceName(s) : `${serviceName(s)}: not here`}>{serviceName(s)}</div>
    {/each}
  </Tiles>
</section>

<style>
  .board { display: flex; flex-direction: column; gap: var(--sp-3); }
  .tile { display: flex; align-items: center; min-height: var(--row-result); padding: 0 var(--sp-4); outline: 1px solid var(--line); outline-offset: -1px; color: var(--grey-dim); font-weight: 400; }
  .lit { color: var(--options-grey); font-weight: 500; }
</style>
