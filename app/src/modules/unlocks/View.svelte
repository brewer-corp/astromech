<script lang="ts">
  // Engineering > Engineers, on DESIGN.md's List with detail (tone5-engineers-rest-2560.png):
  // the search box on top with the summary line beside it, then the ship and suit engineers as
  // two trees side by side (Tree.svelte), and the selected engineer's side panel (Panel.svelte).
  // Where the trees don't fit side by side, the suit tree wraps below the ship tree; where one
  // tree doesn't fit beside the panel, the panel goes below both.
  // While the box holds a search the answer replaces the trees (Search.svelte, astromech-7did),
  // and clearing it brings them back. The search is kept in the link: #/unlocks?q=laser. The old
  // On foot link, #/unlocks?kind=suit, opens this page too.
  import { failure } from "../../lib/ui/errors";
  import { loadEngineers } from "../../lib/engineering";
  import { lightYears, loadUnlocks, searchEngineers, treeSummary, type EngineerSearch, type UnlockTree } from "../../lib/unlocks";
  import Failure from "../../lib/ui/Failure.svelte";
  import { hashParam, pageHash } from "../../lib/hashstate";
  import { loadOdyssey, type EngineerLine } from "../../lib/odyssey";
  import type { Engineers } from "../../lib/api/types/Engineers";
  import IconButton from "../../lib/ui/IconButton.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import ListDetail from "../../lib/ui/layout/ListDetail.svelte";
  import Panel from "./Panel.svelte";
  import Search from "./Search.svelte";
  import Tree from "./Tree.svelte";
  import Trees from "./Trees.svelte";

  let tree = $state<UnlockTree | null>(null);
  let err = $state<string | null>(null);
  let q = $state(hashParam("q") ?? "");
  $effect(() => {
    const onHash = () => (q = hashParam("q") ?? "");
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  function load() {
    err = null;
    loadUnlocks()
      .then((t) => (tree = t))
      .catch((e: unknown) => (err = failure("read the engineer list", e)));
  }
  $effect(() => load());

  // Where the commander is and where the ship engineers are; a failed read leaves distances out.
  let ships = $state<Engineers | null>(null);
  $effect(() => {
    loadEngineers()
      .then((v) => (ships = v))
      .catch(() => {});
  });
  /** The suit engineers' Odyssey lines (coordinates, referral task), by name. A failed read
   * leaves the panel without the referral task and the suit engineers without distances. */
  let suit = $state<Map<string, EngineerLine> | null>(null);
  $effect(() => {
    loadOdyssey()
      .then((v) => (suit = new Map(v.engineers.map((e) => [e.name, e]))))
      .catch(() => {});
  });

  let selected = $state<string | null>(null);
  const chosen = $derived(tree?.nodes.find((n) => n.name === selected) ?? null);
  const shipNodes = $derived(tree?.nodes.filter((n) => n.kind === "ship") ?? []);
  const suitNodes = $derived(tree?.nodes.filter((n) => n.kind === "suit") ?? []);
  const shipInfo = $derived(chosen?.kind === "ship" ? ships?.engineers.find((e) => e.name === chosen.name) : undefined);
  const suitLine = $derived(chosen?.kind === "suit" ? suit?.get(chosen.name) : undefined);
  const distance = $derived.by(() => {
    const here = ships?.here?.pos;
    const at = shipInfo?.coords ?? suitLine?.coords;
    return here && at ? lightYears(here, at) : null;
  });
  const introduces = $derived(chosen ? (tree?.nodes.filter((n) => n.introduced_by.includes(chosen.name)).map((n) => n.name) ?? []) : []);

  /** The search box's words, kept in the link without adding a history entry per key. */
  function typed(v: string) {
    q = v;
    history.replaceState(history.state, "", pageHash("unlocks", { q: v.trim() ? v : null }));
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
  /** The answer, only while it is for the words in the box. */
  const current = $derived(found && found.query.trim() === q.trim() ? found : null);
  /** The narrowest the list keeps one tree's columns beside the panel, in px at 100 percent:
   * the ship tree's three columns of nodes (--node) and their gaps (--sp-6). Two trees side by
   * side need twice that and the gap between them (--sp-12). */
  const ONE_TREE = 3 * 200 + 2 * 24;
</script>

<section>
  <div class="search" class:held={searching}>
    <Input
      class="box page-search"
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
    {#if searching}
      {#if current?.says}
        <span class="type-label">{current.says} · {current.hits.length} {current.hits.length === 1 ? "engineer" : "engineers"}{current.here ? ` · distances from ${current.here}` : ""}</span>
      {/if}
    {:else if tree}
      <span class="type-label">{treeSummary(tree)}</span>
    {:else if !err}
      <span class="type-label">Reading the journal…</span>
    {/if}
  </div>
  {#if searching}
    <div class="answer">
      {#if searchErr}
        <Failure text={searchErr} onaction={() => search(q.trim())} />
      {:else if current}
        <Search res={current} onword={(w) => typed(w)} />
      {:else}
        <!-- The answer to the words before is gone while the new one comes: its actions would act on the wrong search. -->
        <p class="type-label">Searching the engineers…</p>
      {/if}
    </div>
  {:else if err}
    <div class="answer"><Failure text={err} onaction={load} /></div>
  {:else if tree}
    <ListDetail panel={chosen ? panel : null} selected={selected} min={ONE_TREE}>
      <Trees both={2 * ONE_TREE + 48}>
        <Tree title="Ship engineers" nodes={shipNodes} {selected} onselect={(n) => (selected = selected === n ? null : n)} />
        <Tree title="Suit engineers" nodes={suitNodes} {selected} onselect={(n) => (selected = selected === n ? null : n)} />
      </Trees>
    </ListDetail>
  {/if}
</section>

{#snippet panel()}
  {#if chosen}
    <Panel node={chosen} {introduces} line={suitLine} {distance} colonia={shipInfo?.colonia ?? suitLine?.colonia ?? false} />
  {/if}
{/snippet}

<style>
  .search { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-6); padding: var(--sp-6) 0 0; }
  /* DESIGN.md Input: a page's search box keeps its outline while it holds the search the page answers. */
  .search.held :global(.box .cell) { outline: 1px solid var(--accent); }
  .answer { padding: var(--sp-8) 0 var(--sp-2); }
</style>
