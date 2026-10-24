<script lang="ts">
  // DESIGN.md mockup: one Places finder layout, two finders. Each has its own title, main input
  // and lead column (the value it ranks by); the row grammar, actions and states are shared.
  import Chrome from "./Chrome.svelte";
  import Copy from "./Copy.svelte";

  type Col = { head: string; w: string; key: string; unit?: string };
  type Row = { lead: string; place: string; system: string; warn?: string; stale?: boolean; [col: string]: string | boolean | undefined };
  type Finder = {
    title: string; input: string; inputLabel: string;
    lead: { head: string; unit: string };
    cols: Col[];
    constraints: [string, string[], number[]][];
    summary: string;
    rows: Row[];
    foot: string;
    strip: { next: string; of: string; flags?: string[]; warn?: string; dest: string; left: string };
  };

  let { kind }: { kind: "sell" | "mining" } = $props();

  const finders: Record<"sell" | "mining", Finder> = {
    sell: {
      title: "Sell", input: "Painite", inputLabel: "Commodity",
      lead: { head: "Price", unit: "cr" },
      cols: [
        { head: "Demand", w: "88px", key: "demand", unit: "t" },
        { head: "Distance", w: "88px", key: "ly", unit: "ly" },
        { head: "Jumps", w: "56px", key: "jumps" },
        { head: "Pad", w: "40px", key: "pad" },
        { head: "Arrival", w: "88px", key: "ls", unit: "ls" },
        { head: "Updated", w: "72px", key: "age", unit: "d" },
      ],
      constraints: [["Around", ["Here: Sol"], []], ["Pad", ["S", "M", "L"], [2]], ["Reports", ["Any age"], []], ["Carriers", ["Off", "On"], [0]]],
      summary: "Anywhere, distances from Sol · large pad · reports of any age · no carriers · you carry 64 t",
      rows: [
        { lead: "313,239", place: "Parekh Chemical Holdings", system: "Diaguandri", demand: "15", ly: "127.3", jumps: "7", pad: "L", ls: "775", age: "10", warn: "demand" },
        { lead: "254,118", place: "Mackenzie Relay", system: "Col 285 Sector SE-T b18-1", demand: "3,120", ly: "211.6", jumps: "11", pad: "L", ls: "1,422", age: "212", stale: true },
        { lead: "251,040", place: "Tasman Ring", system: "HIP 19934", demand: "940", ly: "158.0", jumps: "9", pad: "L", ls: "640", age: "2,046", stale: true },
        { lead: "240,692", place: "SavorgnandeBrazza Legacy", system: "Synuefe GW-Z a54-3", demand: "193", ly: "674.0", jumps: "34", pad: "L", ls: "184", age: "32" },
        { lead: "235,096", place: "Asgaard", system: "Agni", demand: "23,976", ly: "54.7", jumps: "3", pad: "L", ls: "430,000", age: "8" },
        { lead: "235,095", place: "Prior Depot", system: "Wregoe OI-C b54-6", demand: "1,285", ly: "399.7", jumps: "20", pad: "L", ls: "576", age: "9" },
        { lead: "232,930", place: "Borg Grob's Grog Dock", system: "Eva's Hush", demand: "11,441", ly: "84.7", jumps: "5", pad: "L", ls: "550", age: "8" },
        { lead: "229,410", place: "Ehrlich City", system: "Liaedin", demand: "2,604", ly: "61.2", jumps: "4", pad: "L", ls: "1,210", age: "3" },
        { lead: "226,880", place: "Gordon Terminal", system: "Hyades Sector DR-V c2-23", demand: "880", ly: "148.9", jumps: "8", pad: "L", ls: "2,045", age: "14" },
        { lead: "224,315", place: "Reilly Hub", system: "Kurumanit", demand: "4,410", ly: "92.4", jumps: "5", pad: "L", ls: "88", age: "1" },
        { lead: "221,002", place: "Shajn Market", system: "LTT 4487", demand: "6,903", ly: "30.1", jumps: "2", pad: "L", ls: "302", age: "5" },
        { lead: "219,770", place: "Hutton Orbital", system: "Alpha Centauri", demand: "312", ly: "4.4", jumps: "1", pad: "L", ls: "6,784,404", age: "21" },
      ],
      foot: "12 of 100 · prices from Ardent Insight",
      strip: { next: "Wregoe OI-C b54-6", of: "3 of 41", flags: ["Scoopable"], dest: "Colonia", left: "38 jumps left" },
    },
    mining: {
      title: "Mining hotspots", input: "Painite", inputLabel: "Mineral",
      lead: { head: "Hotspots", unit: "" },
      cols: [
        { head: "Also", w: "120px", key: "also" },
        { head: "Type", w: "80px", key: "ring" },
        { head: "Reserve", w: "80px", key: "reserve" },
        { head: "Distance", w: "88px", key: "ly", unit: "ly" },
        { head: "Jumps", w: "56px", key: "jumps" },
        { head: "Arrival", w: "88px", key: "ls", unit: "ls" },
      ],
      constraints: [["Around", ["Next stop: Wregoe OI-C b54-6"], []], ["Ring", ["Metallic", "Rocky", "Icy"], [0]], ["Reserve", ["Pristine", "Major", "Any"], [0]]],
      summary: "Around the next stop, Wregoe OI-C b54-6 · metallic rings · pristine reserves · within 100 ly",
      rows: [
        { lead: "3", place: "2 A Ring", system: "HIP 21991", also: "Platinum 1", ring: "Metallic", reserve: "Pristine", ly: "18.4", jumps: "1", ls: "1,204" },
        { lead: "3", place: "A 5 A Ring", system: "Wregoe OI-C b54-6", also: "—", ring: "Metallic", reserve: "Pristine", ly: "0.0", jumps: "0", ls: "3,912" },
        { lead: "2", place: "ABC 1 B Ring", system: "Synuefe ZL-J d10-109", also: "Platinum 2", ring: "Metallic", reserve: "Pristine", ly: "41.7", jumps: "3", ls: "880" },
        { lead: "2", place: "7 A Ring", system: "Col 285 Sector RD-S b18-0", also: "Osmium 1", ring: "Metallic", reserve: "Pristine", ly: "47.0", jumps: "3", ls: "212" },
        { lead: "2", place: "B 3 A Ring", system: "HIP 23716", also: "—", ring: "Metallic", reserve: "Pristine", ly: "63.5", jumps: "4", ls: "15,330", warn: "ls" },
        { lead: "1", place: "4 B Ring", system: "Hyades Sector GW-W c1-5", also: "Platinum 1", ring: "Metallic", reserve: "Pristine", ly: "70.2", jumps: "4", ls: "640" },
        { lead: "1", place: "1 A Ring", system: "HIP 20277", also: "—", ring: "Metallic", reserve: "Pristine", ly: "88.9", jumps: "5", ls: "92" },
        { lead: "1", place: "C 2 A Ring", system: "Pleiades Sector HR-W d1-79", also: "Rhodplumsite 1", ring: "Metallic", reserve: "Pristine", ly: "96.3", jumps: "6", ls: "1,775" },
      ],
      foot: "8 rings · hotspots from Spansh",
      strip: { next: "Wregoe OI-C b54-6", of: "3 of 41", warn: "Refuel before the next jump", dest: "Colonia", left: "38 jumps left" },
    },
  };
  const f = $derived(finders[kind]);
  const grid = $derived(`112px minmax(240px, 1fr) ${f.cols.map((c) => c.w).join(" ")} 184px`);
  const hover = 4;
</script>

<Chrome top="Galaxy" tabs={["Places", "Route", "System"]} tab="Places" strip={f.strip}>
  <div class="finder">
    <a class="crumb label" href="#/mockup/ds-places">Places</a>
    <div class="head">
      <h1 class="title">{f.title}</h1>
      <div class="main">
        <span class="label">{f.inputLabel}</span>
        <span class="cell input">{f.input}</span>
      </div>
    </div>
    <div class="filters">
      {#each f.constraints as [label, opts, on] (label)}
        <span class="label">{label}</span>
        {#if on.length}
          <span class="choices">{#each opts as o, i (o)}<button class="choice" class:on={on.includes(i)}>{o}</button>{/each}</span>
        {:else}
          <span class="cell dropdown">{opts[0]}<svg class="chev" viewBox="0 0 16 16" width="16" height="16"><path d="M4 6l4 4 4-4" fill="none" stroke="currentColor" /></svg></span>
        {/if}
      {/each}
    </div>
    <p class="label summary">{f.summary}</p>

    <div class="list" style="--cols: {grid}">
      <div class="row heads label">
        <span class="r">{f.lead.head}</span><span>{kind === "sell" ? "Station" : "Ring"}</span>
        {#each f.cols as c (c.key)}<span class:r={c.key !== "also" && c.key !== "ring" && c.key !== "reserve"}>{c.head}</span>{/each}
        <span></span>
      </div>
      {#each f.rows as r, i (r.place)}
        <div class="row" class:hover={i === hover} class:dim={r.stale}>
          <span class="r lead"><span class="num lit v">{r.lead}</span>{#if f.lead.unit}<span class="unit">{f.lead.unit}</span>{/if}</span>
          <span class="place"><span class="pname">{r.place}</span><span class="sys label">{r.system}</span><Copy /></span>
          {#each f.cols as c (c.key)}
            {@const text = c.key === "also" || c.key === "ring" || c.key === "reserve"}
            <span class:r={!text} class:num={!text} class:lo={text && r[c.key] === "—"}>
              {#if r.warn === c.key}<span class="bang">!</span>{/if}{r[c.key]}{#if c.unit && !text}<span class="unit">{c.unit}</span>{/if}
            </span>
          {/each}
          <span class="acts"><button class="act" class:quiet={i !== hover}>Add stop</button><button class="act" class:quiet={i !== hover}>Replace route</button></span>
        </div>
      {/each}
      <p class="label foot">{f.foot}{kind === "sell" ? " · 2 old reports dimmed" : ""}</p>
    </div>
  </div>
</Chrome>

<style>
  .finder { max-width: var(--list-w); }
  .crumb { color: var(--text-lo); text-decoration: underline; text-decoration-color: var(--line); text-underline-offset: 3px; }
  .head { display: flex; align-items: flex-end; gap: var(--sp-8); margin: var(--sp-1) 0 var(--sp-4); }
  .main { display: flex; align-items: center; gap: var(--sp-3); }
  .input { width: 320px; }
  .filters { display: flex; align-items: center; gap: var(--sp-3); flex-wrap: wrap; }
  .filters .label + .cell, .filters .label + .choices { margin-right: var(--sp-4); }
  .filters .dropdown { min-width: 160px; }
  .summary { margin: var(--sp-4) 0 var(--sp-4); }
  .row { display: grid; grid-template-columns: var(--cols); align-items: center; column-gap: var(--sp-3); height: var(--row-result); padding: 0 var(--sp-3); white-space: nowrap; }
  .row.heads { height: 28px; border-bottom: 1px solid var(--line); }
  .row.hover { background: var(--row-hover); }
  .r { text-align: right; }
  .lead .v { font-size: var(--fs-l); font-weight: 600; }
  .place { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; overflow: hidden; }
  .pname { font-weight: 500; overflow: hidden; text-overflow: ellipsis; }
  .sys { overflow: hidden; text-overflow: ellipsis; }
  .place :global(.copy) { margin-left: -6px; }
  .acts { display: flex; justify-content: flex-end; padding-left: var(--sp-3); }
  .foot { margin: var(--sp-3) var(--sp-3) 0; }
</style>
