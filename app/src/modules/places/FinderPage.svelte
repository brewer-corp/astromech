<script lang="ts">
  // One finder's page (DESIGN.md Places home, mockup Finder.svelte): a "Places" crumb back to the
  // home page, the finder's title, its main input with its name, then the search kit with the
  // finder's result rows. A finder has no tab row of its own (The Two Tab Rows Rule).
  import type { Snippet } from "svelte";
  import type { Finder } from "../../lib/places";

  let { finder, input, children }: { finder: Finder; input: Snippet; children: Snippet } = $props();
</script>

<section class="finder">
  <a class="crumb" href="#/places">Places</a>
  <div class="head">
    <h1>{finder.title}</h1>
    <div class="main">
      <span class="label">{finder.input}</span>
      {@render input()}
    </div>
  </div>
  {@render children()}
</section>

<style>
  .finder { padding: var(--sp-6) 0 72px; min-width: 0; max-width: var(--list-w); }
  /* A link to somewhere else (DESIGN.md Buttons → Links), in Label size. */
  .crumb { font-size: var(--fs-s); color: var(--accent); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .crumb:hover { color: var(--orange-lit); }
  .head { display: flex; flex-wrap: wrap; align-items: flex-end; gap: var(--sp-4) var(--sp-8); margin: var(--sp-1) 0 var(--sp-4); }
  h1 { margin: 0; font-size: var(--fs-xl); font-weight: 600; line-height: 1.2; color: var(--text-hi); }
  .main { display: flex; align-items: center; gap: var(--sp-3); }
  /* A typed or dropped-down main input is 320 px, as Find or plot; a choice keeps its cells' width. */
  .main > :global(:is(.field, .dropdown)) { width: 320px; }
  .label { font-size: var(--fs-s); color: var(--orange-mid); }
</style>
