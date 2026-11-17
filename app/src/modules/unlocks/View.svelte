<script lang="ts">
  // Engineer unlocks as access-level cards. Columns follow who introduces whom; hovering a
  // card lights the engineers who have to introduce them first. A suit engineer's card lists
  // every mod they fit, whether they are in Colonia and what they ask to refer the next, from the Odyssey data (moved from the On foot page,
  // astromech-5a01); without it the card shows their specialties, as a ship engineer's does.
  // A search box at the top answers "where do I engineer my lasers?" (Search.svelte,
  // astromech-7did); while it holds a search the answer replaces the tree, and clearing it
  // brings the tree back. The search is kept in the link: #/unlocks?q=laser.
  import { tip } from "../../lib/ui/tip";
  import { failure } from "../../lib/ui/errors";
  import { columns, loadUnlocks, searchEngineers, type EngineerSearch, type Status, type UnlockNode, type UnlockTree } from "../../lib/unlocks";
  import { specialties } from "../../lib/engineerSpecialties";
  import Choice from "../../lib/ui/Choice.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import { hashParam, pageHash } from "../../lib/hashstate";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import { loadOdyssey, type EngineerLine } from "../../lib/odyssey";
  import Hexes from "../../lib/ui/Hexes.svelte";
  import IconButton from "../../lib/ui/IconButton.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import Search from "./Search.svelte";

  let tree = $state<UnlockTree | null>(null);
  let err = $state<string | null>(null);
  // Ship or suit engineers is a choice on the page, kept in the link: #/unlocks?kind=suit.
  const readKind = () => (hashParam("kind") === "suit" ? "suit" : "ship");
  let kind = $state<"ship" | "suit">(readKind());
  let q = $state(hashParam("q") ?? "");
  $effect(() => {
    const onHash = () => {
      kind = readKind();
      q = hashParam("q") ?? "";
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });
  const KINDS = [
    { value: "ship", label: "Ship engineers" },
    { value: "suit", label: "Suit engineers" },
  ];
  let hover = $state<string | null>(null);
  /** The suit engineers' mods and referrals, by name; read once suit engineers are shown. */
  let suit = $state<Map<string, EngineerLine> | null>(null);
  $effect(() => {
    if (kind !== "suit" || suit) return;
    // A failed read leaves the cards as they are: the tree does not depend on it.
    loadOdyssey()
      .then((v) => (suit = new Map(v.engineers.map((e) => [e.name, e]))))
      .catch(() => {});
  });
  /** The Tip on a "(?)" after an engineer's mod. */
  const UNSURE = "The wiki disagrees with itself on whether this engineer fits this mod";

  function load() {
    err = null;
    loadUnlocks()
      .then((t) => (tree = t))
      .catch((e: unknown) => (err = failure("read the engineer list", e)));
  }
  $effect(() => load());

  /** The search box's words, kept in the link without adding a history entry per key. */
  function typed(v: string) {
    q = v;
    const h = pageHash("unlocks", { kind: kind === "suit" ? "suit" : null, q: v.trim() ? v : null });
    history.replaceState(history.state, "", h);
  }
  let found = $state<EngineerSearch | null>(null);
  let searchErr = $state<string | null>(null);
  let asked = 0;
  function search(words: string) {
    const n = ++asked;
    searchErr = null;
    searchEngineers(words)
      .then((r) => {
        if (n === asked) found = r;
      })
      .catch((e: unknown) => {
        if (n === asked) searchErr = failure("search the engineers", e);
      });
  }
  $effect(() => {
    const words = q.trim();
    if (!words) {
      asked++;
      found = null;
      searchErr = null;
      return;
    }
    search(words);
  });
  const searching = $derived(q.trim() !== "");

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
  <div class="search" class:held={searching}>
    <Input
      class="box"
      aria-label="Engineers for a module"
      placeholder="Engineers for a module: laser, FSD, shields…"
      value={q}
      oninput={(e) => typed(e.currentTarget.value)}
      onkeydown={(e) => {
        if (e.key === "Escape" && q) typed("");
      }}
    >
      {#snippet end()}{#if q}<IconButton icon="clear" label="Clear the search" onclick={() => typed("")} />{/if}{/snippet}
    </Input>
    {#if searching && found?.says && found.query.trim() === q.trim()}
      <span class="type-label">{found.says} · {found.hits.length} {found.hits.length === 1 ? "engineer" : "engineers"}{found.here ? ` · distances from ${found.here}` : ""}</span>
    {/if}
  </div>
  {#if searching}
    <div class="answer">
      {#if searchErr}
        <Failure text={searchErr} onaction={() => search(q.trim())} />
      {:else if found}
        <Search res={found} onword={(w) => typed(w)} />
      {/if}
    </div>
  {:else}
  <div class="query">
    <div class="bar">
      <Choice label="Engineers" options={KINDS} value={kind} onchange={(v) => (location.hash = pageHash("unlocks", { kind: v === "suit" ? "suit" : null }))} />
      {#if !err && !tree}
        <p class="sub">Reading the journal…</p>
      {:else if tree}
        <p class="sub">
          {mine.filter((n) => n.status === "unlocked").length} of {mine.length} unlocked · white marks are your rank · a dim mark means no access yet · dimmer cards have not been introduced
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
              {@const odd = n.kind === "suit" ? suit?.get(n.name) : undefined}
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
                      <!-- A suit engineer has no ranks: one lit hexagon says unlocked. -->
                      <Hexes n={a.count} of={n.kind === "ship" ? 5 : 1} />
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
                {#if odd?.mods.length}
                  <div class="spec">{#each odd.mods as m, j (j)}{j ? " · " : ""}{m.name}{m.unconfirmed ? " " : ""}{#if m.unconfirmed}<span {@attach tip(UNSURE)}>(?)</span>{/if}{/each}</div>
                {:else if specs.length}
                  <div class="spec">{specs.join(" · ")}</div>
                {/if}
                <div class="where"><SystemChip name={n.system} />{n.permit ? " · permit" : ""}{odd?.colonia ? " · Colonia" : ""}</div>
                {#if n.introduced_by.length}<div class="from">← {n.introduced_by.join(", ")}</div>{/if}
                <!-- What this engineer asks before referring the next, shown before they are unlocked so
                     the items can be collected ahead (the tree says only "Unlock <name> first" there). -->
                {#if odd?.refers.length && odd.referral_task}<div class="refers">Refers {odd.refers.join(", ")}: {odd.referral_task}</div>{/if}
                {#if n.left}<p class="left">{n.left}</p>{/if}
              </article>
            {/each}
          </div>
        {/each}
      </div>
    </div>
  {/if}
  {/if}
</section>

<style>
  section { padding: 0 0 1.75rem; }
  .search { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-6); padding: var(--sp-6) 0 0; }
  .search :global(.box) { width: 27.5rem; max-width: 100%; }
  /* DESIGN.md Input: a page's search box keeps its outline while it holds the search the page answers. */
  .search.held :global(.box .cell) { outline: 1px solid var(--accent); }
  .answer { padding: var(--sp-8) 0 var(--sp-2); }
  /* Ship or suit is a choice on the page, not a third tab row (DESIGN.md The Two Tab Rows Rule). */
  .query { display: flex; flex-direction: column; gap: var(--sp-3); padding: var(--sp-6) 0 0; }
  .bar { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-4); }
  .sub { color: var(--label-grey); font-size: var(--fs-s); margin: 0; }
  .out { padding: var(--sp-6) 0 var(--sp-2); }
  .tree { display: grid; grid-template-columns: repeat(var(--cols), minmax(15rem, 1fr)); gap: var(--sp-2) 1.25rem; align-items: start; }
  .col { display: flex; flex-direction: column; gap: 0.625rem; min-width: 0; }
  /* Read-only cards in a Seam outline (DESIGN.md Surfaces and cells: Hull is the frame's), with no
     side border. State is DESIGN.md's engineer colours: unlocked, the name Options Grey and the
     rank marks White; known or invited, the name Orange Mid (you can work toward them) and the step
     to unlock Prose Grey; barred, the name and reason Red Low; not introduced, the name and its
     state Grey Dim, unless you can work on them now. What is left to do stays readable: Grey Dim
     never carries text needed to act. */
  .eng { padding: var(--sp-4); display: flex; flex-direction: column; gap: 0.375rem; outline: 1px solid var(--line); outline-offset: -1px; }
  .eng .name { color: var(--options-grey); }
  .eng:is(.known, .invited) .name { color: var(--orange-mid); }
  .eng.barred :is(.name, .gate) { color: var(--red-low); }
  .eng.locked:not(.next) :is(.name, .gate) { color: var(--grey-dim); }
  .eng.hot { background: var(--row-hover); }
  /* The engineers who introduce the hovered one. */
  .eng.pred { outline-color: var(--accent); }
  .top { display: flex; justify-content: space-between; gap: var(--sp-3); align-items: start; }
  .name { font-size: var(--fs-l); font-weight: 500; }
  .access { display: inline-flex; align-items: center; gap: 0.1875rem; flex: none; }
  .gate { font-size: var(--fs-s); color: var(--prose); }
  .access > :is(.outline, .barred, .mail) { width: 0.75rem; height: 0.875rem; flex: none; }
  .access > .outline { color: var(--mark-off); -webkit-mask: url("/engineers/access.svg") center / contain no-repeat; mask: url("/engineers/access.svg") center / contain no-repeat; background: currentColor; }
  .access > .mail { color: var(--accent); -webkit-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3E%3Cpath fill='none' stroke='black' stroke-width='1.25' d='M2.5 4.5h11v7h-11zM2.5 4.5L8 9l5.5-4.5'/%3E%3C/svg%3E") center / contain no-repeat; mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3E%3Cpath fill='none' stroke='black' stroke-width='1.25' d='M2.5 4.5h11v7h-11zM2.5 4.5L8 9l5.5-4.5'/%3E%3C/svg%3E") center / contain no-repeat; background: currentColor; }
  .access > .barred { border: 1px solid currentColor; background: linear-gradient(to top right, transparent 46%, currentColor 46%, currentColor 54%, transparent 54%); }
  .spec { font-size: var(--fs-s); }
  .where, .from { font-size: var(--fs-s); color: var(--label-grey); }
  .refers { font-size: var(--fs-s); color: var(--prose); }
  .left { margin: 0; font-size: var(--fs-s); line-height: 1.45; }
  @container win (max-width: 56.25rem) {
    .tree { grid-template-columns: 1fr; }
  }
</style>
