<script lang="ts">
  // Journal-fed trader grids: the in-game inventory is a flat list with no grades. Where to
  // find raw sites and the systems a material's source needs are Places finders: this page
  // links into them with the site kind or faction state filled in (decision astromech-g8d5, item 5).
  import { pollFailure } from "../../lib/ui/errors";
  import Choice from "../../lib/ui/Choice.svelte";
  import { hashParam, pageHash } from "../../lib/hashstate";
  import Stale from "../../lib/ui/Stale.svelte";
  import { loadMaterials, type Cell, type Inventory } from "../../lib/materials";
  import { factionRoute, siteRoute } from "../../lib/places";
  import { SITE_KINDS } from "../../lib/sites";
  import ExtLinks from "../../lib/ui/ExtLinks.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import Board from "../../lib/ui/layout/Board.svelte";
  import Tiles from "../../lib/ui/layout/Tiles.svelte";
  import { tilesMin } from "../../lib/ui/layout/layout";

  /** Between the raw-site links (a literal space at a tag edge would be trimmed). */
  const SEP = ", ";
  let inv = $state<Inventory | null>(null);
  let failed = $state<string | null>(null);
  // A material opens at #/materials/<id> (Find or plot links there) and a kind at
  // #/materials?kind=encoded, so back and reload keep them.
  const readSel = () => {
    const m = /^#\/materials\/([^?]+)/.exec(location.hash);
    if (!m?.[1]) return null;
    try {
      return decodeURIComponent(m[1]);
    } catch {
      return null; // a mangled link opens the page with nothing selected
    }
  };
  let selId = $state<string | null>(readSel());
  let wantKind = $state(hashParam("kind"));

  const kinds = $derived([
    ...(inv?.kinds.map((k) => ({ value: k.id, label: k.title })) ?? []),
    ...(inv && inv.other.length ? [{ value: "other", label: "Other" }] : []),
  ]);
  // The selected material's kind, else the one the link asks for, else Raw.
  const selKind = $derived.by(() => {
    const id = selId;
    if (!inv || !id) return undefined;
    if (inv.other.some((c) => c.id === id)) return "other";
    return inv.kinds.find((kd) => kd.groups.some((g) => g.cells.some((c) => c.id === id)))?.id;
  });
  const kindId = $derived(selKind ?? (kinds.some((k) => k.value === wantKind) ? wantKind! : "raw"));
  const kind = $derived(inv?.kinds.find((k) => k.id === kindId) ?? inv?.kinds[0]);
  const sel = $derived(
    kindId === "other"
      ? inv?.other.find((c) => c.id === selId)
      : kind?.groups.flatMap((g) => g.cells).find((c) => c.id === selId),
  );

  $effect(() => {
    const onHash = () => {
      selId = readSel();
      wantKind = hashParam("kind");
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
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

  // DESIGN.md's material tile: a 2 px track along the bottom edge fills Orange Mid with
  // count / cap, Orange Lit at the cap. A tile with none has no track.
  function track(c: Cell) {
    const p = c.capacity > 0 ? Math.min(1, c.count / c.capacity) : 0;
    // A few units of a 300-cap is otherwise a 1px sliver; keep a readable min fill while held.
    const vis = c.count > 0 ? Math.max(p, 0.04) : 0;
    const ink = p >= 1 ? "var(--orange-lit)" : "var(--orange-mid)";
    return `${ink} ${vis * 100}%, var(--line) ${vis * 100}%`;
  }

  function pick(id: string) {
    location.hash = `#/materials/${encodeURIComponent(id)}`;
  }
  const chooseKind = (k: string) => (location.hash = pageHash("materials", { kind: k === "raw" ? null : k }));

  /** The board's widest grid: the most grades of any kind, with the row heads. */
  const min = $derived(tilesMin(Math.max(4, ...(inv?.kinds.map((k) => k.grades.length) ?? [])), true));
</script>

<!-- DESIGN.md Board with its side panel: the grid of grades and categories, and the selected
     material, or the kind when none is selected. -->
<Board panel={about} selected={sel?.id} {min}>
  <div class="find">
    <div class="query">
      <div class="bar">
        {#if kinds.length}<Choice label="Kind" options={kinds} value={kindId} onchange={chooseKind} />{/if}
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
      </div>
      <Stale error={inv ? failed : null} />
      {#if kindId === "manufactured" && inv?.here}
        <p class="sub here">
          <SystemChip name={inv.here.system} />{inv.here.line.startsWith(inv.here.system) ? inv.here.line.slice(inv.here.system.length) : ` · ${inv.here.line}`} <ExtLinks system={inv.here.system} />
        </p>
      {/if}
    </div>

    <div class="out">
      {#if kindId === "other" && inv}
        <Tiles label="Other materials">
          {#each inv.other as c (c.id)}
            <button class="cell" class:sel={selId === c.id} style:--track={track(c)} onclick={() => pick(c.id)}>
              <span class="n">{c.name}</span>
              <span class="q">{c.count} / {c.capacity}</span>
            </button>
          {:else}
            <p class="lo">No off-trader materials in the journal.</p>
          {/each}
        </Tiles>
      {:else if kind}
        <Tiles cols={kind.grades.length} heads label={kind.title}>
          <div class="corner"></div>
          {#each kind.grades as g (g)}
            <div class="gh">G{g}</div>
          {/each}
          {#each kind.groups as g (g.name)}
            <div class="rh">{g.name}</div>
            {#each g.cells as c (c.id)}
              <button class="cell" class:sel={selId === c.id} class:empty={c.count === 0} style:--track={track(c)} onclick={() => pick(c.id)}>
                <span class="n">{c.name}</span>
                <span class="q">{c.count} / {c.capacity}</span>
              </button>
            {/each}
          {/each}
        </Tiles>
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
        {#if kindId === "raw"}
          <p class="sites">
            Raw sites near you, in Places:
            {#each SITE_KINDS as k, i (k.id)}{#if i}<span aria-hidden="true">{SEP}</span>{/if}<a href="#/{siteRoute(k.id)}">{k.label}</a>{/each}
          </p>
        {/if}
      {/if}
    </div>
  </div>

</Board>

{#snippet about()}
  <div class="about">
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
      {#if lookup}<p><a class="link" href="#/{factionRoute(lookup)}">Find the nearest systems</a></p>{/if}
    {:else}
      <h3>{kind?.title ?? "Materials"}</h3>
      {#if kindId === "other"}
        <p class="lo">Guardian, Thargoid and anything the journal named that is not on a trader.</p>
      {:else if kind}
        <p class="qty">{kind.held.toLocaleString()} / {kind.capacity.toLocaleString()}</p>
        {#if kind.trader}<p class="lo">{kind.trader}</p>{/if}
      {/if}
    {/if}
  </div>
{/snippet}

<style>
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .find { min-width: 0; }
  /* The kind is a choice on the page, not a third tab row (DESIGN.md The Two Tab Rows Rule). */
  .query { display: flex; flex-direction: column; gap: var(--sp-3); }
  .bar { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-4); }
  .sub { color: var(--orange-mid); font-size: var(--fs-s); margin: 0; }
  .here { color: var(--accent); }
  .out { padding: var(--sp-6) 0 0; min-width: 0; }
  .farm { list-style: none; margin: 24px 0 0; padding: 0; display: flex; flex-direction: column; }
  .farm li { display: grid; grid-template-columns: minmax(10rem, 14rem) 5rem minmax(0, var(--measure)); gap: 10px 18px; align-items: baseline; padding: 14px 8px; border-bottom: 1px solid var(--line); font-size: var(--fs-s); }
  .farm button { font-weight: 500; }
  .farm button:hover, .farm .on { color: var(--orange-lit); }
  .gh, .rh, .corner { min-height: 40px; display: flex; align-items: center; color: var(--orange-mid); font-size: var(--fs-s); }
  .gh { justify-content: center; }
  .rh { padding: 0 10px; }
  /* Read-only tiles on the hull (DESIGN.md Material tile); none held is Void inside a Seam
     outline, all Orange Dim; the selected one, which the side panel explains, is solid orange. */
  .cell { display: flex; flex-direction: column; justify-content: center; gap: 4px; min-height: 64px; padding: 10px 12px; text-align: left; background: linear-gradient(90deg, var(--track)) bottom / 100% 2px no-repeat, var(--surface); }
  .cell:hover { background-color: var(--row-hover); }
  .cell.empty { background: var(--bg); outline: 1px solid var(--line); outline-offset: -1px; }
  .cell.empty .n, .cell.empty .q { color: var(--orange-dim); }
  /* The Seam outline must not hide the focus ring or the hover (review astromech-hdht). */
  .cell.empty:hover { background: var(--row-hover); }
  .cell.empty:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .cell.sel, .cell.sel:hover { background: var(--accent); outline: 0; }
  .cell.sel :is(.n, .q) { color: var(--on-accent); }
  .cell.sel:focus-visible { outline: 1px solid var(--on-accent); outline-offset: -3px; }
  .n { font-size: var(--fs-s); line-height: 1.25; }
  .q { font-size: var(--fs-s); }
  .farm .q { text-align: right; }
  .about h3 { margin: 0 0 8px; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  .about p { margin: 0 0 10px; line-height: 1.45; }
  .qty { color: var(--accent); }
  .lo { color: var(--orange-mid); }
  .sites { margin: 24px 0 0; padding: 0 8px; font-size: var(--fs-s); color: var(--orange-mid); }
  /* Links to somewhere else (DESIGN.md Buttons → Links). */
  .sites a, .link { color: var(--accent); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .sites a:hover, .link:hover { color: var(--orange-lit); }
</style>
