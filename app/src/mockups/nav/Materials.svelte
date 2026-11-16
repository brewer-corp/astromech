<script lang="ts">
  // ENGINEERING > Materials. Raw is today's trader grid. On foot is the Odyssey inventory as one
  // more kind: it has no grades, so its rows are the ShipLocker's own groups (components, goods,
  // data), and it shows only what is held or wanted by the suit plan. Counts and needs are the
  // preview fixture's (app/src/preview/fixtures/odyssey.json).
  import Frame, { type RouteMode } from "./Frame.svelte";

  let { kind = "raw", route = "open" }: { kind?: "raw" | "foot"; route?: RouteMode } = $props();

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

  type Item = [string, number, number?];
  const foot: [string, Item[]][] = [
    ["Components", [["Carbon Fibre Plating", 0, 7], ["Graphene", 9, 7], ["Circuit Switch", 6, 5], ["Tungsten Carbide", 9, 2], ["Weapon Component", 3, 2], ["Epoxy Adhesive", 11], ["Titanium Plating", 7], ["Micro Supercapacitor", 5], ["Aerogel", 4], ["Optical Fibre", 3], ["Chemical Superbase", 2], ["Electrical Wiring", 1]]],
    ["Goods", [["Health Monitor", 1, 3], ["Surveillance Equipment", 2, 5], ["Suit Schematic", 3, 3], ["Weapon Schematic", 2, 1], ["Compression-Liquefied Gas", 4, 1], ["Ionised Gas", 6], ["Power Regulator", 2]]],
    ["Data", [["NOC Data", 0, 3], ["Radioactivity Data", 1, 3], ["Surveillance Logs", 0, 3], ["Manufacturing Instructions", 4, 4], ["Combat Training Material", 5], ["Weapon Inventory", 3], ["Topographical Surveys", 2], ["Ballistics Data", 1]]],
  ];
  const items = foot.flatMap(([, l]) => l);
  const held = $derived(kind === "raw" ? grid.flat().reduce((s, [, n]) => s + n, 0) : items.reduce((s, [, n]) => s + n, 0));
  const wanted = $derived(kind === "raw" ? Object.keys(need).length : items.filter((i) => i[2]).length);
  const short = $derived(kind === "raw" ? grid.flat().filter(([m, n]) => n < (need[m] ?? 0)).length : items.filter(([, n, w]) => w && n < w).length);
  const selected = $derived(kind === "foot" ? "Carbon Fibre Plating" : "");
</script>

{#snippet tile(name: string, n: number, want: number | undefined, cap: number | null)}
  <div class="tile" class:none={n === 0} class:sel={name === selected}>
    <span class="count">{n}</span>
    <span class="name">{name}</span>
    {#if want}<span class="need">{#if n < want}<span class="bang">!</span>{/if}need {want}</span>{/if}
    {#if cap && n > 0}<span class="track"><span class="fill" class:full={n >= cap} style="width: {(100 * n) / cap}%"></span></span>{/if}
  </div>
{/snippet}

<Frame top="Engineering" tabs={["Materials", "Engineers"]} tab="Materials" {route}>
  <div class="page" class:detail={!!selected}>
    <section>
      <div class="bar">
        <span class="choices">
          <span class="choice" class:on={kind === "raw"}>Raw</span><span class="choice">Manufactured</span><span class="choice">Encoded</span><span class="choice" class:on={kind === "foot"}>On foot</span>
        </span>
        <span class="label">{held.toLocaleString("en-GB")} held · {wanted} wanted by your plans, {short} short</span>
      </div>

      {#if kind === "raw"}
        <div class="grid">
          <span></span>
          {#each caps as cap, g (g)}<span class="label colhead">Grade {g + 1} · max {cap}</span>{/each}
          {#each grid as row, c (c)}
            <span class="label">Category {c + 1}</span>
            {#each row as [name, n], g (name)}{@render tile(name, n, need[name], caps[g] ?? null)}{/each}
          {/each}
        </div>
      {:else}
        {#each foot as [group, list] (group)}
          <h2 class="section" class:current={list.some(([m]) => m === selected)}>{group}</h2>
          <div class="tiles">{#each list as [name, n, want] (name)}{@render tile(name, n, want, null)}{/each}</div>
        {/each}
      {/if}
    </section>

    {#if selected}
      <aside>
        <h2 class="title">Carbon Fibre Plating</h2>
        <p class="label">On foot · component</p>
        <p class="key"><span class="vxl white">0</span><span class="unit">held · your suit plan needs 7</span></p>
        <h3 class="section">Wanted by</h3>
        <div class="wants">
          <span>Maverick Suit<span class="label sub">Grade 1 to 3</span></span><span class="r">7</span>
        </div>
        <h3 class="section">Where to get it</h3>
        <ul class="get">
          <li>Industrial settlements, in lockers and containers</li>
          <li>A bartender, for other goods</li>
        </ul>
      </aside>
    {/if}
  </div>
</Frame>

<style>
  .page { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--sp-12); }
  .page.detail { grid-template-columns: minmax(0, 1fr) var(--panel); }
  .bar { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-6); margin-bottom: var(--sp-6); }
  .grid { display: grid; grid-template-columns: 88px repeat(4, minmax(160px, 240px)); gap: var(--sp-2); align-items: center; }
  .colhead { padding: 0 var(--sp-3); }
  .tiles { display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: var(--sp-2); margin-bottom: var(--sp-8); max-width: calc(5 * 240px + 4 * 8px); }
  .section { margin-bottom: var(--sp-3); }
  .tile { position: relative; height: var(--tile); box-sizing: border-box; padding: var(--sp-2) var(--sp-3); background: var(--surface); display: grid; grid-template-rows: auto 1fr; }
  .count { font-size: var(--fs-xxl); font-weight: 500; line-height: 1; color: var(--orange-lit); }
  .name { align-self: end; font-size: var(--fs-s); color: var(--accent); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .need { position: absolute; top: var(--sp-2); right: var(--sp-3); font-size: var(--fs-xs); color: var(--accent); }
  .need .bang { margin-right: 3px; }
  .track { position: absolute; left: 0; right: 0; bottom: 0; height: 2px; background: var(--line); }
  .fill { display: block; height: 100%; background: var(--orange-mid); }
  .fill.full { background: var(--orange-lit); }
  .tile.none { background: var(--bg); outline: 1px solid var(--line); outline-offset: -1px; }
  .tile.none .count, .tile.none .name { color: var(--orange-dim); }
  .tile.none .need { color: var(--accent); }
  .tile.sel { background: var(--accent); outline: none; }
  .tile.sel .count, .tile.sel .name, .tile.sel .need, .tile.sel .bang { color: var(--on-accent); }

  aside { border-left: 1px solid var(--accent); padding-left: var(--sp-6); }
  aside > .label { margin-top: var(--sp-1); }
  .key { display: flex; align-items: baseline; margin: var(--sp-6) 0 var(--sp-8); }
  .wants { display: grid; grid-template-columns: 1fr auto; gap: var(--sp-3); color: var(--accent); margin-bottom: var(--sp-8); }
  .wants .sub { display: block; }
  .get { display: grid; gap: var(--sp-2); color: var(--accent); margin-bottom: var(--sp-4); }
  @container (max-width: 1000px) {
    .page.detail { grid-template-columns: minmax(0, 1fr); }
    aside { border-left: none; border-top: 1px solid var(--accent); padding: var(--sp-6) 0 0; }
  }
</style>
