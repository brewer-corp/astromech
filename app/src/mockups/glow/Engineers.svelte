<script lang="ts">
  // Glow-up mockup: Engineering > Engineers as the unlock tree (List with a detail panel, the list
  // drawn as a graph). Columns by depth, each engineer joined to the one who introduces them.
  // Unlocked: lit name and grade pips. Known, not unlocked: base orange with the one next step
  // and a thin meter. Not introduced: dim. The links are the game's own introductions.
  import Frame from "./Frame.svelte";
  import Copy from "./Copy.svelte";

  type Eng = { name: string; state: "unlocked" | "known" | "hidden"; grade?: number; at?: string; step?: string; have?: number; need?: number; count?: string; kids?: Eng[] };
  const roots: Eng[] = [
    { name: "Felicity Farseer", state: "unlocked", grade: 5, at: "Deciat", kids: [
      { name: "Juri Ishmaak", state: "unlocked", grade: 2, at: "Giryak", kids: [
        { name: "Colonel Bris Dekker", state: "known", step: "Hand in 1M cr of combat bonds", have: 420, need: 1000, count: "420k of 1M" },
        { name: "The Sarge", state: "known", step: "Bring 50 Aberrant shield pattern analysis", have: 18, need: 50 },
      ] },
    ] },
    { name: "Elvira Martuuk", state: "unlocked", grade: 5, at: "Khun", kids: [
      { name: "Zacariah Nemo", state: "known", step: "Bring 25 Xihe biomorphic companions", have: 0, need: 25 },
    ] },
    { name: "The Dweller", state: "known", step: "Sell at 5 black markets", have: 2, need: 5, kids: [
      { name: "Lei Cheung", state: "hidden", kids: [{ name: "Ram Tah", state: "hidden" }] },
    ] },
    { name: "Liz Ryder", state: "unlocked", grade: 3, at: "Eurybia", kids: [
      { name: "Hera Tani", state: "known", step: "Bring 50 Kamitra cigars", have: 12, need: 50, kids: [
        { name: "Tiana Fortune", state: "hidden" },
        { name: "Broo Tarquin", state: "hidden" },
      ] },
    ] },
    { name: "Tod McQuinn", state: "unlocked", grade: 5, at: "Wolf 397", kids: [
      { name: "Selene Jean", state: "unlocked", grade: 5, at: "Kuk", kids: [
        { name: "Didi Vatermann", state: "unlocked", grade: 2, at: "Leesti" },
        { name: "Bill Turner", state: "known", step: "Bring 50 Bromellite", have: 31, need: 50 },
      ] },
    ] },
    { name: "Marco Qwent", state: "known", step: "Bring 25 Modular terminals", have: 9, need: 25, kids: [
      { name: "Professor Palin", state: "hidden" },
      { name: "Lori Jameson", state: "hidden" },
    ] },
  ];
  const selected = "The Dweller";

  // Lay the tree out: leaves take the next row; a parent sits level with the middle of its
  // children. Columns are depth.
  const W = 340, H = 60, GX = 96, PITCH = 70;
  type Node = Eng & { x: number; y: number };
  const nodes: Node[] = [];
  const links: { from: Node; to: Node }[] = [];
  let row = 0;
  function place(e: Eng, depth: number): Node {
    const kids = (e.kids ?? []).map((k) => place(k, depth + 1));
    const y = kids.length ? (kids[0]!.y + kids[kids.length - 1]!.y) / 2 : row++ * PITCH;
    const n = { ...e, x: depth * (W + GX), y };
    nodes.push(n);
    for (const k of kids) links.push({ from: n, to: k });
    return n;
  }
  roots.forEach((r) => place(r, 0));
  const height = row * PITCH - (PITCH - H);
  const path = (a: Node, b: Node) => {
    const x1 = a.x + W, y1 = a.y + H / 2 + 0.5, x2 = b.x, y2 = b.y + H / 2 + 0.5, mx = x1 + GX / 2 + 0.5;
    return `M${x1} ${y1}H${mx}V${y2}H${x2}`;
  };
  const counts = { unlocked: nodes.filter((n) => n.state === "unlocked").length, known: nodes.filter((n) => n.state === "known").length, hidden: nodes.filter((n) => n.state === "hidden").length };
</script>

<Frame top="Engineering" tabs={["Ships", "Stored modules", "Materials", "On foot", "Engineers"]} tab="Engineers"
  strip={{ next: "Wregoe OI-C b54-6", of: "4 of 10", flags: ["Scoopable"], dest: "Synuefe GW-Z a54-3", left: "6 jumps left" }}>
  <div class="page">
    <section class="main">
      <div class="bar">
        <span class="choices"><button class="choice on">Ship engineers</button><button class="choice">On-foot engineers</button></span>
        <span class="label">{counts.unlocked} unlocked · {counts.known} known · {counts.hidden} not introduced yet</span>
      </div>
      <div class="graph" style="width: {3 * W + 2 * GX}px; height: {height}px; --w: {W}px; --h: {H}px">
        <svg width={3 * W + 2 * GX} height={height} aria-hidden="true">
          {#each links as l (l.to.name)}<path d={path(l.from, l.to)} class:lit={l.to.state !== "hidden"} />{/each}
        </svg>
        {#each nodes as n (n.name)}
          <div class="node {n.state}" class:sel={n.name === selected} style="left: {n.x}px; top: {n.y}px">
            <span class="nm">{n.name}</span>
            {#if n.state === "unlocked"}
              <span class="sub"><span class="pips">{#each [1, 2, 3, 4, 5] as g (g)}<span class:on={g <= (n.grade ?? 0)}></span>{/each}</span><span class="label">{n.at}</span></span>
            {:else if n.state === "known"}
              <span class="sub step"><span class="label">{n.step}</span><span class="count">{n.count ?? `${n.have} of ${n.need}`}</span></span>
              {#if n.name !== selected}<span class="meter"><span style="width: {(100 * (n.have ?? 0)) / (n.need ?? 1)}%"></span></span>{/if}
            {:else}
              <span class="sub"><span class="label">Not introduced</span></span>
            {/if}
          </div>
        {/each}
      </div>
    </section>

    <aside>
      <h2 class="title">The Dweller</h2>
      <p class="where"><span class="label">Black Hide in</span><span class="sys">Wyrd</span><Copy /><span class="label">84 ly · 5 jumps</span></p>
      <p class="key"><span class="vxl white">2</span><span class="unit">of 5 black markets</span></p>
      <h3 class="section">To unlock</h3>
      <ol class="steps">
        <li class="done"><span class="n">1</span><span>Learn about him: open to all</span></li>
        <li><span class="n">2</span><span>Sell at 5 black markets <span class="mid">· 2 of 5</span></span></li>
        <li><span class="n">3</span><span>Pay 500,000 cr <span class="mid">· you have 41.2M</span></span></li>
      </ol>
      <h3 class="section">Works on</h3>
      <dl class="mods">
        <dt>Power distributor</dt><dd><span class="pips">{#each [1, 2, 3, 4, 5] as g (g)}<span class="on"></span>{/each}</span></dd>
        <dt>Pulse laser</dt><dd><span class="pips">{#each [1, 2, 3, 4, 5] as g (g)}<span class:on={g <= 4}></span>{/each}</span></dd>
        <dt>Burst laser</dt><dd><span class="pips">{#each [1, 2, 3, 4, 5] as g (g)}<span class:on={g <= 3}></span>{/each}</span></dd>
        <dt>Beam laser</dt><dd><span class="pips">{#each [1, 2, 3, 4, 5] as g (g)}<span class:on={g <= 3}></span>{/each}</span></dd>
      </dl>
      <p class="label intro">Introduces Lei Cheung and Marsha Hicks</p>
      <div class="acts"><button class="primary">Find black markets</button><button class="act">Plot to Wyrd</button></div>
    </aside>
  </div>
</Frame>

<style>
  .page { display: grid; grid-template-columns: minmax(0, 1fr) var(--panel); gap: var(--sp-12); max-width: 1760px; }
  .bar { display: flex; align-items: center; gap: var(--sp-6); margin-bottom: var(--sp-6); }
  .graph { position: relative; }
  svg { position: absolute; inset: 0; }
  path { fill: none; stroke: var(--line); stroke-width: 1; }
  path.lit { stroke: var(--orange-dim); }

  .node { position: absolute; width: var(--w); height: var(--h); padding: 9px var(--sp-3) 0; background: var(--surface); display: flex; flex-direction: column; gap: 4px; white-space: nowrap; }
  .nm { font-weight: 500; line-height: 1.2; color: var(--accent); }
  .sub { display: flex; align-items: center; gap: var(--sp-3); min-width: 0; }
  .sub .label { overflow: hidden; text-overflow: ellipsis; }
  .step { justify-content: space-between; }
  .count { font-size: var(--fs-s); color: var(--accent); flex: none; }
  .node .meter { position: absolute; left: 0; right: 0; bottom: 0; height: 2px; }
  .node .meter > span { background: var(--orange-mid); }
  .node.unlocked .nm { color: var(--orange-lit); }
  .node.hidden { background: var(--bg); outline: 1px solid var(--line); outline-offset: -1px; }
  .node.hidden .nm, .node.hidden .label { color: var(--orange-dim); }
  .node.sel { background: var(--accent); }
  .node.sel * { color: var(--on-accent) !important; }

  aside { border-left: 1px solid var(--accent); padding-left: var(--sp-6); }
  .where { display: flex; align-items: baseline; gap: var(--sp-2); margin-top: var(--sp-1); white-space: nowrap; }
  .where :global(.copy) { align-self: center; margin-right: var(--sp-1); }
  .sys { color: var(--accent); }
  .key { display: flex; align-items: baseline; margin: var(--sp-6) 0 var(--sp-8); }
  .section { margin-bottom: var(--sp-3); }
  .steps { display: grid; gap: var(--sp-2); margin-bottom: var(--sp-8); }
  .steps li { display: flex; gap: var(--sp-3); color: var(--accent); }
  .steps .n { color: var(--orange-mid); width: 10px; }
  .steps li.done { color: var(--orange-mid); }
  .mods { display: grid; grid-template-columns: 1fr auto; gap: var(--sp-2) var(--sp-3); align-items: center; margin-bottom: var(--sp-4); color: var(--accent); }
  .intro { margin-bottom: var(--sp-6); }
  .acts { display: flex; gap: var(--sp-4); }
</style>
