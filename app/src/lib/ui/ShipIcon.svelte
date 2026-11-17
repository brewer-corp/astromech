<script lang="ts">
  import { shipIcon } from "../shipIcon";
  import Glyph from "./Glyph.svelte";

  // size: px at 100 percent Text size, drawn in rem.
  let { type, size = 48 }: { type: string; size?: number } = $props();
  const src = $derived(shipIcon(type));
</script>

{#if src}
  <img class="art" src={src} alt="" style:width="{size / 16}rem" style:height="{size / 16}rem" />
{:else}
  <span class="fallback" style:width="{size / 16}rem" style:height="{size / 16}rem">
    <Glyph name="ship" />
  </span>
{/if}

<style>
  .art { display: block; object-fit: contain; flex: none; }
  .fallback { display: grid; place-items: center; color: var(--label-grey); flex: none; }
</style>
