<script lang="ts">
  import { shipIcon } from "../shipIcon";
  import Glyph from "./Glyph.svelte";

  let { type, name, size = 48 }: { type: string; name: string; size?: number } = $props();
  const src = $derived(shipIcon(type));
</script>

{#if src}
  <img class="art" src={src} alt="" width={size} height={size} />
{:else}
  <span class="fallback" style:width="{size}px" style:height="{size}px" title={name}>
    <Glyph name="ship" />
  </span>
{/if}

<style>
  .art { display: block; object-fit: contain; flex: none; }
  .fallback { display: grid; place-items: center; color: var(--text-lo); flex: none; }
</style>
