<script lang="ts">
  // One kind's engineers as the tree they are (tone5-engineers-rest-2560.png): columns by
  // `Node.depth`, each engineer beside the ones they introduce, with a Seam line from each
  // engineer to each of their introducers. A node is a Seam outline on Void: the name, then one
  // line, coloured by DESIGN.md's engineer states. Selecting one raises it; the lines to its
  // introducers and their outlines light in Orange Mid.
  import { placeTree, type UnlockNode } from "../../lib/unlocks";
  import Hexes from "../../lib/ui/Hexes.svelte";
  import { tip } from "../../lib/ui/tip";

  let {
    title,
    nodes,
    selected,
    onselect,
  }: {
    title: string;
    nodes: UnlockNode[];
    /** The selected engineer's name, in either tree. */
    selected: string | null;
    onselect: (name: string) => void;
  } = $props();

  const layout = $derived(placeTree(nodes));
  const sel = $derived(nodes.find((n) => n.name === selected) ?? null);
  const preds = $derived(sel?.introduced_by ?? []);

  /** The state's meaning, for the node's Tip (no printed explanations). */
  function meaning(n: UnlockNode): string {
    if (n.status === "unlocked") return n.kind === "ship" ? `Unlocked, rank ${n.rank ?? 1} of 5` : "Unlocked";
    if (n.status === "barred") return "Barred: this engineer won't work for you";
    if (n.next) return "You can work on this engineer now";
    return `Not introduced yet: ${n.left ?? ""}`;
  }

  // The lines, drawn from where the nodes are after layout, and again whenever the grid resizes.
  // The nodes' elements are not state: drawing reads them, it doesn't follow them.
  let grid = $state<HTMLElement>();
  // eslint-disable-next-line svelte/prefer-svelte-reactivity
  const els = new Map<string, HTMLElement>();
  function track(el: HTMLElement, name: string) {
    els.set(name, el);
    return { destroy: () => els.delete(name) };
  }
  type Edge = { d: string; lit: boolean; key: string };
  let edges = $state<Edge[]>([]);
  let size = $state({ w: 0, h: 0 });
  function draw() {
    if (!grid) return;
    const box = grid.getBoundingClientRect();
    size = { w: box.width, h: box.height };
    const out: Edge[] = [];
    for (const n of nodes) {
      const to = els.get(n.name)?.getBoundingClientRect();
      if (!to) continue;
      for (const r of n.introduced_by) {
        const from = els.get(r)?.getBoundingClientRect();
        if (!from) continue;
        const x1 = from.right - box.left;
        const y1 = from.top + from.height / 2 - box.top;
        const x2 = to.left - box.left;
        const y2 = to.top + to.height / 2 - box.top;
        const mx = Math.round((x1 + x2) / 2) + 0.5;
        out.push({ key: `${r}>${n.name}`, d: `M${x1} ${y1}H${mx}V${y2}H${x2}`, lit: n.name === selected });
      }
    }
    // The lit lines last, so they are drawn over the Seam ones they share a stretch with.
    edges = out.sort((a, b) => Number(a.lit) - Number(b.lit));
  }
  $effect(() => {
    void layout;
    void selected;
    if (!grid) return;
    draw();
    const ro = new ResizeObserver(() => draw());
    ro.observe(grid);
    return () => ro.disconnect();
  });
</script>

<section class="tree" aria-label={title}>
  <h2 class="section" class:current={sel !== null}>{title}</h2>
  <div class="grid" bind:this={grid} style:--cols={layout.cols} style:--rows={layout.rows}>
    <svg class="edges" width={size.w} height={size.h} aria-hidden="true">
      {#each edges as e (e.key)}<path d={e.d} class:lit={e.lit} />{/each}
    </svg>
    {#each layout.placed as p (p.node.name)}
      {@const n = p.node}
      <button
        type="button"
        class="node {n.status}"
        class:next={n.next && n.status !== "unlocked"}
        class:sel={n.name === selected}
        class:pred={preds.includes(n.name)}
        style:grid-column={p.col + 1}
        style:grid-row="{p.row + 1} / span {p.span}"
        aria-label={n.name}
        aria-pressed={n.name === selected}
        onclick={() => onselect(n.name)}
        use:track={n.name}
        {@attach tip({ title: n.name, fact: meaning(n) })}
      >
        <span class="type-name name">{n.name}</span>
        {#if n.status === "unlocked"}
          <!-- A suit engineer has no ranks: one lit hexagon says unlocked. -->
          <span class="line"><Hexes n={n.kind === "ship" ? Math.max(1, Math.min(5, n.rank ?? 1)) : 1} of={n.kind === "ship" ? 5 : 1} /><span class="type-label">{n.system}</span></span>
        {:else if n.status === "barred" || n.next}
          <span class="type-label line step">{n.left}</span>
        {:else}
          <span class="type-label line">Not introduced</span>
        {/if}
      </button>
    {/each}
  </div>
</section>

<style>
  .tree { display: flex; flex-direction: column; gap: var(--sp-3); min-width: 0; }
  .grid { position: relative; display: grid; grid-template-columns: repeat(var(--cols), var(--node)); grid-template-rows: repeat(var(--rows), auto); gap: var(--sp-2) var(--sp-6); }
  .edges { position: absolute; inset: 0; pointer-events: none; overflow: visible; }
  .edges path { fill: none; stroke: var(--line); stroke-width: 1; }
  .edges path.lit { stroke: var(--orange-mid); }
  /* DESIGN.md Surfaces: a node is a Seam outline on Void, never a cell. */
  .node { all: unset; box-sizing: border-box; position: relative; align-self: center; display: flex; flex-direction: column; gap: var(--sp-1); padding: var(--sp-2) var(--sp-3); background: var(--bg); outline: 1px solid var(--line); outline-offset: -1px; cursor: pointer; }
  .node:hover { background: var(--row-hover); }
  .node:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  /* Selected: Hull Raised, the name White (DESIGN.md States). */
  .node.sel { background: var(--surface-hi); }
  .node.sel .name { color: var(--text-hi); }
  /* The engineers who introduce the selected one. */
  .node.pred { outline-color: var(--orange-mid); }
  .line { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; }
  /* DESIGN.md's engineer states. */
  .node.next:not(.sel) .name { color: var(--orange-mid); }
  .step { color: var(--prose); text-wrap: pretty; }
  .node.barred:not(.sel) .name, .node.barred .step { color: var(--red-low); }
  .node.locked:not(.next):not(.sel) :is(.name, .line) { color: var(--grey-dim); }
</style>
