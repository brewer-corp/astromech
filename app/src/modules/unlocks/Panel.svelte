<script lang="ts">
  // DESIGN.md Side panel for the selected engineer: the name as the title, where they are and
  // how far, the panel's one main value in Orange Lit (the distance to an unlocked engineer, or
  // the step to take toward one who isn't), the steps to unlock in order, what they work on with
  // the best grade, who they introduce, and Set destination with Add stop.
  import type { EngineerLine } from "../../lib/odyssey";
  import { addedNote, addStop, failedNote, followedNote, followRoute, type FollowNote } from "../../lib/route";
  import type { UnlockNode } from "../../lib/unlocks";
  import Button from "../../lib/ui/Button.svelte";
  import FollowNoteLine from "../../lib/ui/FollowNote.svelte";
  import Hexes from "../../lib/ui/Hexes.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import { tip } from "../../lib/ui/tip";

  let {
    node,
    introduces,
    line,
    distance,
    colonia,
  }: {
    node: UnlockNode;
    /** The engineers naming this one in `introduced_by`. */
    introduces: string[];
    /** A suit engineer's Odyssey line (referral task); absent when that read failed. */
    line?: EngineerLine;
    /** Light years from the commander's system; null when either position is unknown. */
    distance: number | null;
    colonia: boolean;
  } = $props();

  /** The Tip on a "(?)" after a suit engineer's mod. */
  const UNSURE = "The wiki disagrees with itself on whether this engineer fits this mod";
  const ship = $derived(node.kind === "ship");
  const unlocked = $derived(node.status === "unlocked");
  const rank = $derived(Math.max(1, Math.min(5, node.rank ?? 1)));
  const ly = (d: number) => d.toLocaleString("en-GB", { maximumFractionDigits: d < 100 ? 1 : 0 });
  const status = $derived.by(() => {
    if (unlocked) return ship ? `Unlocked, rank ${rank} of 5` : "Unlocked";
    if (node.status === "barred") return "Barred";
    if (node.status === "invited") return "Invited: the workshop unlock is next";
    if (node.status === "known") return "Known: the invitation is next";
    return node.next ? "Not introduced: the introduction is next" : "Not introduced";
  });
  const and = (xs: string[]) => (xs.length < 2 ? (xs[0] ?? "") : `${xs.slice(0, -1).join(", ")} and ${xs.at(-1)}`);

  let note = $state<FollowNote | null>(null);
  async function go() {
    try {
      note = followedNote(await followRoute(node.system, "engineers", [{ system: node.system, label: node.name }]));
    } catch (e) {
      note = failedNote("Couldn't follow it", e, () => void go());
    }
  }
  async function add() {
    try {
      note = addedNote(await addStop({ system: node.system, label: node.name }, "engineers"));
    } catch (e) {
      note = failedNote("Couldn't add it", e, () => void add());
    }
  }
  // A note is about the engineer it was written for.
  $effect(() => {
    void node.name;
    note = null;
  });
</script>

<h2 class="type-title">{node.name}</h2>
<p class="where type-label">
  <SystemChip name={node.system} />
  {#if distance !== null && !unlocked}<span>· {ly(distance)} ly</span>{/if}
  {#if node.permit}<span>· permit</span>{/if}
  {#if colonia}<span>· Colonia</span>{/if}
</p>
{#if unlocked && distance !== null}
  <p class="pmain"><span class="type-value-xl main">{ly(distance)}</span><span class="type-unit">ly</span></p>
{/if}
<p class="status" class:barred={node.status === "barred"}>
  {#if unlocked}<Hexes n={ship ? rank : 1} of={ship ? 5 : 1} />{/if}<span>{status}</span>
</p>

{#if node.steps.length}
  <h3 class="section">To unlock</h3>
  <ol class="steps type-body">
    {#each node.steps as s, i (i)}<li class:first={i === 0} class:barred={node.status === "barred"}>{s}</li>{/each}
  </ol>
{/if}

{#if node.works_on.length}
  <h3 class="section">Works on</h3>
  <ul class="works" class:grades={ship} aria-label="Works on">
    {#each node.works_on as w (w.label)}
      <li>
        <span class="type-name wname">{w.label}{#if w.unsure}&nbsp;<span {@attach tip(UNSURE)}>(?)</span>{/if}</span>
        {#if ship}<Hexes n={unlocked ? Math.min(rank, w.grade) : 0} to={w.grade} tone="grade" label={unlocked ? `Grade ${Math.min(rank, w.grade)} today, reaches ${w.grade}` : `Reaches grade ${w.grade}`} />{/if}
      </li>
    {/each}
  </ul>
{/if}

{#if introduces.length || line?.referral_task}
  <h3 class="section">{ship ? "Introduces" : "Refers"}</h3>
  {#if introduces.length}<p class="type-body">{and(introduces)}</p>{/if}
  {#if !ship && line?.referral_task}<p class="type-body">Referral task: {line.referral_task}</p>{/if}
{/if}

<div class="acts">
  <Button kind="primary" onclick={() => void go()}>Set destination</Button>
  <Button onclick={() => void add()}>Add stop</Button>
</div>
{#if note}<FollowNoteLine bind:note />{/if}

<style>
  h2 { margin: 0 0 var(--sp-2); }
  .where { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-1) var(--sp-2); margin: 0; }
  .pmain { margin: var(--sp-4) 0 0; }
  .status { display: flex; align-items: center; gap: var(--sp-2); margin: var(--sp-4) 0 0; color: var(--prose); }
  .status.barred { color: var(--red-low); }
  .section { margin: var(--sp-8) 0 var(--sp-3); }
  .steps { margin: 0; padding: 0 0 0 var(--sp-6); display: flex; flex-direction: column; gap: var(--sp-2); color: var(--prose); line-height: 1.45; }
  .steps li::marker { color: var(--label-grey); }
  /* The step to take now is the panel's main value. */
  .steps li.first { color: var(--orange-lit); font-weight: 500; }
  .steps li.barred { color: var(--red-low); }
  /* As wide as its longest label, so each grade sits next to its module (DESIGN.md The Next To It Rule). */
  .works { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: max-content; gap: var(--sp-2) var(--sp-6); align-items: center; }
  .works.grades { grid-template-columns: max-content max-content; }
  .works li { display: contents; }
  .wname { color: var(--prose); }
  p.type-body { margin: 0 0 var(--sp-2); }
  .acts { display: flex; flex-wrap: wrap; gap: var(--sp-2); margin-top: var(--sp-8); }
</style>
