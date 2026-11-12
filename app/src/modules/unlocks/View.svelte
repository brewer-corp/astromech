<script lang="ts">
  // Engineer unlocks as access-level cards. Columns follow who introduces whom; hovering a
  // card lights the engineers who have to introduce them first.
  import { tip } from "../../lib/ui/tip";
  import { failure } from "../../lib/ui/errors";
  import { columns, loadUnlocks, type Status, type UnlockNode, type UnlockTree } from "../../lib/unlocks";
  import { specialties } from "../../lib/engineerSpecialties";
  import Choice from "../../lib/ui/Choice.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import { hashParam, pageHash } from "../../lib/hashstate";
  import SystemChip from "../../lib/ui/SystemChip.svelte";

  let tree = $state<UnlockTree | null>(null);
  let err = $state<string | null>(null);
  // Ship or suit engineers is a choice on the page, kept in the link: #/unlocks?kind=suit.
  const readKind = () => (hashParam("kind") === "suit" ? "suit" : "ship");
  let kind = $state<"ship" | "suit">(readKind());
  $effect(() => {
    const onHash = () => (kind = readKind());
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });
  const KINDS = [
    { value: "ship", label: "Ship engineers" },
    { value: "suit", label: "Suit engineers" },
  ];
  let hover = $state<string | null>(null);

  function load() {
    err = null;
    loadUnlocks()
      .then((t) => (tree = t))
      .catch((e: unknown) => (err = failure("read the engineer list", e)));
  }
  $effect(() => load());

  const cols = $derived(tree ? columns(tree, kind) : []);
  const mine = $derived(tree?.nodes.filter((n) => n.kind === kind) ?? []);
  const preds = $derived(mine.find((n) => n.name === hover)?.introduced_by ?? []);

  function access(n: UnlockNode): { mark: Status; count: number; label: string } {
    if (n.status === "unlocked") {
      const rank = Math.max(1, Math.min(5, n.kind === "ship" ? (n.rank ?? 1) : 1));
      return { mark: n.status, count: rank, label: `Unlocked, rank ${rank}` };
    }
    if (n.status === "known") return { mark: n.status, count: 1, label: "Known · no access yet" };
    if (n.status === "invited") return { mark: n.status, count: 1, label: "Invited" };
    if (n.status === "barred") return { mark: n.status, count: 1, label: "Barred" };
    return { mark: n.status, count: 1, label: "Not introduced" };
  }
</script>

<section>
  <div class="query">
    <div class="bar">
      <Choice label="Engineers" options={KINDS} value={kind} onchange={(v) => (location.hash = pageHash("unlocks", { kind: v === "suit" ? "suit" : null }))} />
      {#if !err && !tree}
        <p class="sub">Reading the journal…</p>
      {:else if tree}
        <p class="sub">
          {mine.filter((n) => n.status === "unlocked").length} of {mine.length} unlocked · orange marks are your rank · a dim mark means no access yet · dimmer cards have not been introduced
        </p>
      {/if}
    </div>
    {#if err}
      <Failure text={err} onaction={load} />
    {/if}
  </div>
  {#if tree}
    <div class="out">
      <div class="tree" style:--cols={cols.length}>
        {#each cols as col, d (d)}
          <div class="col">
            {#each col as n (n.name)}
              {@const a = access(n)}
              {@const specs = specialties(n.name, n.kind)}
              <article
                class="eng {n.status}"
                class:next={n.next}
                class:pred={preds.includes(n.name)}
                class:hot={hover === n.name}
                aria-label={n.name}
                onmouseover={() => (hover = n.name)}
                onmouseenter={() => (hover = n.name)}
                onmouseleave={() => (hover = null)}
                onfocusin={() => (hover = n.name)}
                onfocusout={() => (hover = null)}
              >
                <div class="top">
                  <span class="name">{n.name}</span>
                  <span class="access" role="img" aria-label={a.label} {@attach tip({ title: a.label, stop: false })}>
                    {#if a.mark === "unlocked"}
                      {#each Array.from({ length: a.count }, (_, i) => i) as i (i)}
                        <span class="badge" aria-hidden="true"></span>
                      {/each}
                    {:else if a.mark === "invited"}
                      <span class="mail" aria-hidden="true"></span>
                    {:else if a.mark === "barred"}
                      <span class="barred" aria-hidden="true"></span>
                    {:else}
                      <span class="outline" aria-hidden="true"></span>
                    {/if}
                  </span>
                </div>
                {#if a.mark === "known"}<div class="gate">Known · no access yet</div>
                {:else if a.mark === "locked"}<div class="gate">Not introduced</div>
                {:else if a.mark === "invited"}<div class="gate">Invited</div>
                {:else if a.mark === "barred"}<div class="gate">Barred</div>
                {/if}
                {#if specs.length}
                  <div class="spec">{specs.join(" · ")}</div>
                {/if}
                <div class="where"><SystemChip name={n.system} />{n.permit ? " · permit" : ""}</div>
                {#if n.introduced_by.length}<div class="from">← {n.introduced_by.join(", ")}</div>{/if}
                {#if n.left}<p class="left">{n.left}</p>{/if}
              </article>
            {/each}
          </div>
        {/each}
      </div>
    </div>
  {/if}
</section>

<style>
  section { padding: 0 0 28px; }
  /* Ship or suit is a choice on the page, not a third tab row (DESIGN.md The Two Tab Rows Rule). */
  .query { display: flex; flex-direction: column; gap: var(--sp-3); padding: var(--sp-6) 0 0; }
  .bar { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-4); }
  .sub { color: var(--orange-mid); font-size: var(--fs-s); margin: 0; }
  .out { padding: var(--sp-6) 0 8px; }
  .tree { display: grid; grid-template-columns: repeat(var(--cols), minmax(16rem, 1fr)); gap: 8px 20px; align-items: start; }
  .col { display: flex; flex-direction: column; gap: 10px; min-width: 0; }
  /* Read-only cards on the hull (DESIGN.md Surfaces and cells), with no side border: state is
     brightness. An unlocked engineer's name is Orange Lit; one you can work on now (next) stays
     Orange; one not introduced or barred steps down to Orange Mid. */
  .eng { padding: 16px; display: flex; flex-direction: column; gap: 6px; background: var(--surface); outline: 1px solid transparent; outline-offset: -1px; }
  .eng.unlocked .name { color: var(--orange-lit); }
  /* Still readable one step down, since what is left to do is on the card. */
  .eng:is(.locked, .barred):not(.next) { color: var(--orange-mid); }
  .eng.hot { background: var(--row-hover); }
  /* The engineers who introduce the hovered one. */
  .eng.pred { outline-color: var(--accent); }
  .top { display: flex; justify-content: space-between; gap: 12px; align-items: start; }
  .name { font-size: var(--fs-l); font-weight: 500; }
  .access { display: inline-flex; align-items: center; gap: 3px; flex: none; }
  .gate { font-size: var(--fs-s); color: var(--orange-dim); }
  .eng.known .gate, .eng.invited .gate { color: var(--accent); }
  .access > :is(.badge, .outline, .barred, .mail) { width: 12px; height: 14px; flex: none; }
  .access > .badge { background: currentColor; color: var(--accent); -webkit-mask: url("/engineers/access.svg") center / contain no-repeat; mask: url("/engineers/access.svg") center / contain no-repeat; }
  .access > .outline { color: var(--orange-dim); -webkit-mask: url("/engineers/access.svg") center / contain no-repeat; mask: url("/engineers/access.svg") center / contain no-repeat; background: currentColor; }
  .access > .mail { color: var(--accent); -webkit-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3E%3Cpath fill='none' stroke='black' stroke-width='1.25' d='M2.5 4.5h11v7h-11zM2.5 4.5L8 9l5.5-4.5'/%3E%3C/svg%3E") center / contain no-repeat; mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3E%3Cpath fill='none' stroke='black' stroke-width='1.25' d='M2.5 4.5h11v7h-11zM2.5 4.5L8 9l5.5-4.5'/%3E%3C/svg%3E") center / contain no-repeat; background: currentColor; }
  .access > .barred { border: 1px solid currentColor; background: linear-gradient(to top right, transparent 46%, currentColor 46%, currentColor 54%, transparent 54%); }
  .spec { font-size: var(--fs-s); }
  .where, .from { font-size: var(--fs-s); color: var(--orange-mid); }
  .left { margin: 0; font-size: var(--fs-s); line-height: 1.45; }
  @container win (max-width: 900px) {
    .tree { grid-template-columns: 1fr; }
  }
</style>
