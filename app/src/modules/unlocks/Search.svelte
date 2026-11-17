<script lang="ts">
  // The answer to the Engineers page's search (astromech-7did): "where do I engineer my lasers?".
  // The main answer leads, on Hull Raised: the best grade the commander can get today and the
  // engineer who gives it, nearest first, with Set destination and Add stop; with none unlocked,
  // the engineer nearest to unlocking and the step left. Under it every engineer who offers the
  // match, as DESIGN.md's result row in three groups (tone5-engineers-laser-2560.png): what they
  // reach (orange hexagons for what the commander can get, Orange Mid for what needs more rank or
  // an unlock, and a line such as "Pulse 4 · burst 3 · beam 3"), the engineer and the
  // commander's rank or the step left, and where (system with copy, distance), then the actions.
  // Suit engineers have no grades: they reach the modifications they fit.
  import { addedNote, addStop, failedNote, followedNote, followRoute, type FollowNote } from "../../lib/route";
  import { gradeLine, type EngineerSearch, type SearchHit } from "../../lib/unlocks";
  import Button from "../../lib/ui/Button.svelte";
  import FollowNoteLine from "../../lib/ui/FollowNote.svelte";
  import Hexes from "../../lib/ui/Hexes.svelte";
  import IconButton from "../../lib/ui/IconButton.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import { tip } from "../../lib/ui/tip";

  let { res, onword }: { res: EngineerSearch; /** A near match picked. */ onword: (w: string) => void } = $props();

  /** The Tip on a "(?)" after a suit engineer's mod, as on the tree's cards. */
  const UNSURE = "The wiki disagrees with itself on whether this engineer fits this mod";
  const main = $derived(res.main ? res.hits[0] : undefined);
  const ship = $derived(res.kind === "ship");
  const n = $derived(res.types.length);
  /** What the commander can't get from an unlocked engineer yet, and why. */
  function higher(h: SearchHit): string | null {
    if (h.status !== "unlocked" || h.today >= h.best) return null;
    return `grade ${h.best} at rank ${h.best}`;
  }
  /** "Grade 5 on pulse, burst and beam lasers" when one grade covers every type, else the grade line. */
  function mainLine(h: SearchHit): string {
    const g = h.grades[0]?.grade;
    return h.grades.length === n && h.grades.every((x) => x.grade === g) ? `Grade ${g} on ${res.says?.toLowerCase() ?? ""}` : gradeLine(h.grades, n);
  }
  const ly = (d: number) => d.toLocaleString("en-GB", { maximumFractionDigits: d < 100 ? 1 : 0 });

  let note = $state<FollowNote | null>(null);
  /** Which engineer's action wrote the note, so it sits beside its button. */
  let noteFor = $state<string | null>(null);
  async function add(h: SearchHit) {
    noteFor = h.name;
    try {
      note = addedNote(await addStop({ system: h.system, label: h.name }, "engineer search"));
    } catch (e) {
      note = failedNote("Couldn't add it", e, () => void add(h));
    }
  }
  async function go(h: SearchHit) {
    noteFor = h.name;
    try {
      note = followedNote(await followRoute(h.system, "engineer search", [{ system: h.system, label: h.name }]));
    } catch (e) {
      note = failedNote("Couldn't follow it", e, () => void go(h));
    }
  }
</script>

{#snippet who(h: SearchHit)}
  {#if h.status === "unlocked"}
    {#if h.kind === "ship"}
      {@const r = h.rank ?? 1}
      {@const more = higher(h)}
      <span class="rank"><Hexes n={r} label="Rank {r} of 5" /><span class="type-label">Rank {r}{more ? ` · ${more}` : ""}</span></span>
    {:else}
      <span class="type-label">Unlocked</span>
    {/if}
  {:else}
    <span class="type-label step {h.status}">{h.left}</span>
  {/if}
{/snippet}

<!-- A mod the wiki disagrees with itself on for this engineer keeps the tree's "(?)" and its Tip. -->
{#snippet modList(h: SearchHit)}{#each h.mods as m, j (m)}{j ? " · " : ""}{m}{h.unsure.includes(m) ? " " : ""}{#if h.unsure.includes(m)}<span {@attach tip(UNSURE)}>(?)</span>{/if}{/each}{/snippet}

{#snippet reach(h: SearchHit)}
  {#if ship}
    <Hexes n={h.today} offers={h.best} label={h.today ? `Grade ${h.today} today, reaches ${h.best}` : `Reaches grade ${h.best}`} />
    <span class="type-label gl">{gradeLine(h.grades, n)}</span>
  {:else}
    <span class="type-name mods">{@render modList(h)}</span>
  {/if}
{/snippet}

{#if res.kind === null}
  <div class="none">
    {#if res.suit_word}
      <p class="type-body">Nothing called "{res.query}" here. Suit engineers are searched by the modification you want: try
        {#each ["range", "magazine", "night vision"] as w, i (w)}{i ? (i === 2 ? " or " : ", ") : ""}<button class="word type-action" onclick={() => onword(w)}>{w}</button>{/each}.</p>
    {:else}
      <p class="type-body">No module type or modification called "{res.query}".{#if res.near.length}
          Did you mean
          {#each res.near as w, i (w)}{i ? ", " : " "}<button class="word type-action" onclick={() => onword(w)}>{w}</button>{/each}?{/if}</p>
    {/if}
  </div>
{:else if !res.hits.length}
  <p class="type-body none">No engineer works on {res.says?.toLowerCase()}.</p>
{:else}
  <!-- As wide as the list, so Set destination stays next to what it acts on (DESIGN.md The Next To It Rule). -->
  <div class="answer">
  {#if main}
    <div class="best" aria-label="Main answer">
      <span class="type-label">
        {#if res.main === "today"}{ship ? `Best you can get today: grade ${main.today}` : "Unlocked: fits it today"}{:else}None unlocked yet: nearest to unlock{/if}
      </span>
      <div class="bline">
        <span class="type-title">{main.name}</span>
        {@render who(main)}
      </div>
      {#if ship}
        <div class="bline">
          <Hexes n={main.today} offers={main.best} label={main.today ? `Grade ${main.today} today` : `Reaches grade ${main.best}`} />
          <span class="type-label gl">{mainLine(main)}</span>
        </div>
      {:else}
        <div class="bline"><span class="type-name mods">{@render modList(main)}</span></div>
      {/if}
      <div class="bline">
        <SystemChip name={main.system} />
        {#if main.distance_ly !== null}<span class="type-value-l main">{ly(main.distance_ly)}<span class="type-unit">ly</span></span>{/if}
        <span class="grow"></span>
        <Button kind="primary" onclick={() => void go(main)}>Set destination</Button>
        <IconButton icon="add-stop" label={`Add ${main.system} as a stop`} tip={`Add ${main.system} as the next stop`} onclick={() => void add(main)} />
      </div>
      {#if note && noteFor === main.name}<FollowNoteLine bind:note />{/if}
    </div>
  {/if}

  <div class="list" role="list" aria-label="Engineers for {res.says}">
    <div class="row heads type-label" aria-hidden="true"><span>Reaches</span><span>Engineer · your rank</span><span>Where</span><span></span></div>
    {#each res.hits as h (h.name)}
      <div class="row" role="listitem" aria-label={h.name}>
        <div class="grp">{@render reach(h)}</div>
        <div class="grp">
          <span class="type-name ename {h.status}" class:next={h.next}>{h.name}</span>
          {@render who(h)}
        </div>
        <div class="grp">
          <SystemChip name={h.system} />
          <span class="type-label">{h.distance_ly === null ? (h.permit ? "permit" : "") : `${ly(h.distance_ly)} ly${h.permit ? " · permit" : ""}`}</span>
        </div>
        <span class="acts">
          <IconButton icon="add-stop" label={`Add ${h.system} as a stop`} tip={`Add ${h.system} as the next stop`} onclick={() => void add(h)} />
          <IconButton icon="destination" label={`Set ${h.system} as the destination`} tip={`Set ${h.system} as the destination (replaces the route, which Resume brings back)`} onclick={() => void go(h)} />
        </span>
      </div>
      {#if note && noteFor === h.name && h !== main}<div class="rownote"><FollowNoteLine bind:note /></div>{/if}
    {/each}
  </div>
  </div>
{/if}

<style>
  .none { margin: 0; }
  .none p { margin: 0; }
  .word { all: unset; cursor: pointer; text-transform: none; letter-spacing: 0; text-decoration: underline; text-decoration-color: var(--orange-mid); text-underline-offset: 0.1875rem; }
  .word:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  /* The page's one leading block (DESIGN.md Surfaces: Hull Raised). */
  .best { display: flex; flex-direction: column; gap: var(--sp-2); padding: var(--sp-4) var(--sp-4) var(--sp-4) var(--sp-6); background: var(--surface-hi); margin-bottom: var(--sp-8); max-width: 100%; box-sizing: border-box; }
  .bline { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-3); min-width: 0; }
  .bline .type-title { margin-right: var(--sp-3); }
  .grow { flex: 1; }
  .rank { display: inline-flex; align-items: center; gap: var(--sp-2); }
  .gl { color: var(--prose); }
  .mods { color: var(--prose); }
  /* DESIGN.md's engineer states: you can work toward them, Orange Mid; barred, Red Low; not introduced, Grey Dim. */
  .step { color: var(--prose); }
  .step.barred { color: var(--red-low); }
  .ename:is(.known, .invited), .ename.locked.next { color: var(--orange-mid); }
  .ename.barred { color: var(--red-low); }
  .ename.locked:not(.next) { color: var(--grey-dim); }
  .answer { width: max-content; max-width: 100%; }
  .list { display: flex; flex-direction: column; max-width: 100%; }
  .row { display: grid; grid-template-columns: 15rem minmax(14rem, 22rem) minmax(10rem, 14rem) auto; align-items: center; column-gap: var(--sp-6); min-height: var(--row-result); padding: 0 var(--sp-3); }
  .row:not(.heads):hover { background: var(--row-hover); }
  .row.heads { min-height: 1.75rem; border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); }
  .grp { display: flex; flex-direction: column; justify-content: center; align-items: flex-start; gap: var(--sp-1); min-width: 0; }
  .grp > :global(*) { max-width: 100%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  /* The step left is what the commander does next, so it wraps rather than cut (astromech-vbz3). */
  .grp > .step { white-space: normal; text-wrap: pretty; }
  .acts { display: flex; gap: var(--sp-1); }
  .rownote { padding: 0 var(--sp-3) var(--sp-2); }
  @container win (max-width: 56.25rem) {
    .row { grid-template-columns: minmax(0, 1fr) minmax(0, 1fr) auto; }
    .row > .grp:nth-child(3) { grid-column: 1 / 3; }
    .row.heads > :nth-child(3) { display: none; }
  }
</style>
