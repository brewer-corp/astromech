<script lang="ts">
  // Station type as a HUD mark. The full Spansh name is the accessible name.
  import Glyph, { type GlyphName } from "./Glyph.svelte";

  let { type }: { type: string | null | undefined } = $props();

  function kind(t: string): GlyphName {
    const s = t.toLowerCase();
    if (s.includes("settlement")) return "settlement";
    if (s.includes("carrier")) return "carrier";
    if (s.includes("mega")) return "megaship";
    if (s.includes("planet") || s.includes("surface")) return "planetary";
    if (s.includes("outpost")) return "outpost";
    if (s.includes("coriolis") || s.includes("orbis") || s.includes("ocellus") || s.includes("starport") || s.includes("asteroid")) return "starport";
    return "station";
  }
</script>

{#if type}
  <span class="kind" title={type}>
    <Glyph name={kind(type)} label={type} />
  </span>
{/if}

<style>
  .kind { display: inline-flex; color: var(--text-lo); }
</style>
