<script lang="ts" module>
  export type Strip = { next: string; of: string; flags?: string[]; dest: string; left: string };
  export type Status = { ship: string; type: string; jump: string; fuel: string; fuelLow?: boolean; cargo: string };
  export const ship: Status = { ship: "Long Way Round", type: "Krait Mk II", jump: "27.8", fuel: "3.9 / 32", fuelLow: true, cargo: "128 / 144" };
</script>

<script lang="ts">
  // The Glow-up window frame: header (wordmark, top tabs with NOW first, Find or plot, Help), the
  // route strip, at most one row of page tabs, the page, and a 32 px status bar at the foot.
  // The whole frame scales with the window height through one CSS zoom: 1000 px tall is 1.0,
  // capped at 1.8; it never goes below 1.0 (text stays at least 12 px) and never so far that the
  // layout is narrower than the 1100 px minimum window.
  import type { Snippet } from "svelte";
  import "./glow.css";
  import Copy from "./Copy.svelte";
  import Icon from "./Icon.svelte";

  let { top, tabs = [], tab = "", strip, status = ship, children }: { top: string; tabs?: string[]; tab?: string; strip: Strip; status?: Status; children: Snippet } =
    $props();
  const nav = ["Now", "Controls", "Engineering", "Galaxy", "Trade", "Exploration", "Powerplay", "Logbook", "Settings"];

  let w = $state(window.innerWidth);
  let h = $state(window.innerHeight);
  const zoom = $derived(Math.max(1, Math.min(1.8, h / 1000, w / 1100)));
  // The header fits on one row from about 1640 px of layout width; below that the tabs take a
  // second 44 px row under the wordmark, as DESIGN.md's Frame already does.
  const stacked = $derived(w / zoom < 1640);
</script>

<svelte:window bind:innerWidth={w} bind:innerHeight={h} />

<div class="glow frame" class:stacked style="zoom: {zoom}; width: {w / zoom}px; height: {h / zoom}px">
  <header>
    <span class="mark">ASTROMECH</span>
    {#if !stacked}
      <nav class="top">{#each nav as n (n)}<span class:on={n === top}>{n}</span>{/each}</nav>
    {:else}<span class="grow"></span>{/if}
    <div class="cell find"><span class="ph">Find or plot</span><span class="key">Ctrl+K</span></div>
    <a class="help" href="#/mockup/glow-now">Help</a>
    {#if stacked}
      <nav class="top second">{#each nav as n (n)}<span class:on={n === top}>{n}</span>{/each}</nav>
    {/if}
  </header>
  <div class="strip">
    <span class="label">Next</span>
    <span class="sys">{strip.next}</span><Copy />
    <span class="of">{strip.of}</span>
    {#each strip.flags ?? [] as f (f)}<span class="badge">{f}</span>{/each}
    <span class="grow"></span>
    <span class="label">To</span><span class="dest">{strip.dest}</span><Copy />
    <span class="of">{strip.left}</span>
  </div>
  {#if tabs.length}
    <nav class="page">{#each tabs as t (t)}<span class:on={t === tab}>{t}</span>{/each}</nav>
  {/if}
  <main>{@render children()}</main>
  <footer>
    <span class="item"><Icon name="ship" /><span class="name">{status.ship}</span><span class="label">{status.type}</span></span>
    <span class="item"><Icon name="jump" /><span>{status.jump}<span class="unit">ly</span></span></span>
    <span class="item" class:low={status.fuelLow}><Icon name="fuel" />{#if status.fuelLow}<span class="bang">!</span>{/if}<span>{status.fuel}<span class="unit">t fuel</span></span></span>
    <span class="item"><Icon name="cargo" /><span>{status.cargo}<span class="unit">t cargo</span></span></span>
    <span class="grow"></span>
    <span class="label">Elite running · Voice ready</span>
  </footer>
</div>

<style>
  .frame { display: flex; flex-direction: column; background: var(--bg); overflow: hidden; }
  header { flex: none; display: flex; align-items: center; gap: var(--sp-6); height: 64px; margin: 0 var(--gutter); border-bottom: 1px solid var(--accent); }
  .stacked header { flex-wrap: wrap; height: 108px; row-gap: 0; }
  .stacked .mark { height: 64px; display: flex; align-items: center; }
  .mark { font-weight: 600; font-size: var(--fs-l); letter-spacing: 0.12em; color: var(--text-hi); margin-right: var(--sp-2); }
  .top { display: flex; gap: var(--sp-1); flex: 1; }
  .top.second { flex-basis: 100%; height: 44px; align-items: flex-start; margin-left: calc(-1 * var(--sp-3)); }
  .top span { display: flex; align-items: center; height: var(--row); padding: 0 var(--sp-3); font-weight: 500; letter-spacing: var(--caps); text-transform: uppercase; color: var(--orange-mid); }
  .top span.on { background: var(--accent); color: var(--on-accent); }
  .grow { flex: 1; }
  .find { width: 320px; justify-content: space-between; }
  .find .key { font-size: var(--fs-s); }
  .help { color: var(--accent); font-size: var(--fs-s); text-decoration: underline; text-decoration-color: var(--orange-mid); text-underline-offset: 3px; }

  .strip { flex: none; display: flex; align-items: center; gap: var(--sp-3); height: var(--strip); padding: 0 var(--gutter); background: var(--surface); white-space: nowrap; }
  .strip .sys { color: var(--orange-lit); font-weight: 600; }
  .strip .dest { color: var(--accent); }
  .strip .of { color: var(--orange-mid); }

  .page { flex: none; display: flex; gap: var(--sp-6); height: 44px; margin: 0 var(--gutter); }
  .page span { display: flex; align-items: center; color: var(--orange-mid); border-bottom: 2px solid transparent; }
  .page span.on { color: var(--orange-lit); border-bottom-color: var(--accent); }

  main { flex: 1; min-height: 0; overflow: auto; container-type: inline-size; padding: var(--sp-6) var(--gutter) var(--sp-6); }

  footer { flex: none; display: flex; align-items: center; gap: var(--sp-8); height: 32px; padding: 0 var(--gutter); border-top: 1px solid var(--line); white-space: nowrap; font-size: var(--fs-s); }
  footer .item { display: inline-flex; align-items: center; gap: var(--sp-2); color: var(--accent); }
  footer .item :global(.icon) { color: var(--orange-mid); }
  footer .name { color: var(--accent); }
  footer .bang { margin-right: -4px; }
</style>
