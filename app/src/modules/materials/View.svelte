<script lang="ts">
  // Journal-fed trader grids: the in-game inventory is a flat list with no grades.
  import { tick } from "svelte";
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { loadMaterials, type Cell, type Inventory } from "../../lib/materials";
  import Nearby from "../../lib/ui/Nearby.svelte";
  import Sites from "../../lib/ui/Sites.svelte";
  import ExtLinks from "../../lib/ui/ExtLinks.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";

  let inv = $state<Inventory | null>(null);
  let failed = $state<string | null>(null);
  let kindId = $state("raw");
  // A material opens at #/materials/<id> (the command palette links there), so back and reload keep it.
  const readSel = () => {
    const m = /^#\/materials\/(.+)$/.exec(location.hash);
    if (!m?.[1]) return null;
    try {
      return decodeURIComponent(m[1]);
    } catch {
      return null; // a mangled link opens the page with nothing selected
    }
  };
  let selId = $state<string | null>(readSel());

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
    const onHash = () => (selId = readSel());
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  // Show the tab the selected material is on.
  $effect(() => {
    if (!inv || !selId) return;
    const id = selId;
    const k = inv.other.some((c) => c.id === id) ? "other" : inv.kinds.find((kd) => kd.groups.some((g) => g.cells.some((c) => c.id === id)))?.id;
    if (k) kindId = k;
  });

  $effect(() => {
    const load = () =>
      loadMaterials()
        .then((v) => {
          inv = v;
          failed = null;
        })
        .catch((e: unknown) => (failed = pollFailure("read the materials", e, inv !== null)));
    void load();
    // 1 s poll + recompute keeps journal changes on screen within 2 s (astromech-txzq).
    const t = setInterval(() => void load(), 1000);
    return () => clearInterval(t);
  });

  function fill(c: Cell) {
    const p = c.capacity > 0 ? Math.min(1, c.count / c.capacity) : 0;
    // A few units of a 300-cap is otherwise a 1px sliver; keep a readable min fill while held.
    const vis = c.count > 0 ? Math.max(p, 0.12) : 0;
    return `linear-gradient(90deg, var(--cell) ${vis * 100}%, var(--cell-empty) ${vis * 100}%)`;
  }

  let about: HTMLElement | undefined = $state();

  function pick(id: string | null) {
    location.hash = id ? `#/materials/${encodeURIComponent(id)}` : "#/materials";
  }

  // The detail sits below the grid; on a short window it is off screen, so bring it up once per pick.
  let shownId: string | null = null;
  $effect(() => {
    if (!sel) return void (shownId = null);
    if (sel.id === shownId) return;
    shownId = sel.id;
    void tick().then(() => about?.scrollIntoView({ block: "nearest", behavior: "smooth" }));
  });
</script>

<section>
  <div class="find">
    <div class="query">
      <div class="acts">
        {#each tabs as k (k.id)}
          <button class:on={kindId === k.id} onclick={() => { kindId = k.id; pick(null); }}>{k.title}</button>
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
          {failed ?? "Reading the journal…"}
        {/if}
      </p>
      <Stale error={inv ? failed : null} />
      {#if kindId === "manufactured" && inv?.here}
        <p class="sub here">
          <SystemChip name={inv.here.system} />{inv.here.line.startsWith(inv.here.system) ? inv.here.line.slice(inv.here.system.length) : ` · ${inv.here.line}`} <ExtLinks system={inv.here.system} />
        </p>
      {/if}
    </div>

    <div class="out">
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
        {#if kindId === "manufactured" && inv?.farm && inv.farm.length}
          <ul class="farm">
            {#each inv.farm as f (f.id)}
              <li>
                <button class:on={selId === f.id} onclick={() => pick(f.id)}>{f.name}</button>
                <span class="q">{f.count}/{f.capacity}</span>
                <span class="lo">{f.summary}</span>
              </li>
            {/each}
          </ul>
        {/if}
        {#if kindId === "raw"}<Sites />{/if}
      {/if}
    </div>
  </div>

  <aside class="about" bind:this={about}>
    {#if sel}
      <h3>{sel.name}</h3>
      <p>Grade {sel.grade} · {sel.rarity}</p>
      <p>{sel.kind === "raw" ? "Raw" : sel.kind === "encoded" ? "Encoded" : "Manufactured"} · {sel.group}</p>
      <p class="qty">{sel.count} / {sel.capacity}</p>
      {#if sel.sources?.length}
        {#each sel.sources as s, i (i)}
          <p>{s.summary}</p>
          <p class="lo">{s.hint}</p>
        {/each}
      {/if}
      <p class="lo">{sel.trader ? "Shown on the matching material trader." : "Not traded at material traders."}</p>
      {@const lookup = sel.sources?.find((s) => s.query)?.query}
      {#if lookup}<Nearby query={lookup} />{/if}
    {:else}
      <h3>{kind?.title ?? "Materials"}</h3>
      {#if kindId === "other"}
        <p class="lo">Guardian, Thargoid and anything the journal named that is not on a trader.</p>
      {:else if kind}
        <p class="qty">{kind.held.toLocaleString()} / {kind.capacity.toLocaleString()}</p>
        {#if kind.trader}<p class="lo">{kind.trader}</p>{/if}
      {/if}
    {/if}
  </aside>
</section>

<style>
  section { display: grid; grid-template-columns: minmax(0, 1fr) minmax(18rem, 22rem); gap: 0; min-height: 0; padding: 0 0 28px; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .find { min-width: 0; }
  .query { display: flex; flex-direction: column; gap: 12px; padding: 18px 20px 20px; background: var(--bg-list); border-bottom: 1px solid var(--accent-dim); }
  .acts { display: flex; flex-wrap: wrap; gap: 26px; text-transform: uppercase; letter-spacing: var(--caps); }
  .acts button { color: var(--text-lo); }
  .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0; }
  .here { color: var(--text); }
  .out { padding: 24px 8px 8px 4px; min-width: 0; }
  .farm { list-style: none; margin: 24px 0 0; padding: 0; display: flex; flex-direction: column; }
  .farm li { display: grid; grid-template-columns: minmax(10rem, 14rem) 5rem 1fr; gap: 10px 18px; align-items: baseline; padding: 14px 8px; border-bottom: 1px solid #1a1c1e; font-size: var(--fs-s); }
  .farm button { color: var(--text-hi); }
  .farm button:hover, .farm .on { color: var(--accent); }
  .grid { display: grid; grid-template-columns: 10rem repeat(var(--cols), minmax(7.5rem, 1fr)); gap: 8px; align-content: start; }
  .gh, .rh, .corner { min-height: 40px; display: flex; align-items: center; color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .gh { justify-content: center; }
  .rh { padding: 0 10px; }
  .cell { display: flex; flex-direction: column; justify-content: center; gap: 4px; min-height: 64px; padding: 10px 12px; color: var(--text); text-align: left; }
  .cell:hover { outline: 1px solid var(--accent-dim); outline-offset: -1px; }
  .cell.sel { outline: 1px solid var(--accent); outline-offset: -1px; }
  .cell.empty .n, .cell.empty .q { color: var(--text-lo); }
  .n { font-size: var(--fs-s); line-height: 1.25; }
  .q { font-family: var(--mono); font-size: var(--fs-s); }
  .other { display: grid; grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr)); gap: 8px; align-content: start; }
  .about { position: sticky; top: 0; align-self: start; max-height: calc(100vh - 150px); overflow-y: auto; border-left: 1px solid var(--accent-dim); background: var(--bg-list); padding: 20px 8px 28px 24px; }
  .about h3 { margin: 0 0 8px; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  .about p { margin: 0 0 10px; line-height: 1.45; }
  .qty { color: var(--accent); font-family: var(--mono); }
  .lo { color: var(--text-lo); }
  @media (max-width: 1100px) {
    section { display: flex; flex-direction: column; }
    .about { position: static; max-height: none; border-left: 0; border-top: 1px solid var(--accent-dim); padding: 16px 0 20px; }
    .out { padding: 20px 0 8px; }
  }
</style>
