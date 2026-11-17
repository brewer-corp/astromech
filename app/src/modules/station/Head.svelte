<script lang="ts">
  // The station docked at, as a page's title (DESIGN.md Icons, The Lifted Name Rule): its type
  // icon at 24 px, its name in Title White, and under it the system, stepped down to Label with
  // its copy icon, and the distance from the star. Shared by Now and the Station page.
  import StationKind from "../../lib/ui/StationKind.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import type { DockedHere } from "../../lib/station";

  let { here }: { here: DockedHere } = $props();
</script>

<div class="head">
  <h2 class="type-title title"><StationKind type={here.station_type} size={24} /><span>{here.station}</span></h2>
  {#if here.system}
    <p class="type-label where">
      <span>{here.system}</span><SystemChip name={here.system} icon />
      {#if here.dist_from_star_ls != null}<span>{Math.round(here.dist_from_star_ls).toLocaleString("en")} ls from the star</span>{/if}
    </p>
  {/if}
</div>

<style>
  .head { display: flex; flex-direction: column; gap: var(--sp-1); }
  .title { display: flex; align-items: center; gap: var(--sp-2); margin: 0; }
  .title :global(.kind) { flex: none; }
  /* Indented to the name, past the 24 px icon and its gap. */
  .where { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-2); margin: 0; padding-left: var(--sp-8); }
  .where :global(.chip) { margin-left: calc(-1 * var(--sp-2)); }
</style>
