<script lang="ts" module>
  export type Strip = { next: string; of: string; flags?: string[]; warn?: string; dest: string; left: string };
</script>

<script lang="ts">
  // The window frame as DESIGN.md defines it: header (wordmark, top tabs, Find or plot, status),
  // the route strip on every page, at most one row of page tabs, then the page.
  import type { Snippet } from "svelte";
  import "./ds.css";
  import Copy from "./Copy.svelte";

  let { top, tabs, tab, strip, children }: { top: string; tabs: string[]; tab: string; strip: Strip; children: Snippet } = $props();
  const nav = ["Controls", "Engineering", "Galaxy", "Trade", "Exploration", "Powerplay", "Logbook", "Settings"];
</script>

<div class="ds frame">
  <header>
    <span class="mark">ASTROMECH</span>
    <nav class="top">
      {#each nav as n (n)}<span class:on={n === top}>{n}</span>{/each}
    </nav>
    <div class="cell find"><span class="ph">Find or plot</span><span class="label">Ctrl+K</span></div>
    <span class="label">Elite running</span>
    <a class="help" href="#/mockup/ds-places">Help</a>
  </header>
  <div class="strip">
    <span class="label">Next</span>
    <span class="sys lit">{strip.next}</span><Copy />
    <span class="mid">{strip.of}</span>
    {#each strip.flags ?? [] as f (f)}<span class="badge">{f}</span>{/each}
    {#if strip.warn}<span class="warn"><span class="bang">!</span>{strip.warn}</span>{/if}
    <span class="grow"></span>
    <span class="label">To</span><span class="dest">{strip.dest}</span>
    <span class="mid">{strip.left}</span>
  </div>
  {#if tabs.length}
    <nav class="page">
      {#each tabs as t (t)}<span class:on={t === tab}>{t}</span>{/each}
    </nav>
  {:else}<span></span>{/if}
  <main>{@render children()}</main>
</div>

<style>
  .frame { height: 100vh; display: grid; grid-template-rows: 64px var(--strip) 44px 1fr; background: var(--bg); }
  header { display: flex; align-items: center; gap: var(--sp-6); margin: 0 var(--gutter); border-bottom: 1px solid var(--accent); }
  .mark { font-weight: 600; font-size: var(--fs-l); letter-spacing: 0.12em; color: var(--text-hi); margin-right: var(--sp-2); }
  .top { display: flex; gap: var(--sp-1); flex: 1; }
  .top span { display: flex; align-items: center; height: var(--row); padding: 0 var(--sp-3); font-weight: 500; letter-spacing: var(--caps); text-transform: uppercase; color: var(--orange-mid); }
  .top span.on { background: var(--accent); color: var(--on-accent); }
  .find { width: 300px; justify-content: space-between; }
  .help { color: var(--accent); font-size: var(--fs-s); text-decoration: underline; text-decoration-color: var(--orange-mid); text-underline-offset: 3px; }
  .strip { display: flex; align-items: center; gap: var(--sp-3); padding: 0 var(--gutter); background: var(--surface); }
  .strip .sys { font-weight: 500; margin-right: -10px; }
  .strip .grow { flex: 1; }
  .strip .warn { font-size: var(--fs-s); color: var(--accent); }
  .strip .dest { font-size: var(--fs-s); }
  .page { display: flex; gap: var(--sp-6); align-items: stretch; margin: 0 var(--gutter); }
  .page span { display: flex; align-items: center; font-weight: 500; color: var(--orange-mid); border-bottom: 2px solid transparent; }
  .page span.on { color: var(--orange-lit); border-bottom-color: var(--accent); }
  main { min-height: 0; overflow: hidden; padding: var(--sp-6) var(--gutter) 0; }
</style>
