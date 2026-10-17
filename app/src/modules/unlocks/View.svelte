<script lang="ts">
  // The engineer unlock guide as a tech tree: a column per introduction step, each engineer with
  // the commander's progress, who introduces them, and what is left. The ones that can be worked
  // on now stand out.
  import { failure } from "../../lib/ui/errors";
  import { columns, loadUnlocks, STATUS_LABEL, type UnlockTree } from "../../lib/unlocks";
  import SystemChip from "../../lib/ui/SystemChip.svelte";

  let tree = $state<UnlockTree | null>(null);
  let err = $state<string | null>(null);
  let kind = $state<"ship" | "suit">("ship");

  $effect(() => {
    loadUnlocks()
      .then((t) => (tree = t))
      .catch((e: unknown) => (err = failure("read the engineer list", e)));
  });

  const cols = $derived(tree ? columns(tree, kind) : []);
  const mine = $derived(tree?.nodes.filter((n) => n.kind === kind) ?? []);
  const HEADS = ["Common knowledge", "Introduced", "Introduced again", "Further"];
</script>

<section>
  <div class="acts" role="tablist" aria-label="Engineers">
    <button role="tab" aria-selected={kind === "ship"} class:on={kind === "ship"} onclick={() => (kind = "ship")}>Ship engineers</button>
    <button role="tab" aria-selected={kind === "suit"} class:on={kind === "suit"} onclick={() => (kind = "suit")}>Suit engineers</button>
  </div>
  {#if err}
    <p class="sub">{err}</p>
  {:else if !tree}
    <p class="sub">Reading the journal…</p>
  {:else}
    <p class="sub">
      {mine.filter((n) => n.status === "unlocked").length} of {mine.length} unlocked · {mine.filter((n) => n.next).length} you can work on now ·
      {kind === "ship" ? "an engineer introduces the next ones once your rank with them passes grade 3" : "each engineer refers the next after a task"}
    </p>
    <div class="tree" style:--cols={cols.length}>
      {#each cols as col, d (d)}
        <div class="col">
          <h4>{HEADS[Math.min(d, HEADS.length - 1)]}</h4>
          {#each col as n (n.name)}
            <article class="card {n.status}" class:next={n.next} aria-label={n.name}>
              <div class="top">
                <span class="name">{n.name}</span>
                <span class="st">{STATUS_LABEL[n.status]}{n.rank != null && n.status === "unlocked" && n.kind === "ship" ? ` · rank ${n.rank}` : ""}</span>
              </div>
              <div class="where"><SystemChip name={n.system} />{n.permit ? " · permit" : ""}</div>
              {#if n.introduced_by.length}<div class="from">← {n.introduced_by.join(", ")}</div>{/if}
              {#if n.left}<p class="left">{n.next ? "Next: " : ""}{n.left}</p>{/if}
            </article>
          {/each}
        </div>
      {/each}
    </div>
  {/if}
</section>

<style>
  section { padding: 8px 0 28px; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .acts { display: flex; gap: 26px; padding: 14px 0 6px; text-transform: uppercase; letter-spacing: var(--caps); }
  .acts button { color: var(--text-lo); }
  .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0 0 14px; }
  .tree { display: grid; grid-template-columns: repeat(var(--cols), minmax(15rem, 1fr)); gap: 18px; align-items: start; }
  .col { display: flex; flex-direction: column; gap: var(--gap); }
  h4 { margin: 0 0 4px; color: var(--text-lo); font-weight: 500; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .card { background: var(--cell); padding: 8px 10px; display: flex; flex-direction: column; gap: 4px; border-left: 2px solid transparent; }
  .card.next { border-left-color: var(--accent); }
  .card.unlocked { opacity: 0.75; }
  .card.locked:not(.next) .name { color: var(--text-lo); }
  .top { display: flex; justify-content: space-between; gap: 8px; align-items: baseline; }
  .name { color: var(--text-hi); }
  .st { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); white-space: nowrap; }
  .card.next .st { color: var(--accent); }
  .where, .from { font-size: var(--fs-s); color: var(--text-lo); }
  .left { margin: 0; font-size: var(--fs-s); line-height: 1.4; color: var(--text); }
</style>
