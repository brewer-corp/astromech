<script lang="ts">
  // A single-hop or loop trade route from here. Where to sell or buy one commodity, and where to
  // sell the hold, are Places finders (decision astromech-bi94); this page links into them
  // instead of listing them twice (decision astromech-g8d5, item 5).
  import { marketRoute, placesRoute } from "../../lib/places";
  import Route from "./Route.svelte";

  /** Between the links (a literal space at a tag edge would be trimmed). */
  const SEP = " · ";
  const links = [
    { label: "Sell", what: "a commodity", route: marketRoute({ side: "sell", symbol: "painite", anywhere: false }) },
    { label: "Buy", what: "a commodity", route: marketRoute({ side: "buy", symbol: "painite", anywhere: false }) },
    { label: "Sell my cargo", what: "the whole hold", route: placesRoute("sell-cargo") },
  ];
</script>

<section>
  <div class="query">
    <p class="moved">
      Where to sell or buy is in Places:
      {#each links as l, i (l.label)}{#if i}<span aria-hidden="true">{SEP}</span>{/if}<a href="#/{l.route}" title="{l.label} {l.what}">{l.label}</a>{/each}
    </p>
    <p class="sub">The most profitable hop from a station, or a loop that brings cargo back.</p>
  </div>
  <Route />
</section>

<style>
  section { padding: 0; min-width: 0; min-height: 0; display: flex; flex-direction: column; height: 100%; }
  .query { display: flex; flex-direction: column; gap: 12px; padding: 18px 20px 0; background: var(--bg-list); }
  .moved { margin: 0; font-size: var(--fs-s); color: var(--orange-mid); }
  /* Links to somewhere else (DESIGN.md Buttons → Links). */
  .moved a { color: var(--accent); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .moved a:hover { color: var(--orange-lit); }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0; }
</style>
