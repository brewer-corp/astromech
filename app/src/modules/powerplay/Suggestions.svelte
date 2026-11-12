<script lang="ts">
  // What earns my power merits in this system (astromech_powerplay activity rules): the power's
  // ethos and the controller's weak spots first. Each activity's how, and an unverified rule's
  // reason, are in its name's Tip; each tag explains itself in its own. Unverified rules get a
  // quiet "?". Under the list, one line dates the rules.
  import Badge from "../../lib/ui/Badge.svelte";
  import { tip } from "../../lib/ui/tip";
  import type { RulesVersion, Suggestion } from "../../lib/powerplay";

  let { items, rules }: { items: Suggestion[]; rules: RulesVersion } = $props();

  /** "16 Oct 2026". */
  const asOf = $derived(new Date(`${rules.fetched}T00:00:00Z`).toLocaleDateString("en-GB", { day: "numeric", month: "short", year: "numeric", timeZone: "UTC" }));
</script>

<ul>
  {#each items as s, i (`${s.activity}-${i}`)}
    <li>
      <span class="name" {@attach tip({ title: s.name, fact: s.how, note: s.uncertain ? `Not checked in game: ${s.uncertain}` : undefined })}>{s.name}</span>
      {#if s.weak_spot}<Badge {@attach tip("The controlling power is weak to it")}>Weak spot</Badge>{/if}
      {#if s.favoured}<Badge {@attach tip("Your power favours it: bonus merits")}>Ethos</Badge>{/if}
      {#if s.odyssey}<Badge>On foot</Badge>{/if}
      {#if s.uncertain}<span class="unverified" aria-hidden="true">?</span>{/if}
    </li>
  {/each}
</ul>
<p class="foot" {@attach tip("Frontier rebalances Powerplay often: check a surprising line in game")}>Community guides, as of {asOf} (rules v{rules.version})</p>

<style>
  ul { list-style: none; margin: 0; padding: 0; display: grid; gap: 10px; }
  .name { font-weight: 500; }
  li > :global(.badge) { margin-left: 6px; }
  .unverified { margin-left: 6px; color: var(--orange-mid); font-size: var(--fs-s); }
  .foot { margin: 14px 0 0; color: var(--orange-mid); font-size: var(--fs-s); line-height: 1.45; }
</style>
