<script lang="ts" module>
  import type { GlyphName } from "./Glyph.svelte";

  /**
   * The station type icon (DESIGN.md Icons, the station set) for a type as Spansh names it
   * ("Orbis Starport", "Drake-Class Carrier") or as the journal does ("Orbis", "Bernal",
   * "CraterOutpost", "FleetCarrier"). Order matters: a planetary outpost is a planetary port.
   */
  export function stationGlyph(t: string): GlyphName {
    const s = t.toLowerCase();
    if (s.includes("settlement")) return "settlement";
    if (s.includes("carrier")) return "carrier";
    if (s.includes("mega")) return "megaship";
    if (s.includes("planet") || s.includes("surface") || s.includes("crater")) return "planetary";
    if (s.includes("outpost")) return "outpost";
    if (s.includes("coriolis")) return "coriolis";
    if (s.includes("orbis")) return "orbis";
    if (s.includes("ocellus") || s.includes("bernal")) return "ocellus";
    if (s.includes("asteroid")) return "asteroid-base";
    if (s.includes("starport")) return "starport";
    return "station";
  }
</script>

<script lang="ts">
  // Station type as a HUD mark: a fact, not a control, in Label Grey before the name it
  // describes, 16 px in a row and 24 beside a title. Its accessible name is the type: Spansh's
  // full name, or for the journal's one-word type the set's name ("Coriolis starport").
  import { tip } from "./tip";
  import Glyph, { typeNames } from "./Glyph.svelte";

  let { type, size = 16 }: { type: string | null | undefined; size?: number } = $props();

  const glyph = $derived(type ? stationGlyph(type) : "station");
  const named = (g: GlyphName): string | undefined => (typeNames as Partial<Record<GlyphName, string>>)[g];
  const label = $derived(type && !/\s/.test(type) ? (named(glyph) ?? type) : (type ?? ""));
</script>

{#if type}
  <span class="kind" {@attach tip({ title: label, stop: false })}>
    <Glyph name={glyph} {label} {size} />
  </span>
{/if}

<style>
  .kind { display: inline-flex; color: var(--label-grey); }
</style>
