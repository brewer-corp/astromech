<script lang="ts">
  // DESIGN.md mockup: Materials, raw grid. Rows are categories, columns are grades (the trader's
  // own layout). One value per tile (the count), state by orange brightness, the selected tile
  // filled. Its category head turns white, as the trader's current section does.
  // Same sample counts as the current page (docs/design/design-system-materials-before.png).
  import Chrome from "./Chrome.svelte";

  const caps = [300, 250, 200, 150];
  const grid: [string, number][][] = [
    [["Carbon", 89], ["Vanadium", 9], ["Niobium", 0], ["Yttrium", 0]],
    [["Phosphorus", 4], ["Chromium", 0], ["Molybdenum", 0], ["Technetium", 0]],
    [["Sulphur", 0], ["Manganese", 4], ["Cadmium", 0], ["Ruthenium", 0]],
    [["Iron", 22], ["Zinc", 0], ["Tin", 0], ["Selenium", 0]],
    [["Nickel", 0], ["Germanium", 4], ["Tungsten", 0], ["Tellurium", 0]],
    [["Rhenium", 0], ["Arsenic", 0], ["Mercury", 0], ["Polonium", 0]],
    [["Lead", 0], ["Zirconium", 0], ["Boron", 0], ["Antimony", 0]],
  ];
  const need: Record<string, number> = { Vanadium: 12, Carbon: 20 };
  const selected = "Iron";
</script>

<Chrome top="Engineering" tabs={["Ships", "Stored modules", "Materials", "On foot", "Engineers"]} tab="Materials"
  strip={{ next: "Wregoe OI-C b54-6", of: "3 of 41", flags: ["Scoopable"], dest: "Colonia", left: "38 jumps left" }}>
  <div class="page">
    <section class="main">
      <div class="bar">
        <div class="choices">
          <button class="choice on">Raw</button><button class="choice">Manufactured</button><button class="choice">Encoded</button>
        </div>
        <span class="label"><span class="num">132</span> of <span class="num">6,300</span> held · sample journal</span>
      </div>

      <div class="grid">
        <span></span>
        {#each caps as cap, g (g)}<span class="label colhead">Grade {g + 1} · max <span class="num">{cap}</span></span>{/each}
        {#each grid as row, c (c)}
          <span class="label rowhead" class:current={c === 3}>Category {c + 1}</span>
          {#each row as [name, n], g (name)}
            {@const cap = caps[g] ?? 1}
            {@const want = need[name]}
            <div class="tile" class:none={n === 0} class:sel={name === selected}>
              <span class="count num">{n}</span>
              <span class="name">{name}</span>
              {#if want}<span class="need num">{#if n < want}<span class="bang">!</span>{/if}need {want}</span>{/if}
              {#if n > 0 && name !== selected}<span class="track"><span class="fill" class:full={n >= cap} style="width: {(100 * n) / cap}%"></span></span>{/if}
            </div>
          {/each}
        {/each}
      </div>
    </section>

    <aside>
      <h2 class="title">Iron</h2>
      <p class="label">Raw · grade 1 · category 4</p>
      <p class="held"><span class="white big">22</span><span class="unit">of 300</span></p>
      <dl>
        <dt class="label">Trade up</dt><dd><span class="num">6</span> Iron → <span class="num">1</span> Zinc</dd>
        <dt class="label">Trade down</dt><dd>—</dd>
        <dt class="label">Wanted by</dt><dd class="mid">No plan</dd>
      </dl>
      <h3 class="section">Where to find</h3>
      <p class="body prose">Surface prospecting on most rocky and metal-rich bodies, and mining in metallic rings.</p>
      <div class="acts"><button class="act">Find raw sites</button><button class="act">Nearest trader</button></div>
    </aside>
  </div>
</Chrome>

<style>
  .page { display: grid; grid-template-columns: auto var(--panel); justify-content: start; gap: var(--sp-12); height: 100%; }
  .bar { display: flex; align-items: center; gap: var(--sp-6); margin-bottom: var(--sp-6); }
  .grid { display: grid; grid-template-columns: 112px repeat(4, 220px); gap: var(--sp-2); align-items: center; }
  .colhead { padding: 0 var(--sp-3); }
  .rowhead { align-self: center; }
  .rowhead.current { color: var(--text-hi); }
  .tile { position: relative; height: var(--tile); padding: 10px var(--sp-3); background: var(--surface); display: grid; grid-template-rows: auto 1fr; align-content: start; }
  .count { font-size: var(--fs-xxl); font-weight: 500; line-height: 1; color: var(--orange-lit); }
  .name { align-self: end; font-size: var(--fs-s); color: var(--accent); }
  .need { position: absolute; top: 10px; right: var(--sp-3); font-size: var(--fs-xs); color: var(--accent); }
  .track { position: absolute; left: 0; right: 0; bottom: 0; height: 2px; background: var(--line); }
  .fill { display: block; height: 100%; background: var(--orange-mid); }
  .fill.full { background: var(--orange-lit); }
  .tile.none { background: var(--bg); outline: 1px solid var(--line); outline-offset: -1px; }
  .tile.none .count, .tile.none .name { color: var(--orange-dim); }
  .tile.sel { background: var(--accent); }
  .tile.sel .count, .tile.sel .name { color: var(--on-accent); }

  aside { border-left: 1px solid var(--accent); padding: 0 0 0 var(--sp-6); }
  aside .label { margin: var(--sp-1) 0 0; }
  .held { margin: var(--sp-4) 0; }
  .big { font-size: var(--fs-xxl); font-weight: 500; }
  dl { display: grid; grid-template-columns: 96px 1fr; gap: var(--sp-2) var(--sp-3); margin: 0 0 var(--sp-8); }
  dt, dd { margin: 0; }
  dt { align-self: center; }
  aside .section { margin-bottom: var(--sp-3); }
  .body { margin: 0 0 var(--sp-3); max-width: var(--measure); }
  .acts { display: flex; margin-left: calc(-1 * var(--sp-2)); }
</style>
