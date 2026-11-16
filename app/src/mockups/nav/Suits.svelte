<script lang="ts">
  // SHIPS > Suits: the on-foot loadout as a ship is drawn on Ships. The loadout's suit and weapons
  // with their grade and modifications, the plan beside each, and the plan's shopping list; the side
  // panel explains the selected item's next step. The plan and its needs are the preview fixture's
  // (app/src/preview/fixtures/odyssey.json): Maverick grade 1 to 3, Night Vision, Karma P-15 to 2.
  import Frame from "./Frame.svelte";
  import Glyph from "../../lib/ui/Glyph.svelte";

  type Gear = { slot: string; name: string; grade: number; to?: number; mods: string[]; add?: string };
  const gear: Gear[] = [
    { slot: "Suit", name: "Maverick Suit", grade: 1, to: 3, mods: [], add: "Night Vision" },
    { slot: "Primary", name: "Karma C-44", grade: 3, mods: ["Greater Range", "Stability"] },
    { slot: "Primary", name: "Manticore Oppressor", grade: 2, mods: ["Magazine Size"] },
    { slot: "Secondary", name: "Karma P-15", grade: 1, to: 2, mods: [] },
  ];
  const selected = "Maverick Suit";

  // Item, need, have: the plan's three picks added up, short ones first.
  const list: [string, number, number][] = [
    ["Carbon Fibre Plating", 7, 0], ["NOC Data", 3, 0], ["Surveillance Logs", 3, 0], ["Health Monitor", 3, 1], ["Radioactivity Data", 3, 1], ["Surveillance Equipment", 5, 2],
    ["Graphene", 7, 9], ["Manufacturing Instructions", 4, 4], ["Suit Schematic", 3, 3], ["Circuit Switch", 5, 6], ["Tungsten Carbide", 2, 9], ["Weapon Component", 2, 3], ["Weapon Schematic", 1, 2], ["Compression-Liquefied Gas", 1, 4],
  ];
  const short = list.filter(([, n, h]) => h < n).length;
  const loadouts = [
    { name: "Scavenger", suit: "Maverick", g: 1 },
    { name: "Assault", suit: "Dominator", g: 3 },
    { name: "Survey", suit: "Artemis", g: 2 },
    { name: "Spare", suit: "Dominator", g: 1 },
  ];
</script>

{#snippet pips(g: number, to?: number)}
  <span class="pips">{#each [1, 2, 3, 4, 5] as i (i)}<span class:on={i <= g} class:plan={!!to && i > g && i <= to}></span>{/each}</span>
{/snippet}

<Frame top="Ships" tabs={["Ships", "Stored modules", "Suits"]} tab="Suits">
  <div class="page">
    <section class="main">
      <div class="who">
        <h1 class="title">Scavenger</h1>
        <p class="label">Loadout 1 of 4 · Maverick Suit · 3 of 4 items engineered or planned</p>
      </div>

      <h2 class="section current">Loadout</h2>
      <div class="slots">
        <div class="slot heads label"><span>Slot</span><span>Item</span><span>Grade</span><span>Modifications</span><span>Plan</span></div>
        {#each gear as s (s.name)}
          <div class="slot" class:sel={s.name === selected}>
            <span class="label sl">{s.slot}</span>
            <span class="item" class:eng={s.grade > 1 || s.mods.length}>{s.name}</span>
            {@render pips(s.grade, s.to)}
            <span class="mods">{s.mods.join(" · ") || "None"}</span>
            <span class="plan">{[s.to ? `Grade ${s.to}` : "", s.add ?? ""].filter(Boolean).join(" · ")}</span>
          </div>
        {/each}
      </div>

      <h2 class="section">Shopping list<span class="sub label">4,050,000 cr · {short} of {list.length} short</span></h2>
      <div class="shop">
        <div class="line heads label"><span>Item</span><span class="r">Need</span><span class="r">Have</span></div>
        <div class="line heads label two"><span>Item</span><span class="r">Need</span><span class="r">Have</span></div>
        {#each list as [name, n, h] (name)}
          <div class="line" class:done={h >= n}>
            <span>{name}</span><span class="r">{n}</span><span class="r">{#if h < n}<span class="bang">!</span>{/if}{h}</span>
          </div>
        {/each}
      </div>

      <h2 class="section">Loadouts</h2>
      <div class="outs">
        {#each loadouts as l, i (l.name)}
          <div class="out" class:shown={i === 0}><span class="oname">{l.name}</span><span class="label">{l.suit} · grade {l.g}</span></div>
        {/each}
      </div>
    </section>

    <aside>
      <h2 class="title">Maverick Suit</h2>
      <p class="label">Grade 1 · plan: grade 3 and Night Vision</p>
      <p class="key"><span class="vxl white">2,850,000</span><span class="unit">cr to grade 3</span></p>
      <h3 class="section">Grades 2 and 3 need</h3>
      <div class="needs">
        <span class="label">Item</span><span class="label r">Need</span><span class="label r">Have</span>
        <span>Carbon Fibre Plating</span><span class="r">7</span><span class="r"><span class="bang">!</span>0</span>
        <span>Graphene</span><span class="r">7</span><span class="r">9</span>
        <span>Health Monitor</span><span class="r">3</span><span class="r"><span class="bang">!</span>1</span>
        <span>Manufacturing Instructions</span><span class="r">3</span><span class="r">4</span>
        <span>Suit Schematic</span><span class="r">3</span><span class="r">3</span>
      </div>
      <p class="label at">At Pioneer Supplies, in any Odyssey station</p>
      <h3 class="section">Night Vision</h3>
      <p class="eng"><span class="ename">Oden Geiger</span><span class="label">Candiaei</span><span class="ibtn" aria-label="Copy Candiaei"><Glyph name="copy" /></span><span class="ibtn" aria-label="Set Candiaei as the destination"><Glyph name="destination" /></span></p>
      <p class="label">Unlocked · 1,000,000 cr · 5 items, 4 short</p>
    </aside>
  </div>
</Frame>

<style>
  .page { display: grid; grid-template-columns: minmax(0, 1fr) var(--panel); column-gap: var(--sp-12); }
  .who { margin-bottom: var(--sp-8); }
  .who .label { margin-top: var(--sp-1); }
  .section { margin: 0 0 var(--sp-3); }
  .section .sub { margin-left: var(--sp-2); font-weight: 400; letter-spacing: normal; text-transform: none; }
  .slots { display: grid; gap: var(--gap); margin-bottom: var(--sp-8); max-width: 980px; }
  .slot { display: grid; grid-template-columns: 80px minmax(150px, 1.2fr) 56px minmax(0, 1.4fr) minmax(0, 1fr); align-items: center; column-gap: var(--sp-3); height: var(--row); padding: 0 var(--sp-3); background: var(--surface); white-space: nowrap; color: var(--accent); }
  .slot.heads { background: none; height: 24px; }
  .item { overflow: hidden; text-overflow: ellipsis; }
  .item.eng { color: var(--text-hi); }
  .mods, .plan { overflow: hidden; text-overflow: ellipsis; }
  .plan { color: var(--orange-lit); }
  .pips > span.plan { border-color: var(--orange-lit); }
  .slot.sel { background: var(--accent); }
  .slot.sel * { color: var(--on-accent) !important; }
  .slot.sel .pips > span { border-color: var(--on-accent); }
  .slot.sel .pips > span.on { background: var(--on-accent); }

  .shop { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); column-gap: var(--sp-8); row-gap: 0; margin-bottom: var(--sp-8); max-width: 980px; grid-auto-flow: row; }
  .line { display: grid; grid-template-columns: minmax(0, 1fr) 48px 48px; column-gap: var(--sp-3); align-items: center; height: 30px; color: var(--accent); border-bottom: 1px solid var(--line); white-space: nowrap; }
  .line.heads { height: 24px; color: var(--orange-mid); }
  .line.done { color: var(--orange-mid); }
  .outs { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: var(--sp-2); max-width: 980px; }
  .out { display: flex; flex-direction: column; justify-content: center; height: 52px; padding: 0 var(--sp-3); background: var(--surface); }
  .oname { color: var(--accent); }
  .out.shown { background: var(--accent); }
  .out.shown * { color: var(--on-accent); }

  aside { border-left: 1px solid var(--accent); padding-left: var(--sp-6); }
  aside > .label { margin-top: var(--sp-1); }
  .key { display: flex; align-items: baseline; margin: var(--sp-6) 0 var(--sp-6); }
  .needs { display: grid; grid-template-columns: 1fr 40px 40px; gap: var(--sp-2) var(--sp-3); color: var(--accent); margin-bottom: var(--sp-3); }
  .at { margin-bottom: var(--sp-8) !important; }
  .eng { display: flex; align-items: center; gap: var(--sp-2); }
  .ename { color: var(--accent); }
  @container (max-width: 1000px) {
    .page { grid-template-columns: minmax(0, 1fr); }
    aside { border-left: none; border-top: 1px solid var(--accent); padding: var(--sp-6) 0 0; }
  }
</style>
