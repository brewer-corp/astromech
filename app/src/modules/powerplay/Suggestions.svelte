<script lang="ts">
  // What earns my power merits in this system (astromech_powerplay activity rules): the power's
  // ethos and the controller's weak spots first. Unverified rules get a quiet "?" and their
  // reason in small text; the tags are explained under the list (nothing is hover-only).
  import Badge from "../../lib/ui/Badge.svelte";
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
      {#if s.weak_spot}<Badge>Weak spot</Badge>{/if}
      {#if s.favoured}<Badge>Ethos</Badge>{/if}
      {#if s.odyssey}<Badge>On foot</Badge>{/if}
      {#if s.uncertain}<span class="unverified" aria-hidden="true">?</span>{/if}
      <p class="how">{s.how}</p>
      {#if s.uncertain}<p class="unsure">Not checked in game: {s.uncertain}</p>{/if}
    </li>
  {/each}
</ul>
<div class="foot">
  {#if ethos}<p><Badge>Ethos</Badge> your power favours it: bonus merits</p>{/if}
  {#if weak}<p><Badge>Weak spot</Badge> the controlling power is weak to it</p>{/if}
  {#if unverified}<p aria-hidden="true"><span class="key">?</span> not yet checked in game</p>{/if}
  <p>From community guides, as of {asOf} (rules v{rules.version}); Frontier rebalances Powerplay often.</p>
</div>

<style>
  ul { list-style: none; margin: 0; padding: 0; display: grid; gap: 10px; }
  .name { color: var(--text-hi); }
  li > :global(.badge) { margin-left: 6px; }
  .unverified, .key { color: var(--text-lo); font-family: var(--mono); font-size: var(--fs-s); }
  .unverified { margin-left: 6px; }
  .unsure { margin: 2px 0 0; color: var(--text-lo); font-size: var(--fs-s); font-style: italic; opacity: 0.8; }
  .how { margin: 2px 0 0; color: var(--text-lo); font-size: var(--fs-s); line-height: 1.4; }
  .foot { margin: 14px 0 0; color: var(--text-lo); font-size: var(--fs-s); line-height: 1.45; }
  .foot p { margin: 0 0 4px; }
  .key { display: inline-block; width: 1.2em; text-align: center; }
</style>
