<script lang="ts">
  // What earns my power merits in this system (astromech_powerplay activity rules): the power's
  // ethos and the controller's weak spots first. Unverified rules get a quiet "?" and their
  // reason in small text; the tags are explained under the list (nothing is hover-only).
  import type { RulesVersion, Suggestion } from "../../lib/powerplay";

  let { items, rules }: { items: Suggestion[]; rules: RulesVersion } = $props();

  const unverified = $derived(items.some((s) => s.uncertain));
  const ethos = $derived(items.some((s) => s.favoured));
  const weak = $derived(items.some((s) => s.weak_spot));
  /** "16 Oct 2026". */
  const asOf = $derived(new Date(`${rules.fetched}T00:00:00Z`).toLocaleDateString("en-GB", { day: "numeric", month: "short", year: "numeric", timeZone: "UTC" }));
</script>

<ul>
  {#each items as s, i (`${s.activity}-${i}`)}
    <li>
      <span class="name">{s.name}</span>
      {#if s.weak_spot}<span class="tag hot">weak spot</span>{/if}
      {#if s.favoured}<span class="tag">ethos</span>{/if}
      {#if s.odyssey}<span class="tag plain">on foot</span>{/if}
      {#if s.uncertain}<span class="unverified" aria-hidden="true">?</span>{/if}
      <p class="how">{s.how}</p>
      {#if s.uncertain}<p class="unsure">Not checked in game: {s.uncertain}</p>{/if}
    </li>
  {/each}
</ul>
<div class="foot">
  {#if ethos}<p><span class="tag inline">ethos</span> your power favours it: bonus merits</p>{/if}
  {#if weak}<p><span class="tag hot inline">weak spot</span> the controlling power is weak to it</p>{/if}
  {#if unverified}<p aria-hidden="true"><span class="key">?</span> not yet checked in game</p>{/if}
  <p>From community guides, as of {asOf} (rules v{rules.version}); Frontier rebalances Powerplay often.</p>
</div>

<style>
  ul { list-style: none; margin: 0; padding: 0; display: grid; gap: 10px; }
  .name { color: var(--text-hi); }
  .tag { margin-left: 6px; padding: 0 5px; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); color: var(--accent); border: 1px solid var(--accent-dim); }
  .tag.hot { color: var(--on-accent); background: var(--accent); border-color: var(--accent); }
  .tag.plain { color: var(--text-lo); border-color: var(--cell-opt); }
  .unverified, .key { color: var(--text-lo); font-family: var(--mono); font-size: var(--fs-s); }
  .tag.inline { margin-left: 0; }
  .unverified { margin-left: 6px; }
  .unsure { margin: 2px 0 0; color: var(--text-lo); font-size: var(--fs-s); font-style: italic; opacity: 0.8; }
  .how { margin: 2px 0 0; color: var(--text-lo); font-size: var(--fs-s); line-height: 1.4; }
  .foot { margin: 14px 0 0; color: var(--text-lo); font-size: var(--fs-s); line-height: 1.45; }
  .foot p { margin: 0 0 4px; }
  .key { display: inline-block; width: 1.2em; text-align: center; }
</style>
