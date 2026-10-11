<script lang="ts">
  // Journal-fed trader grids: the in-game inventory is a flat list with no grades.
  import { loadMaterials, type Cell, type Inventory } from "../../lib/materials";

  let inv = $state<Inventory | null>(null);
  let kindId = $state("raw");
  let selId = $state<string | null>(null);

  const kind = $derived(inv?.kinds.find((k) => k.id === kindId) ?? inv?.kinds[0]);
  const tabs = $derived([
    ...(inv?.kinds.map((k) => ({ id: k.id, title: k.title })) ?? []),
    ...(inv && inv.other.length ? [{ id: "other", title: "Other" }] : []),
  ]);
  const sel = $derived(
    kindId === "other"
      ? inv?.other.find((c) => c.id === selId)
      : kind?.groups.flatMap((g) => g.cells).find((c) => c.id === selId),
  );

  $effect(() => {
    void loadMaterials().then((v) => (inv = v));
    const t = setInterval(() => void loadMaterials().then((v) => (inv = v)), 2000);
    return () => clearInterval(t);
  });

  function fill(c: Cell) {
    const p = c.capacity > 0 ? Math.min(1, c.count / c.capacity) : 0;
    return `linear-gradient(90deg, var(--cell) ${p * 100}%, var(--cell-empty) ${p * 100}%)`;
  }

  function pick(id: string) {
    selId = id;
  }
</script>

<section>
  <div class="acts">
    {#each tabs as k (k.id)}
      <button class:on={kindId === k.id} onclick={() => { kindId = k.id; selId = null; }}>{k.title}</button>
    {/each}
  </div>
  <p class="sub">
    {#if inv}
      {kindId === "other" ? "Guardian, Thargoid and anything the journal named that is not on a trader." : (kind?.trader ?? "")}
      {#if kindId !== "other" && kind}
        · {kind.held.toLocaleString()} / {kind.capacity.toLocaleString()}
      {/if}
      · {inv.source}
    {:else}
      Reading the journal…
    {/if}
  </p>

  {#if kindId === "other" && inv}
    <div class="other">
      {#each inv.other as c (c.id)}
        <button class="cell" class:sel={selId === c.id} style:background={fill(c)} onclick={() => pick(c.id)}>
          <span class="n">{c.name}</span>
          <span class="q">{c.count} / {c.capacity}</span>
        </button>
      {:else}
        <p class="lo">No off-trader materials in the journal.</p>
      {/each}
    </div>
  {:else if kind}
    <div class="grid" style:--cols={kind.grades.length}>
      <div class="corner"></div>
      {#each kind.grades as g (g)}
        <div class="gh">G{g}</div>
      {/each}
      {#each kind.groups as g (g.name)}
        <div class="rh">{g.name}</div>
        {#each g.cells as c (c.id)}
          <button class="cell" class:sel={selId === c.id} class:empty={c.count === 0} style:background={fill(c)} onclick={() => pick(c.id)}>
            <span class="n">{c.name}</span>
            <span class="q">{c.count} / {c.capacity}</span>
          </button>
        {/each}
      {/each}
    </div>
  {/if}

  <aside class="about">
    {#if sel}
      <h3>{sel.name}</h3>
      <p>Grade {sel.grade} · {sel.rarity}</p>
      <p>{sel.kind === "raw" ? "Raw" : sel.kind === "encoded" ? "Encoded" : "Manufactured"} · {sel.group}</p>
      <p class="qty">{sel.count} / {sel.capacity}</p>
      <p class="lo">{sel.trader ? "Shown on the matching material trader." : "Not traded at material traders."}</p>
    {:else}
      <h3>{kind?.title ?? "Materials"}</h3>
      <p>Same layout as the in-game material trader: each row is a category, each column a grade. The fill is how close that material is to its storage cap.</p>
      <p class="lo">The game’s own inventory is a flat list. This screen is the trader view without flying to one.</p>
    {/if}
  </aside>
</section>

<style>
  section { padding: 8px 0 28px; display: grid; grid-template-columns: 1fr 16rem; grid-template-rows: auto auto 1fr; gap: 0 40px; min-height: 0; }
  .acts, .sub { grid-column: 1 / -1; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .acts { display: flex; flex-wrap: wrap; gap: 26px; padding: 14px 0 6px; text-transform: uppercase; letter-spacing: var(--caps); }
  .acts button { color: var(--text-lo); }
  .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0 0 14px; }
  .grid { display: grid; grid-template-columns: 11rem repeat(var(--cols), minmax(7.5rem, 1fr)); gap: var(--gap); align-content: start; }
  .gh, .rh, .corner { min-height: var(--row); display: flex; align-items: center; color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .gh { justify-content: center; }
  .rh { padding: 0 10px; }
  .cell { display: flex; flex-direction: column; justify-content: center; gap: 2px; min-height: 52px; padding: 6px 10px; color: var(--text); text-align: left; }
  .cell:hover { outline: 1px solid var(--accent-dim); outline-offset: -1px; }
  .cell.sel { outline: 1px solid var(--accent); outline-offset: -1px; }
  .cell.empty .n, .cell.empty .q { color: var(--text-lo); }
  .n { font-size: var(--fs-s); line-height: 1.2; }
  .q { font-family: var(--mono); font-size: var(--fs-s); }
  .other { display: grid; grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr)); gap: var(--gap); align-content: start; }
  .about { border-left: 2px solid var(--accent); padding-left: 24px; }
  .about h3 { margin: 0 0 8px; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  .about p { margin: 0 0 8px; line-height: 1.45; }
  .qty { color: var(--accent); font-family: var(--mono); }
  .lo { color: var(--text-lo); }
</style>
