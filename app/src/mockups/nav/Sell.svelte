<script lang="ts">
  // PLACES > Sell with a result selected, beside the route sidebar: the left sidebar and the right
  // detail panel on screen together. Rows are the real Sell page's (docs/design/increment1).
  import Frame, { type RouteMode, type Skin } from "./Frame.svelte";
  import Glyph, { type GlyphName } from "../../lib/ui/Glyph.svelte";

  let { route = "open", skin = "page", tone = 0 }: { route?: RouteMode; skin?: Skin; tone?: number } = $props();

  type Row = { price: string; kind: GlyphName; kindName: string; place: string; system: string; demand: string; ly: string; jumps: string; pad: string; ls: string; age: string };
  const rows: Row[] = [
    { price: "211,587", kind: "coriolis", kindName: "Coriolis starport", place: "Good Landing", system: "LHS 1827", demand: "13,884", ly: "18.8", jumps: "1", pad: "L", ls: "22,200", age: "27" },
    { price: "200,592", kind: "outpost", kindName: "Outpost", place: "Cousteau Vista", system: "LFT 65", demand: "1,960", ly: "32.4", jumps: "~2", pad: "M", ls: "2,404", age: "27" },
    { price: "200,592", kind: "orbis", kindName: "Orbis starport", place: "Radium Station", system: "LFT 65", demand: "26,173", ly: "32.4", jumps: "~2", pad: "L", ls: "947", age: "27" },
    { price: "200,478", kind: "carrier", kindName: "Fleet carrier", place: "FCX-024", system: "LTT 3572", demand: "1,343", ly: "20.2", jumps: "1", pad: "L", ls: "215", age: "30" },
    { price: "195,192", kind: "outpost", kindName: "Outpost", place: "Tevis Terminal", system: "70 Ophiuchi", demand: "1,828", ly: "16.6", jumps: "1", pad: "M", ls: "856", age: "29" },
    { price: "192,878", kind: "coriolis", kindName: "Coriolis starport", place: "Makhai High", system: "Caer Bran", demand: "29,688", ly: "37.1", jumps: "~2", pad: "L", ls: "586", age: "27" },
    { price: "183,620", kind: "outpost", kindName: "Outpost", place: "Grandin Gateway", system: "Altair", demand: "5,098", ly: "16.7", jumps: "1", pad: "M", ls: "288", age: "34" },
    { price: "117,548", kind: "planetary", kindName: "Planetary port", place: "Sabine Point", system: "Ross 79", demand: "1,670", ly: "35.6", jumps: "~2", pad: "L", ls: "2", age: "28" },
  ];
  const selected = "Good Landing";
</script>

<Frame top="Places" {route} {skin} {tone}>
  <div class="page">
    <section>
      <span class="crumb label">Places</span>
      <div class="head">
        <h1 class="title">Sell</h1>
        <span class="cell input">Painite</span>
      </div>
      <p class="summary"><span class="label">Within 100 ly of Sol · medium or large pad · reports up to 30 days old</span><span class="act">Change</span></p>
      <div class="list">
        <div class="row heads label">
          <span class="r">Price</span><span>Station</span><span class="r">Demand</span><span class="r">Distance</span><span class="r">Jumps</span><span class="r">Pad</span><span class="r">Arrival</span><span class="r">Updated</span><span></span>
        </div>
        {#each rows as r (r.place)}
          <div class="row" class:sel={r.place === selected}>
            <span class="r"><span class="price">{r.price}</span><span class="unit">cr</span></span>
            <span class="place">
              <span class="kind"><Glyph name={r.kind} label={r.kindName} /></span>
              <span class="pname">{r.place}</span><span class="sys label">{r.system}</span><span class="ibtn" aria-label="Copy {r.system}"><Glyph name="copy" /></span>
            </span>
            <span class="r">{r.demand}<span class="unit">t</span></span>
            <span class="r">{r.ly}<span class="unit">ly</span></span>
            <span class="r">{r.jumps}</span>
            <span class="r">{r.pad}</span>
            <span class="r">{r.ls}<span class="unit">ls</span></span>
            <span class="r">{r.age}<span class="unit">d</span></span>
            <span class="acts"><span class="ibtn" aria-label="Add {r.system} as a stop"><Glyph name="add-stop" /></span><span class="ibtn" aria-label="Set {r.system} as the destination"><Glyph name="destination" /></span></span>
          </div>
        {/each}
      </div>
    </section>

    <aside>
      <h2 class="title">Good Landing</h2>
      <p class="where"><span class="label">In</span><span class="sys">LHS 1827</span><span class="ibtn" aria-label="Copy LHS 1827"><Glyph name="copy" /></span></p>
      <p class="key"><span class="vxl white">211,587</span><span class="unit">cr</span></p>
      <dl>
        <dt class="label">Demand</dt><dd>13,884<span class="unit">t</span></dd>
        <dt class="label">Distance</dt><dd>18.8<span class="unit">ly</span> · 1<span class="unit">jump</span></dd>
        <dt class="label">Pad</dt><dd>L</dd>
        <dt class="label">Arrival</dt><dd>22,200<span class="unit">ls</span></dd>
        <dt class="label">Updated</dt><dd>27<span class="unit">d</span></dd>
      </dl>
      <div class="acts-panel"><span class="primary">Set destination</span><span class="act">Add stop</span></div>
    </aside>
  </div>
</Frame>

<style>
  .page { display: grid; grid-template-columns: minmax(0, 1fr) var(--panel); gap: var(--sp-12); }
  .crumb { text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; color: var(--accent); }
  .head { display: flex; align-items: center; gap: var(--sp-6); margin: var(--sp-2) 0 var(--sp-6); }
  .input { width: 320px; box-sizing: border-box; }
  .summary { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-3); margin-bottom: var(--sp-4); }

  .list { width: max-content; max-width: 100%; }
  .row { display: grid; grid-template-columns: 112px minmax(240px, 360px) 80px 80px 52px 36px 84px 64px 64px; align-items: center; column-gap: var(--sp-3); height: var(--row-result); padding: 0 var(--sp-3); white-space: nowrap; color: var(--accent); }
  .row.heads { height: 28px; border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); color: var(--orange-mid); }
  .price { font-size: var(--fs-l); font-weight: 600; color: var(--orange-lit); }
  .place { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; }
  .kind { color: var(--orange-mid); display: grid; }
  .pname { overflow: hidden; text-overflow: ellipsis; }
  .sys { overflow: hidden; text-overflow: ellipsis; }
  .acts { display: flex; justify-content: flex-end; gap: var(--sp-1); }
  .row.sel { background: var(--accent); }
  .row.sel, .row.sel :global(*) { color: var(--on-accent) !important; }

  aside { border-left: 1px solid var(--accent); padding-left: var(--sp-6); }
  .where { display: flex; align-items: center; gap: var(--sp-2); margin-top: var(--sp-1); }
  aside .sys { color: var(--accent); }
  .key { display: flex; align-items: baseline; margin: var(--sp-6) 0 var(--sp-2); }
  dl { display: grid; grid-template-columns: 80px 1fr; gap: var(--sp-2) var(--sp-3); margin: var(--sp-6) 0 var(--sp-8); color: var(--accent); }
  .acts-panel { display: flex; align-items: center; gap: var(--sp-4); }
  @container (max-width: 1300px) {
    .page { grid-template-columns: minmax(0, 1fr); }
    aside { border-left: none; border-top: 1px solid var(--accent); padding: var(--sp-6) 0 0; }
  }
</style>
