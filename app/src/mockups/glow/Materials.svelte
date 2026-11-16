<script lang="ts">
  // Glow-up mockup: Engineering > Materials, the grid DESIGN.md specifies with the type scale made
  // real (List with a detail panel, the list a tile grid). Rows are categories, columns grades.
  // The count is the tile's one value in Value XL; "need 12" corner marks come from the build
  // plans. One tile selected, explained by the panel; one corner mark's Tip shown open.
  import Frame from "./Frame.svelte";
  import Tip from "./Tip.svelte";

  const caps = [300, 250, 200, 150];
  const grid: [string, number][][] = [
    [["Carbon", 89], ["Vanadium", 9], ["Niobium", 0], ["Yttrium", 2]],
    [["Phosphorus", 141], ["Chromium", 37], ["Molybdenum", 12], ["Technetium", 0]],
    [["Sulphur", 300], ["Manganese", 34], ["Cadmium", 6], ["Ruthenium", 3]],
    [["Iron", 212], ["Zinc", 48], ["Tin", 0], ["Selenium", 9]],
    [["Nickel", 166], ["Germanium", 4], ["Tungsten", 21], ["Tellurium", 0]],
    [["Rhenium", 58], ["Arsenic", 0], ["Mercury", 7], ["Polonium", 1]],
    [["Lead", 120], ["Zirconium", 19], ["Boron", 0], ["Antimony", 5]],
  ];
  const need: Record<string, number> = { Vanadium: 12, Carbon: 20, Germanium: 10, Manganese: 6, Arsenic: 3, Tungsten: 4 };
  // The summary line is worked out from the grid, so sample edits can't leave it wrong.
  const all = grid.flat();
  const held = all.reduce((sum, [, n]) => sum + n, 0);
  const wanted = all.filter(([name]) => need[name]).length;
  const short = all.filter(([name, n]) => n < (need[name] ?? 0)).length;
  const selected = "Vanadium";
  const tipOn = "Germanium";
</script>

<Frame top="Engineering" tabs={["Ships", "Stored modules", "Materials", "On foot", "Engineers"]} tab="Materials"
  strip={{ next: "Wregoe OI-C b54-6", of: "4 of 10", flags: ["Scoopable"], dest: "Synuefe GW-Z a54-3", left: "6 jumps left" }}>
  <div class="page">
    <section class="main">
      <div class="bar">
        <span class="choices"><button class="choice on">Raw</button><button class="choice">Manufactured</button><button class="choice">Encoded</button></span>
        <span class="label">{held.toLocaleString("en-GB")} held · {wanted} wanted by your plans, {short} short</span>
      </div>

      <div class="grid">
        <span></span>
        {#each caps as cap, g (g)}<span class="label colhead">Grade {g + 1} · max {cap}</span>{/each}
        {#each grid as row, c (c)}
          <span class="label rowhead" class:current={c === 0}>Category {c + 1}</span>
          {#each row as [name, n], g (name)}
            {@const cap = caps[g] ?? 1}
            {@const want = need[name]}
            <div class="tile" class:none={n === 0} class:sel={name === selected}>
              <span class="count">{n}</span>
              <span class="name">{name}</span>
              {#if want}
                <span class="need" class:short={n < want}>
                  {#if n < want}<span class="bang">!</span>{/if}need {want}
                  {#if name === tipOn}
                    <Tip side="below" end width={340}>
                      <b>Germanium: 4 of 10.</b> Long Way Round's wide angle sensors want 6 and Heavy Lifter's
                      thermal resistant shields want 4. Trade 36 Nickel up to make the 6 you lack.
                    </Tip>
                  {/if}
                </span>
              {/if}
              {#if n > 0 && name !== selected}<span class="track"><span class="fill" class:full={n >= cap} style="width: {(100 * n) / cap}%"></span></span>{/if}
            </div>
          {/each}
        {/each}
      </div>
    </section>

    <aside>
      <h2 class="title">Vanadium</h2>
      <p class="label">Raw · grade 2 · category 1</p>
      <p class="key"><span class="vxl white">9</span><span class="unit">of 250 held</span></p>
      <p class="short-line"><span class="bang">!</span>3 short of what your plans need.</p>

      <h3 class="section">Wanted by</h3>
      <div class="wants">
        <span>Long Way Round<span class="label sub">Thrusters, reinforced G3</span></span><span class="r">4</span>
        <span>Heavy Lifter<span class="label sub">Armour, heavy duty G4</span></span><span class="r">8</span>
      </div>

      <h3 class="section">Trade for it</h3>
      <div class="wants">
        <span>6 Carbon <span class="mid">→ 1</span></span><span class="r"><span class="lit">14</span><span class="unit">from your 89</span></span>
        <span>1 Niobium <span class="mid">→ 3</span></span><span class="r mid">none held</span>
      </div>

      <h3 class="section">Where to get it</h3>
      <ul class="get">
        <li>Outcrops on rocky and metal-rich bodies</li>
        <li>Mining in metallic rings</li>
        <li>A raw material trader</li>
      </ul>
      <div class="acts"><button class="primary">Nearest raw trader</button><button class="act">Find sites</button></div>
    </aside>
  </div>
</Frame>

<style>
  .page { display: grid; grid-template-columns: minmax(0, 1fr) var(--panel); gap: var(--sp-12); max-width: 1760px; }
  .bar { display: flex; align-items: center; gap: var(--sp-6); margin-bottom: var(--sp-6); }
  .grid { display: grid; grid-template-columns: 104px repeat(4, minmax(180px, 290px)); gap: var(--sp-2); align-items: center; }
  .colhead { padding: 0 var(--sp-3); }
  .rowhead.current { color: var(--text-hi); }
  .tile { position: relative; height: var(--tile); padding: var(--sp-2) var(--sp-3); background: var(--surface); display: grid; grid-template-rows: auto 1fr; }
  .count { font-size: var(--fs-xxl); font-weight: 500; line-height: 1; color: var(--orange-lit); }
  .name { align-self: end; font-size: var(--fs-s); color: var(--accent); }
  .need { position: absolute; top: var(--sp-2); right: var(--sp-3); font-size: var(--fs-xs); color: var(--accent); }
  .need .bang { margin-right: 3px; }
  .track { position: absolute; left: 0; right: 0; bottom: 0; height: 2px; background: var(--line); }
  .fill { display: block; height: 100%; background: var(--orange-mid); }
  .fill.full { background: var(--orange-lit); }
  .tile.none { background: var(--bg); outline: 1px solid var(--line); outline-offset: -1px; }
  .tile.none .count, .tile.none .name { color: var(--orange-dim); }
  .tile.sel { background: var(--accent); }
  .tile.sel .count, .tile.sel .name, .tile.sel .need, .tile.sel .bang { color: var(--on-accent); }

  aside { border-left: 1px solid var(--accent); padding-left: var(--sp-6); }
  aside > .label { margin-top: var(--sp-1); }
  .key { display: flex; align-items: baseline; margin: var(--sp-6) 0 var(--sp-2); }
  .short-line { color: var(--accent); margin-bottom: var(--sp-8); }
  .section { margin-bottom: var(--sp-3); }
  .wants { display: grid; grid-template-columns: 1fr auto; align-items: start; gap: var(--sp-3); color: var(--accent); margin-bottom: var(--sp-8); }
  .wants .sub { display: block; }
  .get { display: grid; gap: var(--sp-2); color: var(--accent); margin-bottom: var(--sp-6); }
  .acts { display: flex; gap: var(--sp-4); }
</style>
