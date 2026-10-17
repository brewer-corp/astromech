<script lang="ts">
  // What earns my power merits in this system (astromech_powerplay activity rules): the power's
  // ethos and the controller's weak spots first. Unverified rules get a quiet "?".
  import type { RulesVersion, Suggestion } from "../../lib/powerplay";

  let { items, rules }: { items: Suggestion[]; rules: RulesVersion } = $props();

  const unverified = $derived(items.some((s) => s.uncertain));
  /** "16 Oct 2026". */
  const asOf = $derived(new Date(`${rules.fetched}T00:00:00Z`).toLocaleDateString("en-GB", { day: "numeric", month: "short", year: "numeric", timeZone: "UTC" }));
</script>

<ul>
  {#each items as s, i (`${s.activity}-${i}`)}
    <li>
      <span class="name">{s.name}</span>
      {#if s.weak_spot}<span class="tag hot" title="The controlling power is weak to this">weak spot</span>{/if}
      {#if s.favoured}<span class="tag" title="Your power's ethos favours this: bonus merits">ethos</span>{/if}
      {#if s.odyssey}<span class="tag plain">on foot</span>{/if}
      {#if s.uncertain}<span class="unverified" title="Not checked in game: {s.uncertain}"><span aria-hidden="true">?</span><span class="sr-only">Not checked in game: {s.uncertain}</span></span>{/if}
      <p class="how">{s.how}</p>
    </li>
  {/each}
</ul>
<p class="foot">
  From community guides, as of {asOf} (rules v{rules.version}); Frontier rebalances Powerplay often.
  {#if unverified}<span class="key" aria-hidden="true">?</span><span aria-hidden="true"> not yet checked in game.</span>{/if}
</p>

<style>
  ul { list-style: none; margin: 0; padding: 0; display: grid; gap: 10px; }
  .name { color: var(--text-hi); }
  .tag { margin-left: 6px; padding: 0 5px; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); color: var(--accent); border: 1px solid var(--accent-dim); }
  .tag.hot { color: var(--on-accent); background: var(--accent); border-color: var(--accent); }
  .tag.plain { color: var(--text-lo); border-color: var(--cell-opt); }
  .unverified, .key { color: var(--text-lo); font-family: var(--mono); font-size: var(--fs-s); }
  .unverified { margin-left: 6px; border-bottom: 1px dotted var(--text-lo); cursor: help; }
  .sr-only { position: absolute; width: 1px; height: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }
  .how { margin: 2px 0 0; color: var(--text-lo); font-size: var(--fs-s); line-height: 1.4; }
  .foot { margin: 12px 0 0; color: var(--text-lo); font-size: var(--fs-s); line-height: 1.45; }
</style>
