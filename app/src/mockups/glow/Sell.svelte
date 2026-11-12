<script lang="ts">
  // Glow-up mockup: a Places finder, Sell a commodity (List with a detail panel). The filter rows
  // collapse into the one summary line with CHANGE; the result rows follow DESIGN.md's lookup row
  // (price first and largest). One row selected, explained by the panel; the hovered row's add
  // stop icon shows its Tip in place of a native tooltip.
  import Frame from "./Frame.svelte";
  import Copy from "./Copy.svelte";
  import Icon, { type IconName } from "./Icon.svelte";
  import Tip from "./Tip.svelte";

  type Row = { price: string; kind: IconName; kindName: string; place: string; system: string; demand: string; ly: string; jumps: string; pad: string; ls: string; age: string; badge?: string; far?: boolean };
  const rows: Row[] = [
    { price: "58,960", kind: "coriolis", kindName: "Coriolis starport", place: "Tasman Ring", system: "HIP 19934", demand: "2,410", ly: "61.2", jumps: "4", pad: "L", ls: "640", age: "3" },
    { price: "58,410", kind: "orbis", kindName: "Orbis starport", place: "Reilly Hub", system: "Kurumanit", demand: "940", ly: "92.4", jumps: "5", pad: "L", ls: "88", age: "1" },
    { price: "57,215", kind: "outpost", kindName: "Outpost", place: "Gordon Terminal", system: "Hyades Sector DR-V c2-23", demand: "312", ly: "48.9", jumps: "3", pad: "M", ls: "2,045", age: "6" },
    { price: "56,990", kind: "ocellus", kindName: "Ocellus starport", place: "Shajn Market", system: "LTT 4487", demand: "6,903", ly: "30.1", jumps: "2", pad: "L", ls: "302", age: "5" },
    { price: "56,402", kind: "planetary", kindName: "Planetary port", place: "Asgaard", system: "Agni", demand: "1,285", ly: "54.7", jumps: "3", pad: "L", ls: "4,300", age: "8" },
    { price: "55,870", kind: "coriolis", kindName: "Coriolis starport", place: "Borg Grob's Grog Dock", system: "Eva's Hush", demand: "11,441", ly: "84.7", jumps: "5", pad: "L", ls: "550", age: "8" },
    { price: "55,410", kind: "outpost", kindName: "Outpost", place: "Haxel Port", system: "Wolf 397", demand: "418", ly: "12.0", jumps: "1", pad: "M", ls: "1,120", age: "2" },
    { price: "54,980", kind: "outpost", kindName: "Outpost", place: "Hutton Orbital", system: "Alpha Centauri", demand: "312", ly: "4.4", jumps: "1", pad: "M", ls: "6,784,404", age: "21", far: true },
    { price: "54,300", kind: "orbis", kindName: "Orbis starport", place: "Abraham Lincoln", system: "Sol", demand: "24,880", ly: "0.0", jumps: "0", pad: "L", ls: "504", age: "1", badge: "Permit" },
    { price: "53,870", kind: "coriolis", kindName: "Coriolis starport", place: "Trevithick Dock", system: "LHS 3447", demand: "3,302", ly: "30.6", jumps: "2", pad: "L", ls: "8,300", age: "4" },
    { price: "53,120", kind: "orbis", kindName: "Orbis starport", place: "Ackerman Market", system: "Eravate", demand: "5,720", ly: "41.7", jumps: "2", pad: "L", ls: "260", age: "12" },
    { price: "52,780", kind: "asteroid", kindName: "Asteroid base", place: "Mitchell Dock", system: "Ross 128", demand: "1,104", ly: "11.0", jumps: "1", pad: "M", ls: "1,982", age: "9" },
  ];
  const selected = "Shajn Market";
  const hover = "Ackerman Market";
</script>

<Frame top="Galaxy" tabs={["Places", "Route"]} tab="Places"
  strip={{ next: "Wregoe OI-C b54-6", of: "4 of 10", flags: ["Scoopable"], dest: "Synuefe GW-Z a54-3", left: "6 jumps left" }}>
  <div class="page">
    <section class="main">
      <a class="crumb label" href="#/mockup/glow-sell">Places</a>
      <div class="head">
        <h1 class="title">Sell</h1>
        <span class="cell input">Palladium</span>
        <span class="label">You carry 128 t, bought at 49,820 cr</span>
      </div>
      <p class="summary"><span class="label">Within 100 ly of Sol · medium pad · reports up to 30 days old</span><button class="act">Change</button></p>

      <div class="list">
        <div class="row heads label">
          <span class="r">Price</span><span>Station</span><span class="r">Demand</span><span class="r">Distance</span><span class="r">Jumps</span><span class="r">Pad</span><span class="r">Arrival</span><span class="r">Updated</span><span></span>
        </div>
        {#each rows as r (r.place)}
          <div class="row" class:sel={r.place === selected} class:hover={r.place === hover}>
            <span class="r"><span class="price">{r.price}</span><span class="unit">cr</span></span>
            <span class="place">
              <span class="kind"><Icon name={r.kind} label={r.kindName} /></span>
              <span class="pname">{r.place}</span><span class="sys label">{r.system}</span><Copy />
              {#if r.badge}<span class="badge">{r.badge}</span>{/if}
            </span>
            <span class="r">{r.demand}<span class="unit">t</span></span>
            <span class="r">{r.ly}<span class="unit">ly</span></span>
            <span class="r">{r.jumps}</span>
            <span class="r">{r.pad}</span>
            <span class="r">{#if r.far}<span class="bang">!</span>{/if}{r.ls}<span class="unit">ls</span></span>
            <span class="r">{r.age}<span class="unit">d</span></span>
            <span class="acts">
              <button class="ibtn" aria-label="Add {r.system} as a stop"><Icon name="add-stop" />
                {#if r.place === hover}<Tip side="above" end width={320}>Add Eravate as a stop</Tip>{/if}
              </button>
              <button class="ibtn" aria-label="Set {r.system} as the destination"><Icon name="destination" /></button>
            </span>
          </div>
        {/each}
        <p class="label foot">12 of 64 stations · prices from Ardent Insight</p>
      </div>
    </section>

    <aside>
      <h2 class="title">Shajn Market</h2>
      <p class="where"><span class="label">Ocellus starport in</span><span class="sys">LTT 4487</span><Copy /></p>
      <p class="key"><span class="vxl white">56,990</span><span class="unit">cr per t</span></p>
      <p class="gain"><span class="lit">917,760</span><span class="unit">cr profit on your 128 t</span></p>
      <p class="label">741,120 cr more than selling here</p>
      <dl>
        <dt class="label">Demand</dt><dd>6,903<span class="unit">t</span></dd>
        <dt class="label">Distance</dt><dd>30.1<span class="unit">ly</span> · 2<span class="unit">jumps</span></dd>
        <dt class="label">Arrival</dt><dd>302<span class="unit">ls</span> · large pad</dd>
        <dt class="label">Updated</dt><dd>5 days ago</dd>
        <dt class="label">Also here</dt><dd>Refuel, repair, material trader</dd>
      </dl>
      <div class="acts-panel"><button class="primary">Set destination</button><button class="act">Add stop</button></div>
    </aside>
  </div>
</Frame>

<style>
  .page { display: grid; grid-template-columns: minmax(0, 1fr) var(--panel); gap: var(--sp-12); max-width: 1760px; }
  .crumb { color: var(--orange-mid); text-decoration: underline; text-decoration-color: var(--line); text-underline-offset: 3px; }
  .head { display: flex; align-items: center; gap: var(--sp-6); margin: var(--sp-1) 0 var(--sp-4); }
  .input { width: 320px; }
  .summary { display: flex; align-items: center; gap: var(--sp-3); margin-bottom: var(--sp-4); }

  .list { width: max-content; max-width: 100%; }
  .row { display: grid; grid-template-columns: 120px minmax(260px, 400px) 88px 88px 56px 40px 104px 72px 64px; align-items: center; column-gap: var(--sp-3); height: var(--row-result); padding: 0 var(--sp-3); white-space: nowrap; color: var(--accent); }
  .row.heads { height: 28px; border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); color: var(--orange-mid); }
  .row.hover { background: var(--row-hover); }
  .row.hover .ibtn { color: var(--orange-lit); }
  .price { font-size: var(--fs-l); font-weight: 600; color: var(--orange-lit); }
  .place { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; }
  .kind { color: var(--orange-mid); margin-right: var(--sp-1); }
  .pname { color: var(--accent); overflow: hidden; text-overflow: ellipsis; }
  .sys { overflow: hidden; text-overflow: ellipsis; }
  .place .badge { margin-left: var(--sp-1); }
  .acts { display: flex; justify-content: flex-end; gap: var(--sp-1); }
  .row.sel { background: var(--accent); }
  .row.sel, .row.sel :global(*) { color: var(--on-accent) !important; }
  .row.sel .badge { border-color: var(--on-accent); }
  .foot { margin: var(--sp-3) var(--sp-3) 0; }

  aside { border-left: 1px solid var(--accent); padding-left: var(--sp-6); }
  .where { display: flex; align-items: baseline; gap: var(--sp-2); margin-top: var(--sp-1); }
  .where :global(.copy) { align-self: center; }
  .sys { color: var(--orange-mid); }
  aside .sys { color: var(--accent); }
  .key { display: flex; align-items: baseline; margin: var(--sp-6) 0 var(--sp-2); }
  .gain { display: flex; align-items: baseline; }
  .gain .lit { font-size: var(--fs-l); font-weight: 600; }
  dl { display: grid; grid-template-columns: 88px 1fr; gap: var(--sp-2) var(--sp-3); margin: var(--sp-8) 0; color: var(--accent); }
  dt { align-self: center; }
  .acts-panel { display: flex; gap: var(--sp-4); }
</style>
