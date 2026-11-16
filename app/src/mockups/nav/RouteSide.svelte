<script lang="ts">
  // The Route page as a 320 px sidebar. What changes from the page, so it fits 264 px of text:
  // - The title wraps; the Plotter and Worked out dropdowns lose their labels (Tip names them).
  // - REPLOT FROM HERE is a text action: the sidebar is on every page, which has its own primary.
  // - The star type word goes; the node's icon says it, and its Tip names the star and the system.
  // - Names truncate with an ellipsis; the next stop's name wraps instead, since it is typed in the
  //   galaxy map. The inhabited line keeps allegiance, security and the faction state.
  import Glyph from "../../lib/ui/Glyph.svelte";
  import { ROUTE, ROWS } from "./route";

  let { overlay = false, tips = false, frame = false }: { overlay?: boolean; tips?: boolean; frame?: boolean } = $props();
  const first = 0, last = ROWS.length - 1;
</script>

<div class="side" class:frame>
  {#if frame}
    <!-- A frame heading, not a page Title: the page keeps the only Title on screen. -->
    <div class="fhead">
      <span class="flabel">Route</span>
      <span class="ibtn fold" aria-label={overlay ? "Close the route" : "Collapse the route to a strip"}><span class="chev"><Glyph name="chevron" /></span></span>
    </div>
    <div class="dline"><span>{ROUTE.from}</span> <span class="to">to</span> <span class="dest">{ROUTE.to}</span></div>
  {:else}
  <div class="head">
    <h1 class="title"><span>{ROUTE.from}</span> <span class="to">to</span> <span class="dest">{ROUTE.to}</span></h1>
    <span class="ibtn fold" aria-label={overlay ? "Close the route" : "Collapse the route to a strip"}><span class="chev"><Glyph name="chevron" /></span></span>
  </div>
  {/if}
  <div class="tools">
    <span class="cell dd" aria-label="Plotter">{ROUTE.plotter}<Glyph name="chevron" /></span>
    <span class="cell dd" aria-label="Worked out">{ROUTE.by}<Glyph name="chevron" /></span>
    <span class="ibtn" aria-label="How the plotters differ"><Glyph name="help" /></span>
    <span class="ibtn" aria-label="More route actions"><Glyph name="more" /></span>
  </div>
  <div class="sum"><span class="label">{ROUTE.n} of {ROUTE.of} · {ROUTE.left} jumps left</span><span class="act">Replot from here</span></div>

  <ol class="tl">
    {#each ROWS as r, i (i)}
      {#if r.kind === "stop"}
        <li class="stop {r.at}" class:first={i === first} class:last={i === last}>
          <span class="sp"><span class="node {r.star}"><Glyph name={r.star} size={r.at === "next" ? 24 : 16} label={r.type} /></span></span>
          <div class="main">
            {#if r.at === "next"}<span class="label">Next stop · {ROUTE.away} jumps away</span>{/if}
            <div class="nameline">
              <span class="name">{r.name}</span>
              <span class="ibtn copy" aria-label="Copy {r.name}"><Glyph name="copy" /></span>
            </div>
            {#if r.note}<span class="note">{r.note}</span>{/if}
            {#if r.facts}<span class="facts label">{#each r.facts as f, k (f)}{k ? " · " : ""}<span class:sec-low={f === "Low security"} class:sec-anarchy={f === "Anarchy"}>{f}</span>{/each}{#if r.state} · <span class="state">{r.state}</span>{/if}</span>{/if}
          </div>
        </li>
      {:else if r.kind === "jump"}
        <li class="jump {r.at}">
          <span class="sp"><span class="node {r.star}">{#if r.at === "here"}<Glyph name="here" size={20} label="You are here" />{:else}<Glyph name={r.star} label={r.type} />{/if}</span></span>
          <div class="main">
            {#if r.at === "here"}<span class="inl"><Glyph name={r.star} label={r.type} /></span>{/if}
            <span class="jn">{r.name}</span>
            {#if r.at === "here" && tips}
              <span class="tip below" role="tooltip"><b>You are here</b><br />Arietis Sector LD-H a11-1 · K star, scoopable · jump 2 of 3</span>
            {/if}
          </div>
        </li>
      {:else}
        <li class="leg {r.at}"><span class="sp"></span><div class="main"><span class="label"><span class="count">{r.jumps}</span> jumps</span></div></li>
      {/if}
    {/each}
  </ol>
</div>

<style>
  .side { padding: var(--sp-4) var(--sp-4) var(--sp-6) var(--side); display: flex; flex-direction: column; gap: var(--sp-3); }
  .head { display: flex; align-items: flex-start; gap: var(--sp-2); }
  .title { flex: 1; min-width: 0; }
  .to { color: var(--orange-mid); font-weight: 400; }
  .dest { border-bottom: 1px dashed var(--orange-mid); }
  .fold { flex: none; margin-top: 2px; }
  .chev { display: grid; transform: rotate(90deg); }
  .tools { display: flex; align-items: center; gap: var(--gap); }
  .dd { gap: var(--sp-2); padding: 0 var(--sp-2) 0 var(--sp-3); }
  .tools .ibtn:first-of-type { margin-left: var(--sp-2); }
  .sum { display: flex; align-items: center; justify-content: space-between; gap: var(--sp-2); margin: -4px 0 var(--sp-2); }
  .sum .act { margin-right: -8px; }

  /* The Route page's timeline, with a 24 px spine column so a name gets 200 px. */
  .tl { display: grid; grid-template-columns: 24px minmax(0, 1fr); column-gap: var(--sp-2); margin: 0 0 0 -4px; padding: 0; list-style: none; }
  .tl > li { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; position: relative; isolation: isolate; }
  .tl > li::after { content: ""; position: absolute; inset: 0 0 0 28px; z-index: -1; }
  .sp { position: relative; display: grid; place-items: center; }
  .sp::before { content: ""; position: absolute; top: 0; bottom: 0; left: calc(50% - 1px); border-left: 2px solid var(--accent); }
  .first .sp::before { top: 50%; }
  .last .sp::before { bottom: 50%; }
  .past .sp::before { border-left-color: var(--orange-dim); }
  .ahead .sp::before, .next .sp::before { border-left-style: dashed; }
  .jump.past .sp::before, .jump.here .sp::before { border-left-style: solid; }
  .node { position: relative; z-index: 1; display: grid; place-items: center; padding: 3px 0; background: var(--bg); color: var(--accent); }
  .main { display: flex; flex-direction: column; justify-content: center; min-width: 0; padding: var(--sp-2) 0 var(--sp-2) var(--sp-2); position: relative; }
  .nameline { display: flex; align-items: center; gap: 2px; min-width: 0; }
  .name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0; }
  .copy { flex: none; }
  .note { font-size: var(--fs-s); color: var(--orange-lit); font-weight: 600; }
  .facts { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .state { color: var(--text-hi); }

  .stop > .main { min-height: 44px; }
  .stop.past .node, .stop.past .name { color: var(--orange-dim); }
  .stop.next::after { background: var(--surface); }
  .stop.next > .main { min-height: 76px; }
  .stop.next .node, .stop.next .name { color: var(--orange-lit); }
  .stop.next .name { white-space: normal; font-size: var(--fs-l); font-weight: 600; line-height: 1.2; }
  .stop.next .ibtn { color: var(--accent); align-self: flex-start; margin-top: -3px; }

  .jump > .main { flex-direction: row; align-items: center; justify-content: flex-start; gap: var(--sp-2); min-height: 32px; padding-block: 0; }
  .jump .node { color: var(--orange-mid); }
  .jn { color: var(--accent); font-weight: 400; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0; }
  .jump.past .node, .jump.past .jn { color: var(--orange-dim); }
  .jump.here::after { background: var(--surface); }
  .jump.here > .main { min-height: 40px; }
  .jump.here .node { color: var(--text-hi); }
  .jump.here .jn { color: var(--text-hi); font-weight: 600; }
  .inl { color: var(--accent); display: grid; }

  .leg > .main { min-height: 32px; padding-block: 0; }
  .leg .count { color: var(--accent); }
  .leg.past .count, .leg.past .label { color: var(--orange-dim); }

  /* On the frame's Hull: a Nav-role caps label like the header's tabs, the plan in Name size,
     controls at Label size; raised rows step up to Hull Raised, and the spine's node masks match. */
  .frame { padding-top: var(--sp-3); gap: var(--sp-2); }
  .fhead { display: flex; align-items: center; justify-content: space-between; height: var(--row); margin-right: -4px; }
  .flabel { font-weight: 500; letter-spacing: var(--caps); text-transform: uppercase; color: var(--orange-mid); }
  .dline { font-weight: 500; color: var(--accent); }
  .frame .tools .dd { font-size: var(--fs-s); }
  .frame .sum { margin-bottom: var(--sp-3); }
  .frame .node { background: var(--surface); }
  .frame .stop.next::after, .frame .jump.here::after { background: var(--surface-hi); }

  .tip { position: absolute; z-index: 40; width: max-content; max-width: 250px; padding: var(--sp-2) var(--sp-3); background: var(--surface-hi); border: 1px solid var(--orange-mid); color: var(--accent); font-size: var(--fs-s); font-weight: 400; line-height: 1.4; white-space: normal; }
  .tip :global(b) { color: var(--text-hi); font-weight: 600; }
  .tip.below { top: calc(100% + 6px); left: 0; }
</style>
