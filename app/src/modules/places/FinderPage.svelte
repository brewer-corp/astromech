<script lang="ts">
  // One finder's page (DESIGN.md Places home, mockup Finder.svelte), on the List with detail
  // layout: a "Places" crumb back to the home page, the finder's title, its main input with its
  // name, then the search kit with the finder's result rows, whose selected row fills the side
  // panel. A finder has no tab row of its own (The Two Tab Rows Rule). A finder that asks
  // nothing (Sell my cargo) has no main input.
  import type { Snippet } from "svelte";
  import type { Finder } from "../../lib/places";
  import ListDetail from "../../lib/ui/layout/ListDetail.svelte";

  let { finder, input, children }: { finder: Finder; input?: Snippet; children: Snippet } = $props();
</script>

<ListDetail>
  <a class="crumb" href="#/places">Places</a>
  <div class="head">
    <h1>{finder.title}</h1>
    {#if input}
      <div class="main finder-main">
        <span class="label">{finder.input}</span>
        {@render input()}
      </div>
    {/if}
  </div>
  {@render children()}
</ListDetail>

<style>
  /* A link to somewhere else (DESIGN.md Buttons → Links), in Label size. */
  .crumb { font-size: var(--fs-s); color: var(--accent); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .crumb:hover { color: var(--orange-lit); }
  .head { display: flex; flex-wrap: wrap; align-items: flex-end; gap: var(--sp-4) var(--sp-8); margin: var(--sp-1) 0 var(--sp-4); }
  h1 { margin: 0; font-size: var(--fs-xl); font-weight: 600; line-height: 1.2; color: var(--text-hi); }
  .main { display: flex; align-items: center; gap: var(--sp-3); }
  .label { font-size: var(--fs-s); color: var(--orange-mid); }
</style>
