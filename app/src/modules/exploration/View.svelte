<script lang="ts">
  // Exploration > System (DESIGN.md List with detail): the bodies of the system you are in as you
  // scan them, drawn as the system is built (stars, their planets and moons), each with its
  // mapped value, distance and type icon, the ones worth mapping standing out. Selecting a body
  // fills the side panel with its facts. Under it, the data aboard and where to sell it.
  import Badge from "../../lib/ui/Badge.svelte";
  import Glyph from "../../lib/ui/Glyph.svelte";
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { bodyGlyph, bodyTree, creditParts, credits, loadSystem, loadUnsold, type Body, type SystemView, type Unsold } from "../../lib/exploration";
  import Stat from "../../lib/ui/Stat.svelte";
  import { when } from "../../lib/logbook";
  import { serviceRoute } from "../../lib/places";
  import { tip } from "../../lib/ui/tip";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import ListDetail from "../../lib/ui/layout/ListDetail.svelte";
  import { at100 } from "../../lib/ui/layout/layout";
  import { scale } from "../../lib/scale.svelte";

  let view = $state<SystemView | null>(null);
  let failed = $state<string | null>(null);

  // Follows the journal: bodies appear as they are scanned.
  $effect(() => {
    const tick = () =>
      loadSystem()
        .then((v) => {
          // A new system drops the selection; a poll of the same one keeps it.
          if (view && (v.system_address !== view.system_address || v.system !== view.system)) picked = null;
          view = v;
          failed = null;
        })
        .catch((e: unknown) => (failed = pollFailure("read the system", e, view !== null)));
    void tick();
    const t = setInterval(() => void tick(), 3000);
    return () => clearInterval(t);
  });

  // The data aboard, from the history (it changes on a scan or a sale, not every second).
  let unsold = $state<Unsold | null>(null);
  let unsoldLoaded = $state(false);
  let unsoldFailed = $state<string | null>(null);
  $effect(() => {
    const tick = () =>
      loadUnsold()
        .then((u) => {
          unsold = u;
          unsoldLoaded = true;
          unsoldFailed = null;
        })
        .catch((e: unknown) => (unsoldFailed = pollFailure("read the unsold data", e, unsold !== null)));
    void tick();
    const t = setInterval(() => void tick(), 15000);
    return () => clearInterval(t);
  });

  // Where to sell it: links into the Services finder (decision astromech-g8d5, item 5).
  const sellAt = [
    { route: serviceRoute("universal_cartographics"), label: "Universal Cartographics", what: "cartographic data" },
    { route: serviceRoute("vista_genomics"), label: "Vista Genomics", what: "organic data" },
  ];

  const rows = $derived(view ? bodyTree(view.bodies) : []);
  const worth = $derived(view ? view.bodies.filter((b) => b.worth_mapping).length : 0);

  /** The selected body, by name: it stays selected across polls and goes with a jump. None at first. */
  let picked = $state<string | null>(null);
  const chosen = $derived(picked == null ? undefined : view?.bodies.find((b) => b.name === picked));

  /** "11 bodies, all scanned", "5 of 11 bodies scanned", "5 bodies scanned". */
  function counted(v: SystemView): string {
    const n = v.bodies.length;
    const of = v.body_count;
    if (of !== null && n >= of && v.all_found) return `${of} ${of === 1 ? "body" : "bodies"}, all scanned`;
    return of !== null ? `${n} of ${of} bodies scanned` : `${n} ${n === 1 ? "body" : "bodies"} scanned`;
  }

  /** A whole figure with its thousands: "1,612,400". */
  const full = (n: number) => Math.round(n).toLocaleString();

  /** A badge and what it means, for its Tip. */
  type Tag = { text: string; means: string };
  /** The tags a body has today, in the order the row shows them. */
  function tags(b: Body): Tag[] {
    const t: Tag[] = [];
    if (b.landable) t.push({ text: "Landable", means: "You can land on it" });
    if (b.terraformable && b.class !== "Earthlike body") t.push({ text: "Terraformable", means: "A candidate for terraforming, which adds to its value" });
    if (b.rings) t.push({ text: "Rings", means: "It has rings" });
    if (b.belts) t.push({ text: "Belt", means: "It has an asteroid belt" });
    if (b.bio_signals) t.push({ text: `Bio ${b.bio_signals}`, means: `${b.bio_signals} biological signals: species to sample (Exploration > Biology)` });
    if (b.geo_signals) t.push({ text: `Geo ${b.geo_signals}`, means: `${b.geo_signals} geological signals` });
    if (b.first_discovery) t.push({ text: "First discovery", means: "Nobody had discovered it before you" });
    if (b.worth_mapping) t.push({ text: "Worth mapping", means: "A planet worth at least 500 k cr mapped" });
    if (b.mapped) t.push({ text: "Mapped", means: b.mapped_efficiently ? "You mapped it within the efficiency target" : "You mapped it, over the efficiency target" });
    if (b.first_footfall_likely) t.push({ text: "First footfall likely", means: "Landable, and nobody had discovered it in this unpopulated system. The journal can't say who walked there first." });
    if (b.landings) t.push({ text: `Landed ${b.landings === 1 ? "once" : `${b.landings} times`}`, means: "Your landings here, from your journals" });
    return t;
  }

  /** "Discovered before, never mapped", "First discovery, mapped before". */
  function discovery(b: Body): string {
    const found = b.was_discovered === null ? "Not known if discovered" : b.was_discovered ? "Discovered before" : "First discovery";
    if (b.star || b.was_mapped === null) return found;
    return `${found}, ${b.was_mapped ? "mapped before" : "never mapped"}`;
  }

  function signals(b: Body): string {
    const s = [b.bio_signals && `${b.bio_signals} biological`, b.geo_signals && `${b.geo_signals} geological`].filter(Boolean);
    return s.length ? s.join(" · ") : "None";
  }

  function surface(b: Body): string {
    return [b.landable ? "Landable" : "Not landable", b.terraformable && b.class !== "Earthlike body" ? "terraformable" : ""].filter(Boolean).join(" · ");
  }

  /** The tree's keyboard: up and down move the selection, as a list of rows does. */
  function key(e: KeyboardEvent, i: number) {
    const to = e.key === "ArrowDown" ? i + 1 : e.key === "ArrowUp" ? i - 1 : e.key === "Home" ? 0 : e.key === "End" ? rows.length - 1 : null;
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      picked = rows[i]!.body.name;
      return;
    }
    if (e.key === "Escape") return void (picked = null);
    if (to === null || to < 0 || to >= rows.length) return;
    e.preventDefault();
    picked = rows[to]!.body.name;
    ((e.currentTarget as HTMLElement).parentElement?.children[to + 1] as HTMLElement | undefined)?.focus();
  }

  // What the values are, in the Tips on the head's values and the column head.
  const ESTIMATE = "Estimated with the community's exploration value formulae, the same ones EDSM and OD Explorer use";
  const IF_MAPPED = "The system's value once you map every body worth mapping, within the efficiency target";
  const MAPPED = "Each body's value mapped with the Detailed Surface Scanner, within the efficiency target";
  /**
   * The tree's own width with every row's tags on one line, in px at 100 percent Text size: the
   * narrowest it keeps beside the panel, so the panel goes below the tree before a tag wraps
   * (DESIGN.md The Aligned Columns Rule). The value, distance and body columns are max-content, so
   * their tracks are their natural widths whatever the room; the tags are measured badge by badge,
   * since their cell wraps when it must (a stacked page narrower than the tree).
   */
  let tree = $state<HTMLElement>();
  let natural = $state(0);
  function measure() {
    if (!tree) return;
    const cs = getComputedStyle(tree);
    const tracks = cs.gridTemplateColumns.split(" ").map(parseFloat);
    const gap = parseFloat(cs.columnGap) || 0;
    const row = tree.querySelector<HTMLElement>(".row.body");
    const rs = row ? getComputedStyle(row) : null;
    const pad = rs ? parseFloat(rs.paddingLeft) + parseFloat(rs.paddingRight) : 0;
    let tags = 0;
    for (const cell of tree.querySelectorAll<HTMLElement>(".tags")) {
      const badges = Array.from(cell.children) as HTMLElement[];
      const between = parseFloat(getComputedStyle(cell).columnGap) || 0;
      const w = badges.reduce((sum, b) => sum + b.getBoundingClientRect().width, 0) + between * Math.max(0, badges.length - 1);
      tags = Math.max(tags, w);
    }
    const px = (tracks[0] ?? 0) + (tracks[1] ?? 0) + (tracks[2] ?? 0) + tags + 3 * gap + pad;
    natural = Math.ceil(at100(px, scale.step));
  }
  $effect(() => {
    void rows;
    void scale.step;
    if (!tree) return;
    measure();
    const ro = new ResizeObserver(measure);
    ro.observe(tree);
    return () => ro.disconnect();
  });
</script>

<ListDetail panel={chosen ? about : null} selected={picked} min={natural || undefined}>
  <div class="page">
    <Stale error={view ? failed : null} />
    {#if failed && !view}
      <p class="err">{failed}</p>
    {:else if !view}
      <p class="type-label">Reading the journal…</p>
    {:else if !view.system}
      <p class="type-label">No system in the journal yet. Jump somewhere and its bodies show here as you scan them.</p>
    {:else}
      <header>
        <div class="who">
          <h2 class="title"><span class="type-title">{view.system}</span><SystemChip name={view.system} icon /></h2>
          <p class="type-label">{counted(view)}{worth ? ` · ${worth} worth mapping` : ""}{view.source ? ` · ${view.source}` : ""}</p>
        </div>
        <div class="values">
          <p {@attach tip(ESTIMATE)}><span class="type-value-xl">{full(view.value)}</span><span class="type-unit">cr now</span></p>
          <p {@attach tip(IF_MAPPED)}><span class="type-value-xl main">{full(view.value_if_mapped)}</span><span class="type-unit">cr if mapped</span></p>
        </div>
      </header>
    {/if}
    {#if unsold?.risky}
      <p class="risk" role="alert"><b>!</b> {credits(unsold.total)} aboard. Dying loses all of it: sell it at the nearest station (links below).</p>
    {/if}
    {#if view?.system}
      {#if rows.length}
        <div class="tree" role="treegrid" aria-label="Bodies" bind:this={tree}>
          <div class="row heads" role="row">
            <span class="r type-label" role="columnheader" {@attach tip(MAPPED)}>Mapped value</span>
            <span class="r type-label" role="columnheader">Distance</span>
            <span class="type-label" role="columnheader">Body</span>
            <span role="columnheader" aria-label="Tags"></span>
          </div>
          {#each rows as r, i (r.body.name)}
            {@const b = r.body}
            {@const sel = chosen?.name === b.name}
            <div
              class="row body"
              class:sel
              class:worth={b.worth_mapping}
              role="row"
              aria-level={r.depth + 1}
              aria-selected={sel}
              tabindex={(chosen ? sel : i === 0) ? 0 : -1}
              onclick={() => (picked = b.name)}
              onkeydown={(e) => key(e, i)}
            >
              <span class="r" role="gridcell"><span class={b.worth_mapping ? "type-value-l v" : "type-figures v"}>{full(b.value_mapped)}</span><span class="type-unit">cr</span></span>
              <span class="r" role="gridcell"><span class="type-figures d">{b.distance_ls === null ? "–" : full(b.distance_ls)}</span>{#if b.distance_ls !== null}<span class="type-unit">ls</span>{/if}</span>
              <span class="node" role="gridcell">
                {#each r.through as on, j (j)}<span class="rail" class:on></span>{/each}
                {#if r.depth}<span class="rail elbow" class:last={r.last}></span>{/if}
                <span class="icon" class:kids={r.kids}><Glyph name={bodyGlyph(b)} label={b.class_name} /></span>
                <span class="type-name name">{b.short_name}</span>
                <span class="type-figures cls">{b.class_name}</span>
              </span>
              <span class="tags" role="gridcell">
                {#each tags(b) as t (t.text)}<Badge {@attach tip({ title: t.text, fact: t.means, stop: false })}>{t.text}</Badge>{/each}
              </span>
            </div>
          {/each}
        </div>
      {:else}
        <p class="type-label">Nothing scanned here yet. Honk and use the FSS: bodies show here as you scan them.</p>
      {/if}
    {/if}
  </div>
</ListDetail>

<!-- DESIGN.md Side panel: the body's short name in white, its system, its mapped value as the
     panel's main value, then its facts; OPEN IN BIOLOGY where it has biology. -->
{#snippet about()}
  {#if chosen && view}
    {@const b = chosen}
    <h2 class="type-title ptitle">{b.short_name}</h2>
    <p class="type-label psub">{view.system}</p>
    <p class="pmain"><span class="type-value-xl main">{full(b.value_mapped)}</span><span class="type-unit">{b.star ? "cr" : "cr mapped"}</span></p>
    {#if !b.star}
      <p class="type-label">{full(b.value)} cr now · {b.mapped ? (b.mapped_efficiently ? "mapped" : "mapped, over the efficiency target") : "not mapped yet"}</p>
    {/if}
    <dl class="pfacts">
      <dt class="type-label">Class</dt><dd class="type-figures">{b.class_name}</dd>
      <dt class="type-label">Distance</dt><dd class="type-figures">{b.distance_ls === null ? "Not known" : `${full(b.distance_ls)} ls`}</dd>
      {#if b.gravity_g !== null}<dt class="type-label">Gravity</dt><dd class="type-figures">{b.gravity_g.toFixed(2)} g</dd>{/if}
      {#if !b.star}
        <dt class="type-label">Signals</dt><dd class="type-figures">{signals(b)}</dd>
        <dt class="type-label">Surface</dt><dd class="type-figures">{surface(b)}</dd>
      {/if}
      <dt class="type-label">Discovery</dt><dd class="type-figures">{discovery(b)}</dd>
      {#if b.landable}<dt class="type-label">Landings</dt><dd class="type-figures">{b.landings === 0 ? "None yet" : b.landings === 1 ? "Once" : `${b.landings} times`}</dd>{/if}
    </dl>
    <!-- The row's tags again, each with its Tip and a tab stop: the keyboard's way to what a
         badge means (in the tree the row is the one stop, so its badges take none). -->
    {#if tags(b).length}
      <div class="ptags" role="group" aria-label="Tags">{#each tags(b) as t (t.text)}<Badge {@attach tip({ title: t.text, fact: t.means })}>{t.text}</Badge>{/each}</div>
    {/if}
    {#if b.bio_signals}<p class="pacts"><a class="type-action" href="#/exobiology">Open in Biology</a></p>{/if}
  {/if}
{/snippet}

<div class="after">
  <section aria-labelledby="ex-unsold">
    <h3 class="section" id="ex-unsold">Unsold data</h3>
    <Stale error={unsold ? unsoldFailed : null} />
    {#if unsoldFailed && !unsold}
      <p class="err">{unsoldFailed}</p>
    {:else if unsold}
      {@const [carto, cartoUnit] = creditParts(unsold.cartographic)}
      {@const [organic, organicUnit] = creditParts(unsold.organic)}
      {@const [total, totalUnit] = creditParts(unsold.total)}
      <div class="tiles">
        <Stat label="Cartographic" value={carto} unit={cartoUnit}>{unsold.bodies.toLocaleString()} bodies in {unsold.systems.toLocaleString()} systems</Stat>
        <Stat label="Organic" value={organic} unit={organicUnit}>{unsold.samples} samples{unsold.unknown_samples ? `, ${unsold.unknown_samples} of unknown value` : ""}</Stat>
        <Stat label="Total" value={total} unit={totalUnit} />
      </div>
      <p class="type-label">
        CMDR {unsold.commander.name}{unsold.since ? ` · unsold since ${when(unsold.since)} UTC` : ""}{unsold.source ? ` · ${unsold.source}` : ""}
      </p>
    {:else if unsoldLoaded}
      <p class="type-label">No commander in the journal history yet.</p>
    {:else}
      <p class="type-label">Reading the journal history…</p>
    {/if}
  </section>

  <section aria-labelledby="ex-sell">
    <h3 class="section" id="ex-sell">Where to sell</h3>
    <ul class="sell">
      {#each sellAt as l (l.route)}
        <li><a href="#/{l.route}">Nearest {l.label}</a> <span class="type-label">for {l.what}</span></li>
      {/each}
    </ul>
  </section>
</div>

<style>
  p, h2, dl, dd { margin: 0; }
  .err { color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }

  /* The head and the tree keep the tree's own width (DESIGN.md The Next To It Rule): the values
     end where the rows do, not at the window's edge. */
  .page { display: flex; flex-direction: column; gap: var(--sp-6); width: fit-content; max-width: 100%; }
  header { display: flex; flex-wrap: wrap; align-items: flex-end; justify-content: space-between; gap: var(--sp-4) var(--sp-12); }
  .who { display: flex; flex-direction: column; gap: var(--sp-1); }
  .title { display: flex; align-items: center; gap: var(--sp-1); }
  .values { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--sp-2) var(--sp-8); }
  /* A warning: an orange "!" and plain words, never a filled block (DESIGN.md States). */
  .risk { color: var(--accent); max-width: var(--measure); }
  .risk b { font-weight: 600; }

  /* The tree: the value and distance right-aligned, the body with its lines, then its tags. Rows
     touch, so the lines from a parent to its children run unbroken. */
  .tree { display: grid; grid-template-columns: max-content max-content max-content auto; column-gap: var(--sp-6); }
  .row { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: center; min-height: var(--row); padding: 0 var(--sp-3); }
  .heads { min-height: 0; padding-bottom: var(--sp-2); border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); }
  .r { text-align: right; white-space: nowrap; }
  .body { cursor: pointer; }
  .body:hover { background: var(--row-hover); }
  .body:focus-visible { outline: 1px solid var(--accent); outline-offset: -1px; }
  .v, .d { color: var(--prose); }
  .worth .v { color: var(--options-grey); }
  .node { display: flex; align-items: center; align-self: stretch; white-space: nowrap; }
  /* A rail is one level of indent; its line runs down the middle of the parent's icon above. */
  .rail { position: relative; align-self: stretch; padding-left: var(--sp-6); }
  .rail.on::before, .elbow::before { content: ""; position: absolute; left: var(--sp-2); top: 0; bottom: 0; border-left: 1px solid var(--line); }
  .elbow.last::before { bottom: 50%; }
  .elbow::after { content: ""; position: absolute; left: var(--sp-2); right: var(--sp-1); top: 50%; border-top: 1px solid var(--line); }
  .icon { position: relative; display: flex; align-items: center; align-self: stretch; color: var(--label-grey); }
  /* A body with children: its line leaves the icon downward to the first child's elbow. */
  .icon.kids::after { content: ""; position: absolute; left: var(--sp-2); top: calc(50% + var(--sp-2) + var(--gap)); bottom: 0; border-left: 1px solid var(--line); }
  /* Names keep a common start for their classes within a level (most are "A 1"-short). */
  .name { min-width: var(--sp-12); margin-left: var(--sp-2); }
  .cls { margin-left: var(--sp-4); }
  /* One line beside the panel (the panel stacks first); wrapped only on a stacked page narrower
     than the tree, rather than scrolling sideways. */
  .tags { display: flex; flex-wrap: wrap; gap: var(--sp-1); padding-block: var(--sp-1); }
  /* Selected, while the side panel shows it (DESIGN.md States): Hull Raised, the name White,
     other text one step brighter. The tree is not ranked, so no value turns orange. */
  .body.sel, .body.sel:hover { background: var(--surface-hi); }
  .sel .name { color: var(--text-hi); }
  .sel :is(.v, .d, .cls) { color: var(--options-grey); }
  .sel :global(.type-unit), .sel .icon { color: var(--prose); }

  /* The side panel. */
  .ptitle { overflow-wrap: anywhere; }
  .psub { margin-top: var(--sp-1); color: var(--prose); }
  .pmain { margin-top: var(--sp-6); margin-bottom: var(--sp-2); }
  .pfacts { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: var(--sp-2) var(--sp-6); margin-top: var(--sp-6); }
  .pfacts dt { align-self: baseline; }
  .ptags { display: flex; flex-wrap: wrap; gap: var(--sp-1); margin-top: var(--sp-6); }
  .pacts { margin-top: var(--sp-8); }
  .pacts a { text-decoration: none; }
  .pacts a:hover { color: var(--orange-lit); }

  /* Under the tree: the data aboard, as tiles with one value each, and where to sell it. */
  .after { display: flex; flex-direction: column; gap: var(--sp-8); padding-bottom: var(--sp-12); }
  section { display: flex; flex-direction: column; gap: var(--sp-3); }
  .tiles { display: flex; flex-wrap: wrap; gap: var(--gap); }
  .sell { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: var(--sp-2); }
  /* Links to somewhere else (DESIGN.md Buttons → Links). */
  .sell a { color: var(--accent); font-weight: 500; text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .sell a:hover { color: var(--orange-lit); }
</style>
