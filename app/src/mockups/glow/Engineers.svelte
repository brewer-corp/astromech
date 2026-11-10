<script lang="ts">
  // Glow-up mockup: Engineering > Engineers as the unlock tree (List with a detail panel, the list
  // drawn as a graph). Columns by depth, each engineer joined to the one who introduces them.
  // Unlocked: lit name and grade pips (the journal gives the rank). Known or invited: base orange
  // with the one next step in words, as the unlock tree states it; the app has no counted progress
  // toward a step. Not introduced: dim. The links are the game's own introductions.
  import Frame from "./Frame.svelte";
  import Copy from "./Copy.svelte";

  type Eng = { name: string; state: "unlocked" | "known" | "hidden"; grade?: number; at?: string; step?: string; kids?: Eng[] };
  // The Bubble's ship engineers and their introductions, as data/catalog/engineer-unlocks.json
  // has them; Colonia's four are left out, and the summary line says so. Introductions come once the rank with
  // the introducer passes grade 3, so every introducer here with a known engineer is at grade 4+.
  const roots: Eng[] = [
    { name: "Felicity Farseer", state: "unlocked", grade: 5, at: "Deciat", kids: [
      { name: "Juri Ishmaak", state: "unlocked", grade: 4, at: "Giryak", kids: [
        { name: "Colonel Bris Dekker", state: "known", step: "Unlock: 1,000,000 cr of combat bonds" },
        { name: "The Sarge", state: "known", step: "Invitation: Federal Navy Midshipman" },
      ] },
    ] },
    { name: "Elvira Martuuk", state: "unlocked", grade: 5, at: "Khun", kids: [
      { name: "Marco Qwent", state: "known", step: "Unlock: 25 Modular terminals", kids: [
        { name: "Professor Palin", state: "hidden" },
        { name: "Chloe Sedesi", state: "hidden" },
        { name: "Lori Jameson", state: "hidden" },
      ] },
      { name: "Zacariah Nemo", state: "known", step: "Unlock: 25 Xihe biomorphic companions" },
    ] },
    { name: "The Dweller", state: "known", step: "Invitation: deal at 5 black markets", kids: [
      { name: "Lei Cheung", state: "hidden", kids: [{ name: "Ram Tah", state: "hidden" }] },
    ] },
    { name: "Liz Ryder", state: "unlocked", grade: 4, at: "Eurybia", kids: [
      { name: "Hera Tani", state: "known", step: "Unlock: 50 Kamitra cigars", kids: [
        { name: "Broo Tarquin", state: "hidden" },
        { name: "Tiana Fortune", state: "hidden" },
      ] },
    ] },
    { name: "Tod McQuinn", state: "unlocked", grade: 5, at: "Wolf 397", kids: [
      { name: "Selene Jean", state: "unlocked", grade: 5, at: "Kuk", kids: [
        { name: "Didi Vatermann", state: "unlocked", grade: 2, at: "Leesti" },
        { name: "Bill Turner", state: "known", step: "Unlock: 50 Bromellite" },
      ] },
    ] },
  ];
  const selected = "The Dweller";

  // Lay the tree out: leaves take the next row; a parent sits level with the middle of its
  // children. Columns are depth.
  const W = 360, H = 56, GX = 96, PITCH = 64;
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
        <span class="label">Bubble ship engineers · {counts.unlocked} unlocked · {counts.known} known · {counts.hidden} not introduced yet</span>
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
              <span class="sub"><span class="label">{n.step}</span></span>
            {:else}
              <span class="sub"><span class="label">Not introduced</span></span>
            {/if}
          </div>
        {/each}
      </div>
    </section>

    <aside>
      <h2 class="title">The Dweller</h2>
      <p class="where"><span class="label">Black Hide in</span><span class="sys">Wyrd</span><Copy /></p>
      <p class="label">84 ly · 5 jumps</p>
      <p class="key">Invitation next</p>
      <h3 class="section">To unlock</h3>
      <ol class="steps">
        <li><span class="n">1</span><span>Invitation: deal in stolen or illicit goods at 5 black markets</span></li>
        <li><span class="n">2</span><span>Unlock at Wyrd: pay 500,000 cr</span></li>
      </ol>
      <h3 class="section">Works on</h3>
      <dl class="mods">
        <dt>Power distributor</dt><dd><span class="pips">{#each [1, 2, 3, 4, 5] as g (g)}<span class="on"></span>{/each}</span></dd>
        <dt>Pulse laser</dt><dd><span class="pips">{#each [1, 2, 3, 4, 5] as g (g)}<span class:on={g <= 4}></span>{/each}</span></dd>
        <dt>Burst laser</dt><dd><span class="pips">{#each [1, 2, 3, 4, 5] as g (g)}<span class:on={g <= 3}></span>{/each}</span></dd>
        <dt>Beam laser</dt><dd><span class="pips">{#each [1, 2, 3, 4, 5] as g (g)}<span class:on={g <= 3}></span>{/each}</span></dd>
      </dl>
      <p class="label intro">Introduces Lei Cheung and Marsha Hicks</p>
      <div class="acts"><button class="primary">Find black markets</button></div>
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

  .node { position: absolute; width: var(--w); height: var(--h); padding: var(--sp-2) var(--sp-3) 0; background: var(--surface); display: flex; flex-direction: column; gap: 4px; white-space: nowrap; }
  .nm { font-weight: 500; line-height: 1.2; color: var(--accent); }
  .sub { display: flex; align-items: center; gap: var(--sp-3); min-width: 0; }
  .sub .label { overflow: hidden; text-overflow: ellipsis; }
  .node.unlocked .nm { color: var(--orange-lit); }
  .node.hidden { background: var(--bg); outline: 1px solid var(--line); outline-offset: -1px; }
  .node.hidden .nm, .node.hidden .label { color: var(--orange-dim); }
  .node.sel { background: var(--accent); }
  .node.sel * { color: var(--on-accent) !important; }

  aside { border-left: 1px solid var(--accent); padding-left: var(--sp-6); }
  .where { display: flex; align-items: baseline; gap: var(--sp-2); margin-top: var(--sp-1); white-space: nowrap; }
  .where :global(.copy) { align-self: center; margin-right: var(--sp-1); }
  .sys { color: var(--accent); }
  .key { margin: var(--sp-6) 0 var(--sp-8); font-size: var(--fs-xl); font-weight: 500; line-height: 1.2; color: var(--text-hi); }
  .section { margin-bottom: var(--sp-3); }
  .steps { display: grid; gap: var(--sp-2); margin-bottom: var(--sp-8); }
  .steps li { display: flex; gap: var(--sp-3); color: var(--accent); }
  .steps .n { color: var(--orange-mid); width: 10px; }
  .mods { display: grid; grid-template-columns: 1fr auto; gap: var(--sp-2) var(--sp-3); align-items: center; margin-bottom: var(--sp-4); color: var(--accent); }
  .intro { margin-bottom: var(--sp-6); }
  .acts { display: flex; gap: var(--sp-4); white-space: nowrap; }
</style>
