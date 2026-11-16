<script lang="ts" module>
  /** open: the sidebar docked on the left (or the strip, where the window is too narrow);
   * over: as open, but at a narrow window the route is drawn opened over the page;
   * collapsed: the strip; none: no route followed. */
  export type RouteMode = "open" | "over" | "collapsed" | "none";
  // The tab tree the spike proposes: SHIPS splits from ENGINEERING, GALAXY becomes PLACES.
  export const NAV = ["Now", "Ships", "Engineering", "Places", "Trade", "Exploration", "Powerplay", "Logbook", "Controls", "Settings"];
  /** The sidebar docks only where a List with detail page still fits beside it: the Sell finder's
   * row (about 930 px at its narrowest) and the 320 px panel need about 1300 px, plus the sidebar's
   * 320 and two 40 px gutters. Below that the route is the strip, and opens over the page. 16:9
   * screens from 1920 x 1080 up are 1778 layout px wide, so they dock. */
  export const DOCK_AT = 1700;
  /** page: the first spike, the sidebar on the window like a page column. A: the sidebar on the
   * frame's Hull, flush left, with a frame heading. B: as A, joined to a Hull status bar in an L. */
  export type Skin = "page" | "A" | "B";
</script>

<script lang="ts">
  // Nav spike frame (spike/nav-rework): today's frame (header, page tabs, page, status bar) with
  // the route moved from the strip and the Route page into a left sidebar. Sizes follow App.svelte
  // and DESIGN.md's Frame, and the one root zoom of The Height Scale Rule.
  import type { Snippet } from "svelte";
  import "../glow/glow.css";
  import "./tone.css";
  import Glyph from "../../lib/ui/Glyph.svelte";
  import RouteSide from "./RouteSide.svelte";
  import Strip from "./Strip.svelte";

  let { top, tabs = [], tab = "", route = "open", skin = "page", tone = 0, children }: { top: string; tabs?: string[]; tab?: string; route?: RouteMode; skin?: Skin; tone?: number; children: Snippet } = $props();
  const framed = $derived(skin !== "page");

  let w = $state(window.innerWidth);
  let h = $state(window.innerHeight);
  const zoom = $derived(Math.max(1, Math.min(1.8, h / 1000, w / 1100)));
  const lw = $derived(w / zoom);
  const docked = $derived((route === "open" || route === "over") && lw >= DOCK_AT);
  const strip = $derived(route === "collapsed" || ((route === "open" || route === "over") && !docked));
  const over = $derived(route === "over" && !docked);

  // One header row needs the natural width of its parts; measured from a hidden copy so the
  // spike can report where the ten tabs stack (data-need on the frame, in layout px).
  let need = $state(0);
  let room = $state(0);
  const stacked = $derived(need > 0 && room > 0 && need > room);
</script>

<svelte:window bind:innerWidth={w} bind:innerHeight={h} />

{#snippet tabsRow()}{#each NAV as n (n)}<span class:on={n === top}>{n}</span>{/each}{/snippet}

<div class="glow frame skin-{skin} tone-{tone}" class:framed class:stacked style="zoom: {zoom}; width: {lw}px; height: {h / zoom}px; --side: {Math.max(16, Math.min(40, 0.03 * lw))}px; --hgap: {Math.max(16, Math.min(40, 0.025 * lw))}px" data-need={need} data-lw={Math.round(lw)}>
  <header bind:clientWidth={room}>
    <span class="mark">ASTROMECH</span>
    {#if !stacked}<nav class="top">{@render tabsRow()}</nav>{:else}<span class="grow"></span>{/if}
    <div class="cell find"><span class="ph">Find or plot</span><span class="key">Ctrl+K</span></div>
    <span class="help">Help</span>
    {#if stacked}<nav class="top second">{@render tabsRow()}</nav>{/if}
  </header>
  <div class="measure" aria-hidden="true" bind:clientWidth={need}>
    <span class="mark">ASTROMECH</span><nav class="top">{@render tabsRow()}</nav><div class="cell find"><span class="ph">Find or plot</span><span class="key">Ctrl+K</span></div><span class="help">Help</span>
  </div>

  {#if strip}<Strip open={over} />{/if}

  <div class="body">
    {#if docked}
      <aside class="route" aria-label="Route"><RouteSide frame={framed} /></aside>
    {:else if route === "none"}
      <!-- No route: nothing reserved but the gutter, which holds the way to plot one. -->
      <div class="rail">
        <span class="ibtn plot" aria-label="Plot a route"><Glyph name="add-stop" />
          <span class="tip right" role="tooltip"><b>No route.</b> Plot one in Find or plot (Ctrl+K)</span>
        </span>
      </div>
    {/if}
    <div class="col" class:railed={route === "none"}>
      {#if tabs.length}<nav class="page">{#each tabs as t (t)}<span class:on={t === tab}>{t}</span>{/each}</nav>{/if}
      <main>{@render children()}</main>
    </div>
    {#if over}
      {#if framed}<div class="scrim" aria-hidden="true"></div>{/if}
      <aside class="route over" aria-label="Route"><RouteSide overlay frame={framed} /></aside>
    {/if}
  </div>

  <footer>
    <span class="ship">Heavy Lifter</span><span class="mid">Anaconda</span>
    <span class="stat"><span class="mid">Jump</span><span>18.0<span class="unit">ly</span></span></span>
    <span class="stat"><span class="mid">Fuel</span><span>24.6<span class="sep">|</span>32<span class="unit">t</span></span></span>
    <span class="stat"><span class="mid">Cargo</span><span>120<span class="sep">|</span>256<span class="unit">t</span></span></span>
    <span class="state">Elite running · Voice ready</span>
  </footer>
</div>

<style>
  .frame { container: win / inline-size; display: flex; flex-direction: column; background: var(--bg); overflow: hidden; position: relative; }
  .frame > * { flex: none; }
  header { box-sizing: border-box; display: flex; align-items: center; gap: var(--hgap); height: 64px; margin: 0 var(--side); border-bottom: 1px solid var(--accent); }
  .measure { position: absolute; visibility: hidden; display: flex; align-items: center; gap: var(--hgap); width: max-content; white-space: nowrap; }
  .measure .top { flex: none; }
  .stacked header { height: 108px; flex-wrap: wrap; align-content: flex-start; row-gap: 0; }
  .stacked .mark { display: flex; align-items: center; height: 63px; margin-right: auto; }
  .mark { font-weight: 600; font-size: var(--fs-l); letter-spacing: 0.12em; color: var(--text-hi); }
  .top { display: flex; gap: var(--sp-1); flex: 1; white-space: nowrap; }
  .top.second { order: 1; flex-basis: 100%; height: 44px; align-items: center; }
  .top span { display: flex; align-items: center; height: var(--row); padding: 0 var(--sp-3); font-weight: 500; letter-spacing: var(--caps); text-transform: uppercase; color: var(--orange-mid); }
  .top span.on { background: var(--accent); color: var(--on-accent); }
  .grow { flex: 1; }
  .find { width: 320px; box-sizing: border-box; justify-content: space-between; flex: none; }
  .find .key { font-size: var(--fs-s); }
  .help { color: var(--accent); font-size: var(--fs-s); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }

  .body { flex: 1 1 0; min-height: 0; display: flex; position: relative; }
  /* The docked route: 320 px on Void with a Seam rule on its right, from the header to the status bar. */
  .route { flex: none; width: 320px; box-sizing: border-box; border-right: 1px solid var(--line); overflow: auto; }
  .route.over { position: absolute; z-index: 30; top: 0; bottom: 0; left: 0; background: var(--bg); border-right-color: var(--orange-mid); }
  .rail { flex: none; width: 48px; display: flex; justify-content: center; padding-top: var(--sp-6); border-right: 1px solid var(--line); }
  .plot { position: relative; color: var(--orange-mid); }
  .col { flex: 1; min-width: 0; display: flex; flex-direction: column; }
    .page { flex: none; display: flex; gap: var(--sp-6); height: 44px; margin: 0 var(--side); }
  .page span { display: flex; align-items: center; color: var(--orange-mid); border-bottom: 2px solid transparent; }
  .page span.on { color: var(--orange-lit); border-bottom-color: var(--accent); }
  main { flex: 1 1 0; min-height: 0; overflow: auto; container-type: inline-size; padding: var(--sp-6) var(--side); }

  footer { box-sizing: border-box; display: flex; align-items: center; gap: var(--sp-6); height: var(--status-bar); padding: 0 var(--side); border-top: 1px solid var(--line); white-space: nowrap; font-size: var(--fs-s); }
  footer .ship { margin-right: calc(var(--sp-3) - var(--sp-6)); }
  footer .stat { display: flex; align-items: baseline; gap: var(--sp-2); }
  footer .sep { color: var(--orange-mid); margin: 0 var(--sp-1); }
  footer .state { margin-left: auto; color: var(--orange-mid); }

  /* Variants A and B: the route is frame, not page. The header's orange rule runs the full width
     over it; the sidebar takes the strip's Hull, flush to the window's left edge, with its own
     padding inside and a Seam rule on its right; the page column keeps Void and its gutter. */
  .framed header { margin: 0; padding: 0 var(--side); }
  .framed .route { background: var(--surface); border-right-color: var(--line); }
  /* Opened over the page (1100 x 700): an overlay, so the page behind steps down to Dim
     brightness under a Void scrim, and the panel's edge is the 1 px orange separator line. */
  .framed .route.over { background: var(--surface); border-right: 1px solid var(--accent); }
  .scrim { position: absolute; inset: 0; z-index: 25; background: var(--bg); opacity: 0.72; }
  /* B: the status bar takes the same Hull and the sidebar runs into it with no rule, so the
     sidebar and the bar are one L of frame; the page is the inset, edged by one Seam L inside. */
  .skin-B footer { background: var(--surface); border-top: none; }
  .skin-B .col { border-bottom: 1px solid var(--line); }
  .skin-B .route { border-right-color: var(--line); }

  /* The Tip, drawn open (DESIGN.md Tip: Hull Raised, a 1 px Orange Mid border, a pointer). */
  .tip { position: absolute; z-index: 40; width: max-content; max-width: 420px; padding: var(--sp-2) var(--sp-3); background: var(--surface-hi); border: 1px solid var(--orange-mid); color: var(--accent); font-weight: 400; line-height: 1.4; white-space: normal; }
  .tip :global(b) { color: var(--text-hi); font-weight: 600; }
  .tip.right { left: calc(100% + 10px); top: -4px; }
</style>
