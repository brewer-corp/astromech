<script lang="ts">
  // Superpower mark plus its name. Unknown names use the Independent mark.
  import Glyph, { type GlyphName } from "./Glyph.svelte";

  let { name }: { name: string } = $props();

  function mark(a: string): GlyphName {
    const s = a.toLowerCase();
    if (s.startsWith("fed")) return "federation";
    if (s.startsWith("emp")) return "empire";
    if (s.startsWith("alli")) return "alliance";
    return "independent";
  }
</script>

<span class="al" title={name}>
  <Glyph name={mark(name)} label={name} />
  <span>{name}</span>
</span>

<style>
  .al { display: inline-flex; align-items: center; gap: 6px; min-width: 0; }
  .al span { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
