<script lang="ts">
  // Glow-up mockup: Engineering > Ships as a loadout (List with a detail panel). The ship and four
  // hero stats, then the slots grouped as the game's outfitting groups them. Each slot is a hull
  // row: class and rating, module, its engineering as grade pips and the blueprint. Plain slots
  // carry no pips or blueprint, so engineering stands out. The fleet is a strip at the foot.
  import Frame from "./Frame.svelte";
  import Copy from "./Copy.svelte";

  type Slot = { size: string; module: string; grade?: number; blueprint?: string; special?: string };
  const groups: [string, Slot[]][] = [
    ["Hardpoints", [
      { size: "3C", module: "Multi-cannon, gimballed", grade: 5, blueprint: "Overcharged", special: "Corrosive shell" },
      { size: "3C", module: "Multi-cannon, gimballed", grade: 5, blueprint: "Overcharged", special: "Auto loader" },
      { size: "3D", module: "Beam laser, gimballed", grade: 3, blueprint: "Efficient" },
      { size: "2E", module: "Pulse laser, gimballed" },
      { size: "2E", module: "Pulse laser, gimballed" },
    ]],
    ["Utility", [
      { size: "0A", module: "Shield booster", grade: 1, blueprint: "Heavy duty" },
      { size: "0A", module: "Shield booster" },
      { size: "0I", module: "Chaff launcher" },
      { size: "0I", module: "Heat sink launcher" },
    ]],
    ["Core", [
      { size: "", module: "Military grade composite", grade: 5, blueprint: "Heavy duty" },
      { size: "7A", module: "Power plant", grade: 3, blueprint: "Overcharged" },
      { size: "6A", module: "Thrusters", grade: 3, blueprint: "Dirty drive tuning" },
      { size: "5A", module: "Frame shift drive", grade: 5, blueprint: "Increased range", special: "Mass manager" },
      { size: "4D", module: "Life support" },
      { size: "7A", module: "Power distributor", grade: 3, blueprint: "Charge enhanced" },
      { size: "6D", module: "Sensors", grade: 3, blueprint: "Light weight" },
      { size: "5C", module: "Fuel tank" },
    ]],
    ["Optional", [
      { size: "6A", module: "Shield generator", grade: 3, blueprint: "Reinforced" },
      { size: "6E", module: "Cargo rack, 64 t" },
      { size: "5E", module: "Cargo rack, 32 t" },
      { size: "5E", module: "Cargo rack, 32 t" },
      { size: "4E", module: "Cargo rack, 16 t" },
      { size: "3A", module: "Fuel scoop" },
    ]],
  ];
  const left = groups.slice(0, 2);
  const right = groups.slice(2);
  const selected = "Power distributor";

  const fleet = [
    { name: "Long Way Round", type: "Krait Mk II", img: "krait-mk-ii", at: "Here" },
    { name: "Heavy Lifter", type: "Anaconda", img: "anaconda", at: "127 ly" },
    { name: "Mule", type: "Type-9 Heavy", img: "type-9-heavy", at: "Here" },
    { name: "Blink", type: "Imperial Courier", img: "imperial-courier", at: "98 ly" },
    { name: "Synthetic Krait", type: "Krait Phantom", img: "krait-phantom", at: "1,012 ly" },
  ];
</script>

{#snippet slots(gs: [string, Slot[]][])}
  {#each gs as [group, list] (group)}
    <h2 class="section" class:current={list.some((s) => s.module === selected)}>{group}</h2>
    <div class="slots">
      {#each list as s, i (i)}
        <div class="slot" class:sel={s.module === selected} class:plain={!s.grade}>
          <span class="size">{s.size}</span>
          <span class="module">{s.module}</span>
          <span class="pips">{#if s.grade}{#each [1, 2, 3, 4, 5] as g (g)}<span class:on={g <= (s.grade ?? 0)}></span>{/each}{/if}</span>
          <span class="bp">{s.blueprint ?? ""}{#if s.special}<span class="special">· {s.special}</span>{/if}</span>
        </div>
      {/each}
    </div>
  {/each}
{/snippet}

<Frame top="Engineering" tabs={["Ships", "Stored modules", "Materials", "On foot", "Engineers"]} tab="Ships"
  strip={{ next: "Wregoe OI-C b54-6", of: "4 of 10", flags: ["Scoopable"], dest: "Synuefe GW-Z a54-3", left: "6 jumps left" }}>
  <div class="page">
    <section class="main">
      <div class="ship">
        <img src="/ships/krait-mk-ii.png" alt="Krait Mk II" />
        <div class="who">
          <h1 class="title">Long Way Round</h1>
          <p class="label">Krait Mk II · KR-27A · 9 of 23 modules engineered</p>
        </div>
        <dl class="stats">
          <div><dt class="label">Jump range</dt><dd><span class="vxl">31.4</span><span class="unit">ly</span></dd></div>
          <div><dt class="label">Shields</dt><dd><span class="vxl">612</span><span class="unit">MJ</span></dd></div>
          <div><dt class="label">Top speed</dt><dd><span class="vxl">312</span><span class="unit">m/s</span></dd></div>
          <div><dt class="label">Power used</dt><dd><span class="vxl">26.1</span><span class="unit">of 31.2 MW</span></dd></div>
        </dl>
      </div>

      <div class="loadout">
        <div class="col">{@render slots(left)}</div>
        <div class="col">{@render slots(right)}</div>
      </div>
    </section>

    <aside>
      <h2 class="title">Power distributor</h2>
      <p class="label">7A · Charge enhanced, grade 3 of 5</p>
      <p class="key"><span class="vxl white">+31%</span><span class="unit">recharge</span></p>
      <dl class="facts">
        <dt class="label">Weapons</dt><dd>5.6<span class="unit">MW/s</span></dd>
        <dt class="label">Engines</dt><dd>4.0<span class="unit">MW/s</span></dd>
        <dt class="label">Systems</dt><dd>4.0<span class="unit">MW/s</span></dd>
      </dl>
      <h3 class="section">Grade 4 needs</h3>
      <div class="needs">
        <span class="label">Material</span><span class="label r">Need</span><span class="label r">Have</span>
        <span>Grid resistors</span><span class="r">1</span><span class="r">4</span>
        <span>Conductive polymers</span><span class="r">1</span><span class="r">3</span>
        <span>Hybrid capacitors</span><span class="r">1</span><span class="r"><span class="bang">!</span>0</span>
      </div>
      <p class="eng"><span class="label">Engineer</span><span class="ename">The Dweller</span><span class="label">Wyrd</span><Copy /></p>
      <p class="warn"><span class="bang">!</span>Unlock The Dweller first: sell at 5 black markets, 2 of 5.</p>
      <div class="acts"><button class="primary">Add to plan</button><button class="act">Engineers</button></div>
    </aside>

    <section class="fleet">
      <h2 class="section">Fleet</h2>
      <div class="ships">
        {#each fleet as f, i (f.name)}
          <div class="boat" class:shown={i === 0}>
            <img src="/ships/{f.img}.png" alt="" />
            <span class="fname">{f.name}</span>
            <span class="label">{f.type} · {f.at}</span>
          </div>
        {/each}
      </div>
    </section>
  </div>
</Frame>

<style>
  .page { display: grid; grid-template-columns: minmax(0, 1fr) var(--panel); grid-template-rows: 1fr auto; column-gap: var(--sp-12); row-gap: var(--sp-6); max-width: 1760px; min-height: 100%; }
  .ship { display: flex; align-items: center; gap: var(--sp-6); margin-bottom: var(--sp-4); }
  .ship img { width: 96px; height: 96px; margin: -12px -8px -12px -8px; }
  .who { flex: 1; }
  .who .label { margin-top: var(--sp-1); }
  .stats { display: flex; gap: var(--sp-12); }
  .stats dd { display: flex; align-items: baseline; margin-top: var(--sp-2); }

  .loadout { display: grid; grid-template-columns: 1fr 1fr; gap: var(--sp-8); align-items: start; }
  .section { margin: var(--sp-6) 0 var(--sp-3); }
  .col > :global(.section:first-child) { margin-top: 0; }
  .slots { display: grid; gap: var(--gap); }
  .slot { display: grid; grid-template-columns: 32px minmax(0, 1fr) 52px minmax(0, 1fr); align-items: center; column-gap: var(--sp-3); height: 34px; padding: 0 var(--sp-3); background: var(--surface); white-space: nowrap; }
  .size { color: var(--orange-mid); }
  .module { color: var(--accent); overflow: hidden; text-overflow: ellipsis; }
  .slot:not(.plain) .module { color: var(--text-hi); }
  .bp { color: var(--accent); overflow: hidden; text-overflow: ellipsis; }
  .special { color: var(--orange-mid); margin-left: 6px; }
  .slot.plain { background: transparent; outline: 1px solid var(--line); outline-offset: -1px; }
  .slot.sel { background: var(--accent); outline: none; }
  .slot.sel * { color: var(--on-accent) !important; }
  .slot.sel .pips > span { border-color: var(--on-accent); }
  .slot.sel .pips > span.on { background: var(--on-accent); }

  aside { grid-row: 1 / 3; grid-column: 2; border-left: 1px solid var(--accent); padding-left: var(--sp-6); }
  aside > .label { margin-top: var(--sp-1); }
  .key { display: flex; align-items: baseline; margin: var(--sp-6) 0 var(--sp-4); }
  .facts { display: grid; grid-template-columns: 112px 1fr; gap: var(--sp-2) var(--sp-3); margin-bottom: var(--sp-8); }
  dt { align-self: center; }
  aside .section { margin-top: 0; }
  .needs { display: grid; grid-template-columns: 1fr 56px 56px; gap: var(--sp-2) var(--sp-3); color: var(--accent); margin-bottom: var(--sp-6); }
  .eng { display: flex; align-items: baseline; gap: var(--sp-3); }
  .eng :global(.copy) { align-self: center; margin-left: -8px; }
  .ename { color: var(--accent); }
  .warn { margin: var(--sp-2) 0 var(--sp-6); color: var(--accent); }
  .acts { display: flex; gap: var(--sp-4); }

  .fleet { grid-column: 1; align-self: end; }
  .fleet .section { margin-top: 0; }
  .ships { display: grid; grid-template-columns: repeat(5, minmax(0, 1fr)); gap: var(--sp-2); }
  .boat { display: grid; grid-template-columns: 56px 1fr; grid-template-rows: auto auto; column-gap: var(--sp-3); align-items: center; height: 56px; padding: 0 var(--sp-3) 0 var(--sp-2); background: var(--surface); border-bottom: 2px solid transparent; white-space: nowrap; }
  .boat img { grid-row: 1 / 3; width: 52px; height: 52px; }
  .boat .fname { align-self: end; color: var(--accent); }
  .boat .label { align-self: start; overflow: hidden; text-overflow: ellipsis; }
  .boat.shown { border-bottom-color: var(--accent); }
  .boat.shown .fname { color: var(--orange-lit); }
</style>
