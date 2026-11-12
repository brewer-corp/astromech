<script lang="ts">
  // Glow-up icon sheet (astromech-dsyu): every Glyph with its name at 16 px and at 1.4x, the top
  // of the window scale. Rendered to docs/design/glow-up-icons.png by scripts/mockup-shots.sh.
  import "./ds/ds.css";
  import Glyph, { typeNames, type GlyphName } from "../lib/ui/Glyph.svelte";

  const named = typeNames as Partial<Record<GlyphName, string>>;
  const groups: [string, [GlyphName, string?][]][] = [
    ["Stars", [["star-scoopable"], ["star-neutron"], ["star-white-dwarf"], ["star-brown-dwarf"], ["star-black-hole"], ["star-other"], ["star-unknown"]]],
    ["Bodies", [["body-rocky"], ["body-high-metal"], ["body-metal-rich"], ["body-icy"], ["body-rocky-ice"], ["body-water-world"], ["body-earth-like"], ["body-ammonia"], ["body-gas-giant"], ["body-water-giant"], ["body-ringed"]]],
    ["Stations", [["coriolis"], ["orbis"], ["ocellus"], ["outpost"], ["planetary"], ["settlement"], ["asteroid-base"], ["carrier"], ["megaship"], ["starport", "Starport (any)"], ["station", "Station (other)"]]],
    ["Actions and others", [["copy", "Copy"], ["copied", "Copied"], ["add-stop", "Add stop"], ["destination", "Set destination"], ["external", "Open external"], ["chevron", "Chevron"], ["clear", "Clear"], ["ship", "Ship"], ["federation", "Federation"], ["empire", "Empire"], ["alliance", "Alliance"], ["independent", "Independent"]]],
  ];
</script>

<main class="ds sheet">
  {#each groups as [title, icons] (title)}
    <section>
      <h2 class="section">{title}</h2>
      <div class="head label"><span class="nowrap">16 px</span><span>1.4x</span><span></span></div>
      {#each icons as [name, own] (name)}
        {@const label = own ?? named[name] ?? name}
        <div class="row">
          <Glyph {name} {label} />
          <Glyph {name} {label} size={22.4} />
          <span class="mid">{label}</span>
        </div>
      {/each}
    </section>
  {/each}
</main>

<style>
  .sheet { min-height: 100vh; box-sizing: border-box; background: var(--bg); padding: var(--sp-8); display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--sp-12); align-items: start; }
  section { display: grid; gap: var(--sp-1); }
  .head, .row { display: grid; grid-template-columns: 40px 32px 1fr; align-items: center; gap: var(--sp-3); }
  .row { min-height: 30px; color: var(--accent); }
  .head { margin-top: var(--sp-2); }
  .nowrap { white-space: nowrap; }
</style>
