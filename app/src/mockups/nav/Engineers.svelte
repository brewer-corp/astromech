<script lang="ts">
  // ENGINEERING > Engineers: ship and suit engineers in one unlock tree (the Glow-up tree, with
  // the suit engineers beside the ship ones instead of behind a choice), and a search by module
  // type at the top. Typing "laser" swaps the tree for the answer: the nearest unlocked engineer
  // who reaches the highest grade, raised, then every engineer for those modules with their grade
  // per module, the commander's rank, system and distance. Grades and systems are the app's
  // catalog's (data/catalog/engineering.json, engineers.json); distances are from Sol.
  import Frame from "./Frame.svelte";
  import Glyph from "../../lib/ui/Glyph.svelte";

  let { query = "" }: { query?: string } = $props();

  type Eng = { name: string; state: "unlocked" | "known" | "hidden"; rank?: number; at?: string; step?: string; kids?: Eng[]; also?: string[] };
  const ship: Eng[] = [
    { name: "Tod McQuinn", state: "unlocked", rank: 5, at: "Wolf 397", kids: [
      { name: "Selene Jean", state: "unlocked", rank: 5, at: "Kuk", kids: [
        { name: "Didi Vatermann", state: "unlocked", rank: 2, at: "Leesti" },
        { name: "Bill Turner", state: "known", step: "Unlock: 50 Bromellite" },
      ] },
      { name: "Petra Olmanova", state: "known", step: "Invite: Expert combat rank" },
    ] },
    { name: "Felicity Farseer", state: "unlocked", rank: 5, at: "Deciat", kids: [
      { name: "Juri Ishmaak", state: "unlocked", rank: 4, at: "Giryak", kids: [
        { name: "Colonel Bris Dekker", state: "known", step: "Invite: Friendly, Federation" },
        { name: "The Sarge", state: "known", step: "Invite: Midshipman" },
      ] },
    ] },
    { name: "Elvira Martuuk", state: "unlocked", rank: 5, at: "Khun", kids: [
      { name: "Marco Qwent", state: "known", step: "Unlock: 25 Modular terminals", kids: [
        { name: "Professor Palin", state: "hidden" },
        { name: "Chloe Sedesi", state: "hidden" },
        { name: "Lori Jameson", state: "hidden" },
      ] },
      { name: "Zacariah Nemo", state: "known", step: "Unlock: 25 Xihe companions" },
      { name: "Mel Brandon", state: "known", step: "Invite: Friendly, Colonia" },
    ] },
    { name: "Liz Ryder", state: "unlocked", rank: 4, at: "Eurybia", kids: [
      { name: "Hera Tani", state: "unlocked", rank: 5, at: "Kuwemaki", kids: [
        { name: "Broo Tarquin", state: "unlocked", rank: 3, at: "Muang" },
        { name: "Tiana Fortune", state: "known", step: "Invite: Friendly, Empire" },
      ] },
      { name: "Etienne Dorn", state: "known", step: "Invite: Dealer trade rank" },
    ] },
    { name: "The Dweller", state: "unlocked", rank: 5, at: "Wyrd", kids: [
      { name: "Lei Cheung", state: "known", step: "Invite: trade at 50 markets", kids: [{ name: "Ram Tah", state: "hidden" }] },
      { name: "Marsha Hicks", state: "known", step: "Invite: Surveyor explorer rank" },
    ] },
  ];
  const suit: Eng[] = [
    { name: "Jude Navarro", state: "unlocked", at: "Aurai", kids: [
      { name: "Terra Velasquez", state: "unlocked", at: "Shou Xing", kids: [{ name: "Oden Geiger", state: "unlocked", at: "Candiaei" }] },
    ] },
    { name: "Domino Green", state: "unlocked", at: "Orishis", kids: [
      { name: "Kit Fowler", state: "known", step: "Invite: sell 5 Opinion Polls", kids: [{ name: "Yarden Bond", state: "hidden" }] },
    ] },
    { name: "Hero Ferrari", state: "known", step: "Invite: 10 conflict zones", kids: [
      { name: "Wellington Beck", state: "hidden", kids: [{ name: "Uma Laszlo", state: "hidden" }] },
    ] },
    { name: "Baltanos", state: "known", step: "Invite: Friendly, Colonia" },
    { name: "Eleanor Bresa", state: "known", step: "Invite: visit 5 settlements" },
    { name: "Rosa Dayette", state: "known", step: "Invite: sell 10 recipes", kids: [{ name: "Yi Shen", state: "hidden", also: ["Baltanos", "Eleanor Bresa"] }] },
  ];

  // Leaves take the next row; a parent sits level with the middle of its children; columns are depth.
  const W = 200, H = 50, GX = 28, PITCH = 56;
  type Node = Eng & { x: number; y: number };
  function layout(roots: Eng[]) {
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
    for (const n of nodes) for (const a of n.also ?? []) { const f = nodes.find((m) => m.name === a); if (f) links.push({ from: f, to: n }); }
    return { nodes, links, height: row * PITCH - (PITCH - H), width: 3 * W + 2 * GX };
  }
  const trees = [{ title: "Ship engineers", ...layout(ship) }, { title: "Suit engineers", ...layout(suit) }];
  const path = (a: Node, b: Node) => {
    const x1 = a.x + W, y1 = a.y + H / 2 + 0.5, x2 = b.x, y2 = b.y + H / 2 + 0.5, mx = x1 + GX / 2 + 0.5;
    return `M${x1} ${y1}H${mx}V${y2}H${x2}`;
  };
  const count = (t: Eng[], s: string): number => t.reduce((c, e) => c + (e.state === s ? 1 : 0) + count(e.kids ?? [], s), 0);

  // The "laser" answer: pulse, burst and beam lasers. Grade = the engineer's best blueprint grade
  // on that module; rank from the journal; the main answer is the nearest unlocked engineer whose
  // grade is the highest any engineer offers.
  const hits = [
    { name: "Broo Tarquin", grades: [5, 5, 5], state: "unlocked", rank: 3, at: "Muang", ly: "174" },
    { name: "The Dweller", grades: [4, 3, 3], state: "unlocked", rank: 5, at: "Wyrd", ly: "34" },
    { name: "Mel Brandon", grades: [5, 5, 5], state: "known", rank: 0, at: "Luchtaine", ly: "22,014" },
  ];
  const kinds = ["Pulse", "Burst", "Beam"];
</script>

{#snippet pips(g: number)}<span class="pips">{#each [1, 2, 3, 4, 5] as i (i)}<span class:on={i <= g}></span>{/each}</span>{/snippet}

<Frame top="Engineering" tabs={["Materials", "Engineers"]} tab="Engineers" route="open">
  <div class="page">
    <div class="search">
      <span class="cell input" class:filled={!!query} aria-label="Engineers for a module">
        {#if query}<span>{query}</span><span class="clear"><Glyph name="clear" label="Clear" /></span>
        {:else}<span class="ph">Engineers for a module: laser, FSD, shields…</span>{/if}
      </span>
      {#if query}
        <span class="label">Pulse, burst and beam lasers · 3 engineers</span>
      {:else}
        <span class="label">{count(ship, "unlocked")} of 25 ship and {count(suit, "unlocked")} of 13 suit engineers unlocked</span>
      {/if}
    </div>

    {#if !query}
      <div class="trees">
        {#each trees as t (t.title)}
          <section>
            <h2 class="section">{t.title}</h2>
            <div class="graph" style="width: {t.width}px; height: {t.height}px; --w: {W}px; --h: {H}px">
              <svg width={t.width} height={t.height} aria-hidden="true">
                {#each t.links as l, i (i)}<path d={path(l.from, l.to)} class:lit={l.to.state !== "hidden"} />{/each}
              </svg>
              {#each t.nodes as n (n.name)}
                <div class="node {n.state}" style="left: {n.x}px; top: {n.y}px">
                  <span class="nm">{n.name}</span>
                  {#if n.state === "unlocked"}
                    <span class="sub">{#if n.rank}{@render pips(n.rank)}{/if}<span class="label">{n.at}</span></span>
                  {:else if n.state === "known"}
                    <span class="sub"><span class="label">{n.step}</span></span>
                  {:else}
                    <span class="sub"><span class="label">Not introduced</span></span>
                  {/if}
                </div>
              {/each}
            </div>
          </section>
        {/each}
      </div>
    {:else}
      {@const best = hits[0]!}
      <div class="answer">
      <div class="best">
        <span class="label">Nearest to reach grade 5</span>
        <div class="bline">
          <span class="bname">{best.name}</span>
          {@render pips(best.rank)}<span class="label">Rank {best.rank}</span>
        </div>
        <div class="bline">
          <span class="sys">{best.at}</span><span class="ibtn" aria-label="Copy {best.at}"><Glyph name="copy" /></span>
          <span class="dist"><span class="vl">{best.ly}</span><span class="unit">ly</span></span>
          <span class="grow"></span>
          <span class="primary">Set destination</span>
          <span class="ibtn" aria-label="Add {best.at} as a stop"><Glyph name="add-stop" /></span>
        </div>
      </div>

      <div class="list">
        <div class="row heads label">
          <span class="r">Best</span><span>Engineer</span>{#each kinds as k (k)}<span class="r">{k}</span>{/each}<span>Your rank</span><span>System</span><span class="r">Distance</span><span></span>
        </div>
        {#each hits as h (h.name)}
          <div class="row" class:far={h.state !== "unlocked"}>
            <span class="r"><span class="vl">{Math.max(...h.grades)}</span></span>
            <span class="ename">{h.name}</span>
            {#each h.grades as g, i (i)}<span class="r">{g}</span>{/each}
            <span class="rank">{#if h.state === "unlocked"}{@render pips(h.rank)}{:else}<span class="label">Known</span>{/if}</span>
            <span class="place"><span>{h.at}</span><span class="ibtn" aria-label="Copy {h.at}"><Glyph name="copy" /></span></span>
            <span class="r">{h.ly}<span class="unit">ly</span></span>
            <span class="acts"><span class="ibtn" aria-label="Add {h.at} as a stop"><Glyph name="add-stop" /></span><span class="ibtn" aria-label="Set {h.at} as the destination"><Glyph name="destination" /></span></span>
          </div>
        {/each}
      </div>
      </div>
    {/if}
  </div>
</Frame>

<style>
  .search { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-6); margin-bottom: var(--sp-8); }
  .input { width: 440px; box-sizing: border-box; justify-content: space-between; }
  .input:not(.filled) { outline: none; }
  .input.filled { outline: 1px solid var(--accent); outline-offset: 0; }
  .clear { display: grid; color: var(--accent); }

  .trees { display: flex; flex-wrap: wrap; gap: var(--sp-8) var(--sp-12); align-items: flex-start; }
  .section { margin-bottom: var(--sp-4); }
  .graph { position: relative; }
  svg { position: absolute; inset: 0; }
  path { fill: none; stroke: var(--line); stroke-width: 1; }
  path.lit { stroke: var(--orange-dim); }
  .node { position: absolute; width: var(--w); height: var(--h); box-sizing: border-box; padding: 6px var(--sp-3) 0; background: var(--surface); display: flex; flex-direction: column; gap: 2px; white-space: nowrap; }
  .nm { font-weight: 500; line-height: 1.2; color: var(--accent); overflow: hidden; text-overflow: ellipsis; }
  .sub { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; }
  .sub .label { overflow: hidden; text-overflow: ellipsis; }
  .node.unlocked .nm { color: var(--orange-lit); }
  .node.hidden { background: var(--bg); outline: 1px solid var(--line); outline-offset: -1px; }
  .node.hidden .nm, .node.hidden .label { color: var(--orange-dim); }

  .answer { width: max-content; max-width: 100%; }
  .best { display: flex; flex-direction: column; gap: var(--sp-2); box-sizing: border-box; padding: var(--sp-4) var(--sp-4) var(--sp-4) var(--sp-6); background: var(--surface); margin-bottom: var(--sp-8); }
  .bline { display: flex; align-items: center; gap: var(--sp-3); }
  .bname { font-size: var(--fs-xl); font-weight: 600; line-height: 1.2; color: var(--text-hi); margin-right: var(--sp-3); }
  .sys { color: var(--accent); }
  .bline .ibtn { margin-left: -8px; }
  .bline .primary + .ibtn { margin-left: 0; }
  .dist { margin-left: var(--sp-4); display: flex; align-items: baseline; }
  .grow { flex: 1; }

  .list { max-width: 100%; }
  .row { display: grid; grid-template-columns: 56px minmax(150px, 200px) 52px 52px 52px 112px minmax(110px, 150px) 88px 64px; align-items: center; column-gap: var(--sp-3); height: var(--row-result); padding: 0 var(--sp-3); white-space: nowrap; color: var(--accent); }
  .row.heads { height: 28px; border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); color: var(--orange-mid); }
  .row.far .vl { color: var(--orange-mid); }
  .ename { overflow: hidden; text-overflow: ellipsis; }
  .place { display: flex; align-items: center; gap: var(--sp-1); min-width: 0; }
  .acts { display: flex; justify-content: flex-end; gap: var(--sp-1); }
  @container (max-width: 1000px) {
    .trees { flex-direction: column; }
  }
</style>
