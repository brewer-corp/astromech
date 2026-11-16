<script lang="ts" module>
  /** Demand against the hold: short below what you carry, tight below five holds, else plenty. */
  export const CARGO = 120;
  export type Depth = "short" | "tight" | "plenty";
  export const depth = (t: number): Depth => (t < CARGO ? "short" : t < 5 * CARGO ? "tight" : "plenty");
</script>

<script lang="ts">
  // PLACES > Sell, revised (tone 5): a result row reads as a result, not a record. Two lines:
  // price over its demand (the one orange column), the station with its pad and arrival over its
  // system, and the trip (jumps over light years). Report age leaves the row for the side panel,
  // except on a stale report, where it replaces the light years. Demand is coloured against the
  // 120 t in the hold; a pad the ship can't use is red-low and struck through.
  import Frame, { type RouteMode } from "./Frame.svelte";
  import Glyph, { type GlyphName } from "../../lib/ui/Glyph.svelte";

  let { route = "open", tipRow = "Cousteau Vista" }: { route?: RouteMode; tipRow?: string } = $props();

  type Row = { price: string; kind: GlyphName; kindName: string; place: string; system: string; demand: number; ly: string; jumps: string; pad: "L" | "M"; ls: string; age: number };
  const rows: Row[] = [
    { price: "211,587", kind: "coriolis", kindName: "Coriolis starport", place: "Good Landing", system: "LHS 1827", demand: 13884, ly: "18.8", jumps: "1", pad: "L", ls: "22,200", age: 27 },
    { price: "200,592", kind: "outpost", kindName: "Outpost", place: "Cousteau Vista", system: "LFT 65", demand: 96, ly: "32.4", jumps: "~2", pad: "M", ls: "2,404", age: 27 },
    { price: "200,592", kind: "orbis", kindName: "Orbis starport", place: "Radium Station", system: "LFT 65", demand: 26173, ly: "32.4", jumps: "~2", pad: "L", ls: "947", age: 27 },
    { price: "200,478", kind: "carrier", kindName: "Fleet carrier", place: "FCX-024", system: "LTT 3572", demand: 340, ly: "20.2", jumps: "1", pad: "L", ls: "215", age: 30 },
    { price: "195,192", kind: "outpost", kindName: "Outpost", place: "Tevis Terminal", system: "70 Ophiuchi", demand: 1828, ly: "16.6", jumps: "1", pad: "M", ls: "856", age: 29 },
    { price: "192,878", kind: "coriolis", kindName: "Coriolis starport", place: "Makhai High", system: "Caer Bran", demand: 29688, ly: "37.1", jumps: "~2", pad: "L", ls: "586", age: 27 },
    { price: "183,620", kind: "outpost", kindName: "Outpost", place: "Grandin Gateway", system: "Altair", demand: 5098, ly: "16.7", jumps: "1", pad: "M", ls: "288", age: 34 },
    { price: "117,548", kind: "planetary", kindName: "Planetary port", place: "Sabine Point", system: "Ross 79", demand: 210, ly: "35.6", jumps: "~2", pad: "L", ls: "2", age: 28 },
  ];
  const selected = "Good Landing";
  const STALE = 30;
  const fmt = (n: number) => n.toLocaleString("en-GB");
  const word: Record<Depth, string> = { short: "short", tight: "tight", plenty: "plenty" };
  const bars: Record<Depth, number> = { short: 1, tight: 2, plenty: 3 };
</script>

{#snippet meter(d: Depth)}
  <svg class="bars {d}" viewBox="0 0 14 12" width="14" height="12" aria-hidden="true">
    {#each [0, 1, 2] as i (i)}<rect x={i * 5} y={8 - i * 4} width="3" height={4 + i * 4} class:on={i < bars[d]} />{/each}
  </svg>
{/snippet}

<Frame top="Places" {route} skin="B" tone={5}>
  <div class="page">
    <section>
      <span class="crumb">Places</span>
      <div class="head">
        <h1 class="title">Sell</h1>
        <span class="cell input">Painite</span>
        <span class="aboard">{CARGO} t in your hold</span>
      </div>
      <p class="summary"><span>Within 100 ly of Sol · medium or large pad · any report age</span><span class="act">Change</span></p>

      <div class="list">
        <div class="row heads">
          <span class="r">Price · demand</span><span>Station · pad · arrival</span><span class="r">Trip</span><span></span>
        </div>
        {#each rows as r (r.place)}
          {@const d = depth(r.demand)}
          {@const stale = r.age > STALE}
          {@const unfit = r.pad === "M"}
          <div class="row" class:sel={r.place === selected} class:stale>
            <div class="money">
              <span class="price">{r.price}<span class="unit">cr</span></span>
              <span class="demand {d}" aria-label="Demand {fmt(r.demand)} t, {word[d]} for your {CARGO} t">
                {@render meter(d)}<span>{fmt(r.demand)}<span class="unit">t</span></span>
                {#if r.place === tipRow}
                  <span class="tip" role="tooltip"><b>{fmt(r.demand)} t demand</b>, less than the {CARGO} t you carry. The rest won't sell here, and the price falls as you sell.</span>
                {/if}
              </span>
            </div>
            <div class="where">
              <span class="l1">
                <span class="kind"><Glyph name={r.kind} label={r.kindName} /></span>
                <span class="pname">{r.place}</span>
                <span class="pad" class:unfit aria-label={unfit ? "Medium pad: too small for your Anaconda" : "Large pad"}>{r.pad}</span>
                <span class="ls">{r.ls}<span class="unit">ls</span></span>
              </span>
              <span class="l2"><span class="sys">{r.system}</span><span class="ibtn" aria-label="Copy {r.system}"><Glyph name="copy" /></span></span>
            </div>
            <div class="trip">
              <span class="jumps">{r.jumps}<span class="unit">{r.jumps === "1" ? "jump" : "jumps"}</span></span>
              {#if stale}<span class="age">{r.age} d old</span>{:else}<span class="ly">{r.ly}<span class="unit">ly</span></span>{/if}
            </div>
            <span class="acts"><span class="ibtn" aria-label="Add {r.system} as a stop"><Glyph name="add-stop" /></span><span class="ibtn" aria-label="Set {r.system} as the destination"><Glyph name="destination" /></span></span>
          </div>
        {/each}
      </div>
    </section>

    <aside>
      <h2 class="title">Good Landing</h2>
      <p class="in"><Glyph name="coriolis" label="Coriolis starport" /><span>LHS 1827</span><span class="ibtn" aria-label="Copy LHS 1827"><Glyph name="copy" /></span></p>
      <p class="key"><span class="vxl">211,587</span><span class="unit">cr</span></p>
      <p class="demand plenty big">{@render meter("plenty")}<span>13,884<span class="unit">t demand · plenty for your {CARGO} t</span></span></p>
      <dl>
        <dt>Pad</dt><dd>L</dd>
        <dt>Arrival</dt><dd>22,200<span class="unit">ls</span></dd>
        <dt>Trip</dt><dd>1<span class="unit">jump</span> · 18.8<span class="unit">ly</span></dd>
        <dt>Updated</dt><dd>27<span class="unit">d ago</span></dd>
      </dl>
      <div class="acts-panel"><span class="primary">Set destination</span><span class="act">Add stop</span></div>
    </aside>
  </div>
</Frame>

<style>
  .page { --o-lit: #ffa04a; --o: #ff7100; --o-mid: #d97a34; --g-hi: #e0e0e0; --g: #c4c4c4; --g-lo: #8c8c8c; --red-low: #c86455;
    display: grid; grid-template-columns: minmax(0, 1fr) var(--panel); gap: var(--sp-12); color: var(--g); font-weight: 400; }
  .unit { color: var(--g-lo); font-weight: 400; }
  .crumb { font-size: var(--fs-s); color: var(--o); text-decoration: underline 1px var(--o-mid); text-underline-offset: 3px; }
  .head { display: flex; align-items: center; gap: var(--sp-6); margin: var(--sp-2) 0 var(--sp-6); }
  .input { width: 320px; box-sizing: border-box; color: var(--o); }
  .aboard { font-size: var(--fs-s); color: var(--g-lo); }
  .summary { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-3); margin-bottom: var(--sp-4); font-size: var(--fs-s); color: var(--g-lo); }
  .summary .act { color: var(--o); }

  .list { width: max-content; max-width: 100%; }
  .row { display: grid; grid-template-columns: 136px minmax(280px, 400px) 96px 64px; align-items: center; column-gap: var(--sp-6); height: 60px; padding: 0 var(--sp-3); white-space: nowrap; }
  .row.heads { height: 28px; border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); font-size: var(--fs-s); color: var(--g-lo); }
  .money, .where, .trip { display: flex; flex-direction: column; justify-content: center; gap: 2px; min-width: 0; }
  .money, .trip { align-items: flex-end; }
  .price { font-size: var(--fs-l); font-weight: 600; line-height: 1.2; color: var(--o); }
  .demand { position: relative; display: inline-flex; align-items: center; gap: 6px; font-size: var(--fs-s); }
  .demand.plenty { color: var(--g); }
  .demand.tight { color: var(--o-mid); }
  .demand.short { color: var(--red-low); }
  .bars rect { fill: #3a3633; }
  .bars rect.on { fill: currentColor; }
  .l1, .l2 { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; }
  .kind { display: grid; color: var(--g-lo); }
  .pname { color: var(--g-hi); font-weight: 500; overflow: hidden; text-overflow: ellipsis; }
  .pad { display: inline-flex; align-items: center; height: 18px; padding: 0 5px; border: 1px solid var(--line); font-size: var(--fs-s); color: var(--g); }
  .pad.unfit { color: var(--red-low); border-color: var(--red-low); text-decoration: line-through; }
  .ls { font-size: var(--fs-s); color: var(--g); }
  .l2 { padding-left: 24px; }
  .sys { font-size: var(--fs-s); color: var(--g-lo); }
  .l2 .ibtn { width: 22px; height: 22px; margin-left: -4px; }
  .ibtn { color: var(--g-lo); }
  .jumps { color: var(--g); }
  .ly { font-size: var(--fs-s); color: var(--g-lo); }
  .age { font-size: var(--fs-s); color: var(--o); }
  .acts { display: flex; justify-content: flex-end; gap: var(--sp-1); }
  .row.stale .price { color: var(--o-mid); }
  .row.sel { background: var(--surface-hi); }
  .row.sel .price { color: var(--o-lit); }
  .row.sel .pname { color: #ffffff; }
  .row.sel .ibtn { color: var(--o-mid); }

  .tip { position: absolute; z-index: 40; left: -12px; top: calc(100% + 10px); width: 300px; padding: var(--sp-2) var(--sp-3); background: var(--surface-hi); border: 1px solid var(--o-mid); color: var(--g); font-size: var(--fs); line-height: 1.4; white-space: normal; text-align: left; }
  .tip b { color: #ffffff; font-weight: 600; }

  aside { border-left: 1px solid var(--line); padding-left: var(--sp-6); }
  aside .title { color: #ffffff; }
  .in { display: flex; align-items: center; gap: var(--sp-2); margin-top: var(--sp-1); color: var(--g); }
  .in > :global(svg) { color: var(--g-lo); }
  .key { display: flex; align-items: baseline; margin: var(--sp-6) 0 var(--sp-2); }
  .key .vxl { color: var(--o-lit); }
  .demand.big { font-size: var(--fs); }
  dl { display: grid; grid-template-columns: 80px 1fr; gap: var(--sp-2) var(--sp-3); margin: var(--sp-6) 0 var(--sp-8); color: var(--g); }
  dt { font-size: var(--fs-s); color: var(--g-lo); align-self: center; }
  .acts-panel { display: flex; align-items: center; gap: var(--sp-4); }
  .acts-panel .act { color: var(--o); }
  @container (max-width: 1300px) {
    .page { grid-template-columns: minmax(0, 1fr); }
    aside { border-left: none; border-top: 1px solid var(--line); padding: var(--sp-6) 0 0; }
  }
</style>
